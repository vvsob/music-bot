use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink};
use std::{collections::VecDeque, fs::File, io::BufReader, thread, thread::JoinHandle};

use crate::channel::{self, Requester, Responder, TryRecvError};
use crate::track::{Track, TrackInfo};

#[derive(Debug)]
enum WorkerRequest {
    AddTrack(Track),
    Pause,
    Play,
    Stop,
    SkipOne,
    ListTracks,
}

#[derive(Debug)]
enum WorkerResponse {
    TrackList(VecDeque<TrackInfo>),
    None,
}

use WorkerRequest::*;
use WorkerResponse::*;

pub struct MusicPlayer {
    requester: Requester<WorkerRequest, WorkerResponse>,
    worker: Option<JoinHandle<()>>,
}

struct Worker {
    _stream: OutputStream,
    _handle: OutputStreamHandle,
    sink: Sink,
    queue: VecDeque<Track>,
    current: Option<Track>,
    responder: Responder<WorkerRequest, WorkerResponse>,
}

fn get_source(track: &Track) -> Decoder<BufReader<File>> {
    let file = BufReader::new(File::open(track.file.get_path()).unwrap());
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
                self.current.take();
                None
            }
            SkipOne => {
                self.sink.skip_one();
                self.current.take();
                None
            }
            ListTracks => TrackList(self.queue.iter().map(|track| track.info.clone()).collect::<VecDeque<TrackInfo>>().clone()),
        }
    }

    pub fn main(&mut self) {
        loop {
            match self.responder.try_recv() {
                Ok(mut req) => {
                    let data = req.data.take().unwrap();
                    let _ = req.respond(self.match_request(data));
                }
                Err(TryRecvError::Empty) => {}
                Err(TryRecvError::Disconnected) => break,
            };
            if self.sink.empty() && !self.queue.is_empty() {
                let next_track = self.queue.pop_front().unwrap();
                self.sink.append(get_source(&next_track));
                self.current = Some(next_track);
            } else if self.sink.empty() {
                self.current.take();
            }
        }
    }

    pub fn build(responder: Responder<WorkerRequest, WorkerResponse>) -> Worker {
        let (_stream, _handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&_handle).unwrap();
        let queue: VecDeque<Track> = VecDeque::new();
        Worker {
            _stream,
            _handle,
            sink,
            queue,
            current: Option::None,
            responder,
        }
    }
}

impl MusicPlayer {
    pub fn build() -> MusicPlayer {
        let (requester, responder) = channel::channel();
        let worker = thread::spawn(move || Worker::build(responder).main());
        MusicPlayer { requester, worker: Some(worker) }
    }

    pub fn enqueue(&mut self, track: Track) {
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

impl Drop for MusicPlayer {
    fn drop(&mut self) {
        self.stop();
        self.worker.take().unwrap().join().unwrap();
    }
}
