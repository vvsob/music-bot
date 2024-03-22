use music_bot::{MusicPlayer, TrackInfo};
use std::{path::Path, thread, time::Duration};
use rustube::{blocking::Video, url::Url};

fn main() {
    // let url = "https://www.youtube.com/watch?v=UnIhRpIT7nc";
    // let url = Url::parse(url).unwrap();
    // let video = Video::from_url(&url).unwrap();

    // let stream = video.best_audio().unwrap();
    // let path = stream.blocking_download().unwrap();

    // let path = path.to_str().unwrap();

    // println!("{path}")

    let mut player = MusicPlayer::new();
    player.enqueue(TrackInfo::new(&Path::new("music.mp3")));
    player.enqueue(TrackInfo::new(&Path::new("music.mp3")));
    // MusicPlayer::play(TrackInfo::new(&Path::new("music.mp3")));
    for track in player.list_tracks() {
        println!("{}", track.path.into_os_string().into_string().unwrap())
    }
    thread::sleep(Duration::from_secs(5));
    player.skip_one();
    thread::sleep(Duration::from_secs(4 * 60 + 13))
}
