mod track;
pub mod channel;
pub mod download;

pub use track::TrackInfo;

use channel::{Requester, Responder, TryRecvError};
use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink};
use std::{
    collections::VecDeque,
    fs::File,
    io::BufReader,
    thread,
    thread::JoinHandle,
};

#[derive(Debug, Clone)]
enum WorkerRequest {
    AddTrack(TrackInfo),
    Pause,
    Play,
    Stop,
    SkipOne,
    ListTracks,
}

#[derive(Debug, Clone)]
enum WorkerResponse {
    TrackList(VecDeque<TrackInfo>),
    None,
}

use WorkerRequest::*;
use WorkerResponse::*;

pub struct MusicPlayer {
    requester: Requester<WorkerRequest, WorkerResponse>,
    _worker: JoinHandle<()>,
}

struct Worker {
    _stream: OutputStream,
    _handle: OutputStreamHandle,
    sink: Sink,
    queue: VecDeque<TrackInfo>,
    responder: Responder<WorkerRequest, WorkerResponse>,
}

fn get_source(track: TrackInfo) -> Decoder<BufReader<File>> {
    let file = BufReader::new(File::open(&track.path).unwrap());
    Decoder::new(file).unwrap()
}

impl Worker {
    fn match_request(&mut self, req: WorkerRequest) -> WorkerResponse {
        match req {
            AddTrack(track) => {
                self.queue.push_back(track);
                None
            }
            Pause => {
                self.sink.pause();
                None
            }
            Play => {
                self.sink.play();
                None
            }
            Stop => {
                self.sink.stop();
                self.queue.clear();
                None
            }
            SkipOne => {
                self.sink.skip_one();
                None
            }
            ListTracks => TrackList(self.queue.clone()),
        }
    }

    pub fn main(&mut self) {
        loop {
            match self.responder.try_recv() {
                Ok(req) => {
                    let _ = req.respond(self.match_request(req.data.clone()));
                }
                Err(TryRecvError::Empty) => {}
                Err(TryRecvError::Disconnected) => break,
            };
            if self.sink.empty() && !self.queue.is_empty() {
                let next_track = self.queue.pop_front().unwrap();
                self.sink.append(get_source(next_track));
            }
        }
    }

    pub fn build(responder: Responder<WorkerRequest, WorkerResponse>) -> Worker {
        let (_stream, _handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&_handle).unwrap();
        let queue: VecDeque<TrackInfo> = VecDeque::new();
        Worker {
            _stream,
            _handle,
            sink,
            queue,
            responder,
        }
    }
}

impl MusicPlayer {
    pub fn new() -> MusicPlayer {
        let (requester, responder) = channel::channel();
        let _worker = thread::spawn(move || Worker::build(responder).main());
        MusicPlayer { requester, _worker }
    }

    pub fn enqueue(&mut self, track: TrackInfo) {
        self.requester.send(AddTrack(track)).unwrap();
    }

    pub fn skip_one(&mut self) {
        self.requester.send(SkipOne).unwrap();
    }

    pub fn pause(&mut self) {
        self.requester.send(Pause).unwrap();
    }

    pub fn play(&mut self) {
        self.requester.send(Play).unwrap();
    }

    pub fn stop(&mut self) {
        self.requester.send(Stop).unwrap();
    }

    pub fn list_tracks(&self) -> VecDeque<TrackInfo> {
        let response = self.requester.send(ListTracks).unwrap();
        match response.recv().unwrap() {
            TrackList(tracks) => tracks,
            r => panic!("On .list_tracks should get TrackList, not {:#?}.", r),
        }
    }
}
