mod audio;
mod playlist;

use audio::{AudioEngine, AudioStatus};
use eframe::egui;
use playlist::{Playlist, Song};
use rfd::FileDialog;
use std::path::PathBuf;

struct RustMusicApp {
    audio_engine: AudioEngine,
    playlist: Playlist,
    volume: f32,
    is_playing: bool,
    is_paused: bool,
    search_query: String,
    status_message: String,
    last_folder: Option<PathBuf>,
}

impl RustMusicApp {
    fn new(_cc: &eframe::CreationContext) -> Self {
        RustMusicApp {
            audio_engine: AudioEngine::new(),
            playlist: Playlist::new("Default"),
            volume: 0.8,
            is_playing: false,
            is_paused: false,
            search_query: String::new(),
            status_message: "Welcome to RustMusic!".to_string(),
            last_folder: None,
        }
    }

    fn format_time(secs: f64) -> String {
        if secs.is_nan() || secs <= 0.0 {
            return "00:00".to_string();
        }
        let total_secs = secs as u64;
        let hours = total_secs / 3600;
        let minutes = (total_secs % 3600) / 60;
        let seconds = total_secs % 60;

        if hours > 0 {
            format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
        } else {
            format!("{:02}:{:02}", minutes, seconds)
        }
    }

    /// Truncate text to max_chars and add "..." if it was truncated
    fn truncate_text(text: &str, max_chars: usize) -> String {
        if text.chars().count() > max_chars {
            format!("{}...", text.chars().take(max_chars).collect::<String>())
        } else {
            text.to_string()
        }
    }

    fn get_filtered_songs(&self) -> Vec<(usize, &Song)> {
        if self.search_query.is_empty() {
            self.playlist
                .songs
                .iter()
                .enumerate()
                .collect()
        } else {
            let query = self.search_query.to_lowercase();
            self.playlist
                .songs
                .iter()
                .enumerate()
                .filter(|(_, song)| {
                    song.title.to_lowercase().contains(&query)
                        || song
                            .artist
                            .as_ref()
                            .map_or(false, |a| a.to_lowercase().contains(&query))
                        || song
                            .album
                            .as_ref()
                            .map_or(false, |a| a.to_lowercase().contains(&query))
                })
                .collect()
        }
    }

    fn load_folder(&mut self, dir: PathBuf) {
        let mut new_playlist = Playlist::new("Music Library");
        new_playlist.add_songs_from_dir(&dir);
        if !new_playlist.is_empty() {
            self.playlist = new_playlist;
            self.last_folder = Some(dir.clone());
            self.status_message = format!(
                "Loaded {} songs from {:?}",
                self.playlist.len(),
                dir.file_name().unwrap_or_default()
            );
        } else {
            self.status_message = "No music files found in selected folder".to_string();
        }
    }

    fn play_song(&mut self, index: usize) {
        if let Some(song) = self.playlist.play_at(index) {
            self.audio_engine.play(song.path.clone());
            self.audio_engine.set_volume(self.volume);
            self.is_playing = true;
            self.is_paused = false;
            self.status_message = format!("Now playing: {}", song.title);
        }
    }

    fn toggle_play_pause(&mut self) {
        if self.is_playing && !self.is_paused {
            self.audio_engine.pause();
            self.is_paused = true;
            self.status_message = "Paused".to_string();
        } else if self.is_playing && self.is_paused {
            self.audio_engine.resume();
            self.is_paused = false;
            self.status_message = "Resumed".to_string();
        } else if let Some(song) = self.playlist.get_current_song() {
            self.audio_engine.play(song.path.clone());
            self.is_playing = true;
            self.is_paused = false;
            self.status_message = format!("Now playing: {}", song.title);
        }
    }

    fn play_next(&mut self) {
        if let Some(song) = self.playlist.next() {
            self.audio_engine.play(song.path.clone());
            self.audio_engine.set_volume(self.volume);
            self.is_playing = true;
            self.is_paused = false;
            self.status_message = format!("Now playing: {}", song.title);
        }
    }

    fn play_previous(&mut self) {
        if let Some(song) = self.playlist.previous() {
            self.audio_engine.play(song.path.clone());
            self.audio_engine.set_volume(self.volume);
            self.is_playing = true;
            self.is_paused = false;
            self.status_message = format!("Now playing: {}", song.title);
        }
    }
}

