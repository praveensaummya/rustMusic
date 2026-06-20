use rodio::{Decoder, OutputStream, Sink, Source};
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

pub enum AudioCommand {
    Play(PathBuf),
    Pause,
    Resume,
    Stop,
    Seek(f64),
    SetVolume(f32),
    Quit,
}

pub enum AudioStatus {
    Playing,
    Paused,
    Stopped,
    Finished,
    Error(String),
}

pub struct AudioEngine {
    command_sender: Sender<AudioCommand>,
    status_receiver: Receiver<AudioStatus>,
    current_position: Arc<Mutex<f64>>,
    total_duration: Arc<Mutex<f64>>,
    current_song: Arc<Mutex<Option<String>>>,
}

impl AudioEngine {
    pub fn new() -> Self {
        let (cmd_tx, cmd_rx) = mpsc::channel::<AudioCommand>();
        let (status_tx, status_rx) = mpsc::channel::<AudioStatus>();
        let current_position = Arc::new(Mutex::new(0.0));
        let total_duration = Arc::new(Mutex::new(0.0));
        let current_song = Arc::new(Mutex::new(None));

        let pos = current_position.clone();
        let dur = total_duration.clone();
        let song = current_song.clone();

        thread::spawn(move || {
            let (_stream, stream_handle) = match OutputStream::try_default() {
                Ok(s) => s,
                Err(e) => {
                    let _ = status_tx.send(AudioStatus::Error(format!(
                        "Failed to open audio output: {}",
                        e
                    )));
                    return;
                }
            };

            let mut sink: Option<Sink> = None;
            let mut is_paused = false;
            let mut volume = 1.0;
            let mut current_file: Option<PathBuf> = None;

            loop {
                // Update position if playing
                if let Some(ref s) = sink {
                    if !s.empty() && !is_paused {
                        if let Ok(mut p) = pos.lock() {
                            *p = s.get_pos().as_secs_f64();
                        }
                    }
                    if s.empty() && !is_paused && sink.is_some() {
                        let _ = status_tx.send(AudioStatus::Finished);
                        if let Ok(mut p) = pos.lock() {
                            *p = 0.0;
                        }
                        sink = None;
                        current_file = None;
                        if let Ok(mut s) = song.lock() {
                            *s = None;
                        }
                    }
                }

                // Check for commands
                match cmd_rx.try_recv() {
                    Ok(AudioCommand::Play(p)) => {
                        if let Some(s) = sink.take() {
                            s.stop();
                        }
                        is_paused = false;
                        current_file = Some(p.clone());

                        match File::open(&p) {
                            Ok(file) => {
                                let reader = BufReader::new(file);
                                match Decoder::new(reader) {
                                    Ok(source) => {
                                        let total = source.total_duration();
                                        if let Ok(mut d) = dur.lock() {
                                            *d = total.map(|t| t.as_secs_f64()).unwrap_or(0.0);
                                        }
                                        if let Ok(mut p) = pos.lock() {
                                            *p = 0.0;
                                        }
                                        if let Ok(mut s) = song.lock() {
                                            *s = p
                                                .file_name()
                                                .map(|n| n.to_string_lossy().to_string());
                                        }

                                        let new_sink = Sink::try_new(&stream_handle).unwrap();
                                        new_sink.set_volume(volume);
                                        new_sink.append(source);
                                        sink = Some(new_sink);
                                        let _ = status_tx.send(AudioStatus::Playing);
                                    }
                                    Err(e) => {
                                        let _ = status_tx.send(AudioStatus::Error(format!(
                                            "Failed to decode audio: {}",
                                            e
                                        )));
                                    }
                                }
                            }
                            Err(e) => {
                                let _ = status_tx.send(AudioStatus::Error(format!(
                                    "Failed to open file: {}",
                                    e
                                )));
                            }
                        }
                    }
                    Ok(AudioCommand::Pause) => {
                        if let Some(ref s) = sink {
                            s.pause();
                            is_paused = true;
                            let _ = status_tx.send(AudioStatus::Paused);
                        }
                    }
                    Ok(AudioCommand::Resume) => {
                        if let Some(ref s) = sink {
                            s.play();
                            is_paused = false;
                            let _ = status_tx.send(AudioStatus::Playing);
                        }
                    }
                    Ok(AudioCommand::Stop) => {
                        if let Some(s) = sink.take() {
                            s.stop();
                        }
                        is_paused = false;
                        current_file = None;
                        if let Ok(mut p) = pos.lock() {
                            *p = 0.0;
                        }
                        if let Ok(mut s) = song.lock() {
                            *s = None;
                        }
                        let _ = status_tx.send(AudioStatus::Stopped);
                    }
                    Ok(AudioCommand::Seek(target_pos)) => {
                        // Seek by restarting playback at the target position
                        // Do NOT reset position to 0.0 - set it directly to target
                        if let Some(ref file_path) = current_file {
                            if let Some(ref s) = sink {
                                s.stop();
                            }
                            sink = None;

                            match File::open(file_path) {
                                Ok(file) => {
                                    let reader = BufReader::new(file);
                                    match Decoder::new(reader) {
                                        Ok(source) => {
                                            let total = source.total_duration();
                                            let total_secs =
                                                total.map(|t| t.as_secs_f64()).unwrap_or(0.0);
                                            if let Ok(mut d) = dur.lock() {
                                                *d = total_secs;
                                            }

                                            let seek_pos = target_pos.min(total_secs).max(0.0);
                                            let skipped = source.skip_duration(Duration::from_secs_f64(seek_pos));

                                            // Set position directly to seek target (not 0)
                                            if let Ok(mut p) = pos.lock() {
                                                *p = seek_pos;
                                            }

                                            let new_sink =
                                                Sink::try_new(&stream_handle).unwrap();
                                            new_sink.set_volume(volume);
                                            new_sink.append(skipped);
                                            sink = Some(new_sink);

                                            if !is_paused {
                                                let _ = status_tx.send(AudioStatus::Playing);
                                            }
                                        }
                                        Err(e) => {
                                            let _ = status_tx.send(AudioStatus::Error(
                                                format!("Failed to seek: {}", e),
                                            ));
                                        }
                                    }
                                }
                                Err(e) => {
                                    let _ = status_tx.send(AudioStatus::Error(format!(
                                        "Failed to seek: {}",
                                        e
                                    )));
                                }
                            }
                        }
                    }
                    Ok(AudioCommand::SetVolume(v)) => {
                        volume = v;
                        if let Some(ref s) = sink {
                            s.set_volume(v);
                        }
                    }
                    Ok(AudioCommand::Quit) => {
                        if let Some(s) = sink.take() {
                            s.stop();
                        }
                        break;
                    }
                    Err(mpsc::TryRecvError::Empty) => {}
                    Err(mpsc::TryRecvError::Disconnected) => break,
                }

                thread::sleep(Duration::from_millis(50));
            }
        });

        AudioEngine {
            command_sender: cmd_tx,
            status_receiver: status_rx,
            current_position,
            total_duration,
            current_song,
        }
    }

    pub fn play(&self, path: PathBuf) {
        let _ = self.command_sender.send(AudioCommand::Play(path));
    }

    pub fn pause(&self) {
        let _ = self.command_sender.send(AudioCommand::Pause);
    }

    pub fn resume(&self) {
        let _ = self.command_sender.send(AudioCommand::Resume);
    }

    pub fn stop(&self) {
        let _ = self.command_sender.send(AudioCommand::Stop);
    }

    pub fn set_volume(&self, volume: f32) {
        let _ = self.command_sender.send(AudioCommand::SetVolume(volume));
    }

    pub fn seek(&self, position: f64) {
        let _ = self.command_sender.send(AudioCommand::Seek(position));
    }

    pub fn get_position(&self) -> f64 {
        *self.current_position.lock().unwrap()
    }

    pub fn get_duration(&self) -> f64 {
        *self.total_duration.lock().unwrap()
    }

    pub fn get_current_song(&self) -> Option<String> {
        self.current_song.lock().unwrap().clone()
    }

    pub fn check_status(&self) -> Option<AudioStatus> {
        self.status_receiver.try_recv().ok()
    }
}

impl Drop for AudioEngine {
    fn drop(&mut self) {
        let _ = self.command_sender.send(AudioCommand::Quit);
    }
}