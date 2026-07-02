{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    crane.url = "github:ipetkov/crane";
  };

  outputs = { self, nixpkgs, flake-utils, crane, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        craneLib = crane.mkLib pkgs;
        src = craneLib.cleanCargoSource ./.;

        my-bot = craneLib.buildPackage {
          inherit src;
          strictDeps = true;
          buildInputs = with pkgs; [
            openssl
            alsa-lib
          ];
          nativeBuildInputs = with pkgs; [ pkg-config ];
        };

        asoundConf = pkgs.writeText "asound.conf" ''
          pcm.!default {
            type pulse
          }
          ctl.!default {
            type pulse
          }
        '';

        my-bot-wrapped = pkgs.symlinkJoin {
          name = "music_bot";
          paths = [ my-bot ];
          buildInputs = [ pkgs.makeWrapper ];
          postBuild = ''
            wrapProgram $out/bin/music_bot \
              --set ALSA_CONFIG_PATH "${asoundConf}" \
              --set ALSA_PLUGIN_DIR "${pkgs.alsa-plugins}/lib/alsa-lib" \
              --prefix LD_LIBRARY_PATH : "${pkgs.lib.makeLibraryPath [ pkgs.alsa-lib ]}"
          '';
        };
      in
      {
        packages.default = my-bot-wrapped;
        apps.default = flake-utils.lib.mkApp { drv = my-bot-wrapped; };
        devShells.default = pkgs.mkShell {
          inputsFrom = [ my-bot ];
          packages = with pkgs; [ rust-analyzer clippy rustfmt alsa-plugins alsa-utils ];
        };
      }
    );
}