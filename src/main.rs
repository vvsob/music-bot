use music_bot::{download, MusicPlayer, TrackInfo};
use std::{path::Path, process::Command, thread, time::Duration};

fn main() {
    // let url = "https://www.youtube.com/watch?v=UnIhRpIT7nc";
    let url = "https://www.youtube.com/watch?v=8bB0FNGlrEs";
    let track = download::download_from_youtube(url);

    let mut player = MusicPlayer::new();
    player.enqueue(track.clone());
    player.enqueue(track.clone());
    thread::sleep(Duration::from_secs(5));
    player.skip_one();
    thread::sleep(Duration::from_secs(600));
}