impl eframe::App for RustMusicApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Check for audio engine status updates
        while let Some(status) = self.audio_engine.check_status() {
            match status {
                AudioStatus::Playing => {
                    self.is_playing = true;
                    self.is_paused = false;
                }
                AudioStatus::Paused => {
                    self.is_paused = true;
                }
                AudioStatus::Stopped => {
                    self.is_playing = false;
                    self.is_paused = false;
                }
                AudioStatus::Finished => {
                    self.is_playing = false;
                    self.is_paused = false;
                    // Auto-play next song
                    if !self.playlist.is_empty() {
                        if self.playlist.repeat {
                            self.play_next();
                        } else if self.playlist.current_index.is_some()
                            && self.playlist.current_index.unwrap() + 1 < self.playlist.len()
                        {
                            self.play_next();
                        }
                    }
                }
                AudioStatus::Error(msg) => {
                    self.status_message = format!("Error: {}", msg);
                }
            }
        }

        // Main content area (playlist) takes up most of the space
        egui::CentralPanel::default()
            .frame(egui::Frame {
                fill: egui::Color32::from_rgb(18, 18, 18),
                ..Default::default()
            })
            .show(ctx, |ui| {
                // Menu bar
                egui::TopBottomPanel::top("menu_bar")
                    .min_height(40.0)
                    .frame(egui::Frame {
                        fill: egui::Color32::from_rgb(30, 30, 30),
                        ..Default::default()
                    })
                    .show_inside(ui, |ui| {
                        ui.horizontal(|ui| {
                            // Application title
                            ui.label(
                                egui::RichText::new("🎵 RustMusic")
                                    .size(18.0)
                                    .strong()
                                    .color(egui::Color32::from_rgb(0, 200, 150)),
                            );
                            ui.separator();

                            // File menu
                            if ui.button("📂 Open Folder").clicked() {
                                if let Some(dir) = FileDialog::new()
                                    .set_title("Select Music Folder")
                                    .pick_folder()
                                {
                                    self.load_folder(dir);
                                }
                            }

                            // Refresh button - rescans the last opened folder
                            if self.last_folder.is_some() {
                                if ui.button("🔄 Refresh").clicked() {
                                    if let Some(ref dir) = self.last_folder.clone() {
                                        self.load_folder(dir.clone());
                                    }
                                }
                            }

                            if ui.button("➕ Add Files").clicked() {
                                if let Some(files) = FileDialog::new()
                                    .set_title("Select Music Files")
                                    .add_filter("Audio Files", &["mp3", "wav", "flac", "ogg", "m4a"])
                                    .pick_files()
                                {
                                    let count = files.len();
                                    for file in files {
                                        self.playlist.add_song(file);
                                    }
                                    self.status_message = format!("Added {} songs", count);
                                }
                            }

                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                // Search bar
                                ui.add(
                                    egui::TextEdit::singleline(&mut self.search_query)
                                        .hint_text("🔍 Search songs...")
                                        .desired_width(200.0)
                                        .text_color(egui::Color32::WHITE)
                                        .background_color(egui::Color32::from_rgb(50, 50, 50)),
                                );
                            });
                        });
                    });

                // Song list (main content)
                let mut clicked_index: Option<usize> = None;
                egui::ScrollArea::vertical()
                    .auto_shrink([false; 2])
                    .show(ui, |ui| {
                        let songs = self.get_filtered_songs();

                        if songs.is_empty() {
                            ui.add_space(60.0);
                            ui.vertical_centered(|ui| {
                                ui.label(
                                    egui::RichText::new("🎵")
                                        .size(48.0)
                                        .color(egui::Color32::from_rgb(100, 100, 100)),
                                );
                                ui.add_space(10.0);
                                ui.label(
                                    egui::RichText::new("No songs in playlist")
                                        .size(16.0)
                                        .color(egui::Color32::from_rgb(150, 150, 150)),
                                );
                                ui.add_space(5.0);
                                ui.label(
                                    egui::RichText::new("Click 'Open Folder' or 'Add Files' to get started")
                                        .size(12.0)
                                        .color(egui::Color32::from_rgb(120, 120, 120)),
                                );
                            });
                        } else {
                            // Table header
                            egui::Frame::new()
                                .fill(egui::Color32::from_rgb(35, 35, 35))
                                .show(ui, |ui| {
                                    ui.horizontal(|ui| {
                                        ui.add_space(10.0);
                                        ui.label(
                                            egui::RichText::new("#")
                                                .size(12.0)
                                                .color(egui::Color32::from_rgb(150, 150, 150)),
                                        );
                                        ui.add_space(30.0);
                                        ui.label(
                                            egui::RichText::new("TITLE")
                                                .size(12.0)
                                                .strong()
                                                .color(egui::Color32::from_rgb(150, 150, 150)),
                                        );
                                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                            ui.add_space(10.0);
                                        });
                                    });
                                });

                            // Song rows
                            for (i, (original_index, song)) in songs.iter().enumerate() {
                                let is_current = self.playlist.current_index == Some(*original_index);
                                let bg_color = if is_current {
                                    egui::Color32::from_rgb(0, 80, 60)
                                } else if i % 2 == 0 {
                                    egui::Color32::from_rgb(25, 25, 25)
                                } else {
                                    egui::Color32::from_rgb(30, 30, 30)
                                };

                                let response = egui::Frame::new()
                                    .fill(bg_color)
                                    .show(ui, |ui| {
                                        ui.horizontal(|ui| {
                                            // Row number or play icon
                                            ui.add_space(10.0);
                                            if is_current {
                                                ui.label(
                                                    egui::RichText::new("▶")
                                                        .size(14.0)
                                                        .color(egui::Color32::from_rgb(0, 200, 150)),
                                                );
                                            } else {
                                                ui.label(
                                                    egui::RichText::new(format!("{}", original_index + 1))
                                                        .size(12.0)
                                                        .color(egui::Color32::from_rgb(120, 120, 120)),
                                                );
                                            }
                                            ui.add_space(15.0);

                                            // Song title and artist (truncated)
                                            ui.vertical(|ui| {
                                                let truncated_title = Self::truncate_text(&song.title, 40);
                                                ui.label(
                                                    egui::RichText::new(truncated_title)
                                                        .size(14.0)
                                                        .color(if is_current {
                                                            egui::Color32::from_rgb(0, 200, 150)
                                                        } else {
                                                            egui::Color32::WHITE
                                                        }),
                                                );
                                                if let Some(artist) = &song.artist {
                                                    let truncated_artist = Self::truncate_text(artist, 40);
                                                    ui.label(
                                                        egui::RichText::new(truncated_artist)
                                                            .size(11.0)
                                                            .color(egui::Color32::from_rgb(140, 140, 140)),
                                                    );
                                                }
                                            });

                                            // Duration on the right
                                            if song.duration_secs > 0.0 {
                                                ui.with_layout(
                                                    egui::Layout::right_to_left(egui::Align::Center),
                                                    |ui| {
                                                        ui.add_space(10.0);
                                                        ui.label(
                                                            egui::RichText::new(Self::format_time(
                                                                song.duration_secs,
                                                            ))
                                                            .size(12.0)
                                                            .color(egui::Color32::from_rgb(
                                                                140, 140, 140,
                                                            )),
                                                        );
                                                    },
                                                );
                                            }
                                        });
                                    });

                                // Double-click to play
                                let sense = ui.interact(
                                    response.response.rect,
                                    ui.next_auto_id(),
                                    egui::Sense::click(),
                                );
                                if sense.double_clicked() {
                                    clicked_index = Some(*original_index);
                                }
                            }
                        }
                    });
                if let Some(idx) = clicked_index {
                    self.play_song(idx);
                }
            });

        // Bottom player bar
        egui::TopBottomPanel::bottom("player_bar")
            .min_height(80.0)
            .frame(egui::Frame {
                fill: egui::Color32::from_rgb(40, 40, 40),
                ..Default::default()
            })
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    // Song info (left side) - fixed width to prevent layout shift
                    ui.vertical(|ui| {
                        ui.add(egui::Label::new(
                            egui::RichText::new(
                                self.audio_engine
                                    .get_current_song()
                                    .map(|t| Self::truncate_text(&t, 30))
                                    .unwrap_or_else(|| "No track selected".to_string()),
                            )
                            .size(14.0)
                            .strong()
                            .color(egui::Color32::WHITE),
                        ));
                        ui.label(
                            egui::RichText::new(&self.status_message)
                                .size(11.0)
                                .color(egui::Color32::from_rgb(120, 120, 120)),
                        );
                    });

                    // Player controls (center)
                    ui.with_layout(
                        egui::Layout::centered_and_justified(egui::Direction::LeftToRight),
                        |ui| {
                            ui.add_space(50.0);
                            ui.horizontal(|ui| {
                                // Previous button
                                let prev_btn = egui::Button::new(
                                    egui::RichText::new("⏮").size(20.0),
                                )
                                .fill(egui::Color32::from_rgb(50, 50, 50))
                                .min_size(egui::vec2(40.0, 40.0));
                                if ui.add(prev_btn).clicked() {
                                    self.play_previous();
                                }

                                ui.add_space(10.0);

                                // Play/Pause button
                                let play_icon = if self.is_playing && !self.is_paused {
                                    "⏸"
                                } else {
                                    "▶"
                                };
                                let play_btn = egui::Button::new(
                                    egui::RichText::new(play_icon).size(24.0),
                                )
                                .fill(egui::Color32::from_rgb(0, 180, 130))
                                .min_size(egui::vec2(50.0, 40.0));
                                if ui.add(play_btn).clicked() {
                                    self.toggle_play_pause();
                                }

                                ui.add_space(10.0);

                                // Next button
                                let next_btn = egui::Button::new(
                                    egui::RichText::new("⏭").size(20.0),
                                )
                                .fill(egui::Color32::from_rgb(50, 50, 50))
                                .min_size(egui::vec2(40.0, 40.0));
                                if ui.add(next_btn).clicked() {
                                    self.play_next();
                                }
                            });

                            ui.add_space(30.0);

                            // Shuffle/Repeat toggles
                            ui.horizontal(|ui| {
                                let shuffle_btn = egui::Button::new(
                                    egui::RichText::new("🔀").size(16.0),
                                )
                                .fill(egui::Color32::from_rgb(50, 50, 50))
                                .min_size(egui::vec2(32.0, 32.0));
                                if ui.add(shuffle_btn).clicked() {
                                    self.playlist.shuffle = !self.playlist.shuffle;
                                }

                                let repeat_btn = egui::Button::new(
                                    egui::RichText::new("🔁").size(16.0),
                                )
                                .fill(egui::Color32::from_rgb(50, 50, 50))
                                .min_size(egui::vec2(32.0, 32.0));
                                if ui.add(repeat_btn).clicked() {
                                    self.playlist.repeat = !self.playlist.repeat;
                                }
                            });

                            ui.add_space(50.0);

                            // Progress bar and time
                            let position = self.audio_engine.get_position();
                            let duration = self.audio_engine.get_duration();

                            ui.vertical(|ui| {
                                ui.add_space(5.0);
                                ui.horizontal(|ui| {
                                    ui.label(
                                        egui::RichText::new(Self::format_time(position))
                                            .size(12.0)
                                            .color(egui::Color32::from_rgb(180, 180, 180)),
                                    );

                                    // Progress slider - now properly seeks
                                    let mut progress = if duration > 0.0 {
                                        (position / duration) as f32
                                    } else {
                                        0.0
                                    };
                                    let slider = egui::Slider::new(&mut progress, 0.0..=1.0)
                                        .show_value(false);
                                    if ui.add_sized(egui::vec2(200.0, 20.0), slider).drag_stopped() {
                                        let new_pos = progress as f64 * duration;
                                        self.audio_engine.seek(new_pos);
                                    }

                                    ui.label(
                                        egui::RichText::new(Self::format_time(duration))
                                            .size(12.0)
                                            .color(egui::Color32::from_rgb(180, 180, 180)),
                                    );
                                });
                            });
                        },
                    );

                    // Volume control (right side)
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.add_space(10.0);
                        ui.horizontal(|ui| {
                            let volume_icon = if self.volume == 0.0 {
                                "🔇"
                            } else if self.volume < 0.5 {
                                "🔉"
                            } else {
                                "🔊"
                            };
                            ui.label(
                                egui::RichText::new(volume_icon).size(16.0),
                            );
                            let mut vol = self.volume;
                            ui.add_sized(
                                egui::vec2(100.0, 20.0),
                                egui::Slider::new(&mut vol, 0.0..=1.0)
                                    .show_value(false),
                            );
                            if vol != self.volume {
                                self.volume = vol;
                                self.audio_engine.set_volume(self.volume);
                            }
                        });
                    });
                });
            });

        // Request continuous repaint for real-time updates
        ctx.request_repaint();
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1000.0, 700.0])
            .with_min_inner_size([600.0, 400.0])
            .with_title("RustMusic Player"),
        ..Default::default()
    };

    eframe::run_native(
        "RustMusic",
        options,
        Box::new(|cc| Ok(Box::new(RustMusicApp::new(cc)))),
    )
}