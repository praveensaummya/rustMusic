use rand::Rng;
use std::path::PathBuf;
use walkdir::WalkDir;

#[derive(Debug, Clone)]
pub struct Song {
    pub path: PathBuf,
    pub title: String,
    pub duration_secs: f64,
    pub artist: Option<String>,
    pub album: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Playlist {
    pub name: String,
    pub songs: Vec<Song>,
    pub current_index: Option<usize>,
    pub shuffle: bool,
    pub repeat: bool,
}

impl Playlist {
    pub fn new(name: &str) -> Self {
        Playlist {
            name: name.to_string(),
            songs: Vec::new(),
            current_index: None,
            shuffle: false,
            repeat: false,
        }
    }

    pub fn add_song(&mut self, path: PathBuf) {
        let title = path
            .file_stem()
            .map(|s| s.to_string_lossy().to_string())
            .unwrap_or_else(|| "Unknown".to_string());

        self.songs.push(Song {
            path,
            title,
            duration_secs: 0.0,
            artist: None,
            album: None,
        });
    }

    pub fn add_songs_from_dir(&mut self, dir: &PathBuf) {
        let extensions = ["mp3", "wav", "flac", "ogg", "m4a", "aac", "wma"];

        for entry in WalkDir::new(dir).into_iter().filter_map(|e| e.ok()) {
            if entry.file_type().is_file() {
                if let Some(ext) = entry.path().extension() {
                    if extensions
                        .iter()
                        .any(|e| e == &ext.to_string_lossy().to_lowercase())
                    {
                        self.add_song(entry.path().to_path_buf());
                    }
                }
            }
        }
    }

    pub fn remove_song(&mut self, index: usize) {
        if index < self.songs.len() {
            self.songs.remove(index);
            if let Some(current) = self.current_index {
                if current >= index {
                    if current > 0 {
                        self.current_index = Some(current - 1);
                    } else {
                        self.current_index = if self.songs.is_empty() {
                            None
                        } else {
                            Some(0)
                        };
                    }
                }
            }
        }
    }

    pub fn get_current_song(&self) -> Option<&Song> {
        self.current_index.and_then(|i| self.songs.get(i))
    }

    pub fn next(&mut self) -> Option<&Song> {
        if self.songs.is_empty() {
            return None;
        }

        match self.current_index {
            Some(i) => {
                if self.shuffle {
                    let mut rng = rand::thread_rng();
                    let new_index = rng.gen_range(0..self.songs.len());
                    self.current_index = Some(new_index);
                } else if i + 1 < self.songs.len() {
                    self.current_index = Some(i + 1);
                } else if self.repeat {
                    self.current_index = Some(0);
                } else {
                    return None;
                }
            }
            None => {
                if !self.songs.is_empty() {
                    self.current_index = Some(0);
                }
            }
        }

        self.get_current_song()
    }

    pub fn previous(&mut self) -> Option<&Song> {
        if self.songs.is_empty() {
            return None;
        }

        match self.current_index {
            Some(i) => {
                if i > 0 {
                    self.current_index = Some(i - 1);
                } else if self.repeat {
                    self.current_index = Some(self.songs.len() - 1);
                } else {
                    return None;
                }
            }
            None => {
                if !self.songs.is_empty() {
                    self.current_index = Some(0);
                }
            }
        }

        self.get_current_song()
    }

    pub fn play_at(&mut self, index: usize) -> Option<&Song> {
        if index < self.songs.len() {
            self.current_index = Some(index);
            self.get_current_song()
        } else {
            None
        }
    }

    pub fn clear(&mut self) {
        self.songs.clear();
        self.current_index = None;
    }

    pub fn total_duration(&self) -> f64 {
        self.songs.iter().map(|s| s.duration_secs).sum()
    }

    pub fn is_empty(&self) -> bool {
        self.songs.is_empty()
    }

    pub fn len(&self) -> usize {
        self.songs.len()
    }
}