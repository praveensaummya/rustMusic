use crate::audio::{AudioEngine, AudioStatus};
use crate::config::AppConfig;
use crate::playlist::{Playlist, Song};
use crate::theme::Theme;
use eframe::egui;
use rfd::FileDialog;
use std::path::PathBuf;

pub struct RustMusicApp {
    pub audio_engine: AudioEngine,
    pub playlist: Playlist,
    pub volume: f32,
    pub is_playing: bool,
    pub is_paused: bool,
    pub search_query: String,
    pub status_message: String,
    pub last_folder: Option<PathBuf>,
    pub theme: Theme,
    pub config: AppConfig,
    pub show_settings: bool,
    pub mini_mode: bool,
    pub prev_mini_mode: bool,
}

impl RustMusicApp {
    pub fn new(cc: &eframe::CreationContext) -> Self {
        let config = AppConfig::load();
        let theme = Theme::from_name(&config.theme);
        let volume = config.volume;
        let last_folder = config
            .last_folder
            .as_ref()
            .map(|s| PathBuf::from(s));

        Self::apply_theme(&cc.egui_ctx, theme);

        let mut app = RustMusicApp {
            audio_engine: AudioEngine::new(),
            playlist: Playlist::new("Default"),
            volume,
            is_playing: false,
            is_paused: false,
            search_query: String::new(),
            status_message: "Welcome to RustMusic!".to_string(),
            last_folder: last_folder.clone(),
            theme,
            config,
            show_settings: false,
            mini_mode: false,
            prev_mini_mode: false,
        };

        if let Some(ref folder) = last_folder {
            app.load_folder(folder.clone());
            app.status_message = format!(
                "Loaded {} songs from last session",
                app.playlist.len()
            );
        }

        app
    }

    fn apply_theme(ctx: &egui::Context, theme: Theme) {
        let mut style = (*ctx.style()).clone();
        style.visuals.dark_mode = matches!(theme, Theme::Dark | Theme::Midnight | Theme::Ocean | Theme::Forest);
        style.visuals.window_fill = theme.settings_bg();
        style.visuals.panel_fill = theme.bg_main();
        ctx.set_style(style);
    }

    fn save_config(&self) {
        let mut config = self.config.clone();
        config.theme = self.theme.name().to_string();
        config.volume = self.volume;
        config.last_folder = self
            .last_folder
            .as_ref()
            .map(|p| p.to_string_lossy().to_string());
        config.save();
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

    fn truncate_text(text: &str, max_chars: usize) -> String {
        if text.chars().count() > max_chars {
            format!("{}...", text.chars().take(max_chars).collect::<String>())
        } else {
            text.to_string()
        }
    }

    fn get_filtered_songs(&self) -> Vec<(usize, &Song)> {
        if self.search_query.is_empty() {
            self.playlist.songs.iter().enumerate().collect()
        } else {
            let query = self.search_query.to_lowercase();
            self.playlist
                .songs
                .iter()
                .enumerate()
                .filter(|(_, song)| {
                    song.title.to_lowercase().contains(&query)
                        || song.artist.as_ref().map_or(false, |a| a.to_lowercase().contains(&query))
                        || song.album.as_ref().map_or(false, |a| a.to_lowercase().contains(&query))
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
        self.save_config();
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

    fn render_menu_bar(&mut self, ui: &mut egui::Ui) {
        let t = self.theme;
        egui::TopBottomPanel::top("menu_bar")
            .min_height(40.0)
            .frame(egui::Frame {
                fill: t.bg_surface(),
                inner_margin: egui::Margin::symmetric(12, 10),
                ..Default::default()
            })
            .show_inside(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label(
                        egui::RichText::new("🎵 RustMusic")
                            .size(18.0)
                            .strong()
                            .color(t.accent()),
                    );
                    ui.separator();

                    let menu_btn = |label: egui::RichText| {
                        egui::Button::new(label)
                            .fill(t.btn_bg())
                            .corner_radius(egui::CornerRadius::same(6))
                            .min_size(egui::vec2(0.0, 28.0))
                    };

                    ui.add_space(8.0);
                    if ui.add(menu_btn(egui::RichText::new("📂 Open Folder").size(13.0).color(t.text_primary()))).clicked() {
                        if let Some(dir) = FileDialog::new()
                            .set_title("Select Music Folder")
                            .pick_folder()
                        {
                            self.load_folder(dir);
                        }
                    }

                    if self.last_folder.is_some() {
                        ui.add_space(4.0);
                        if ui.add(menu_btn(egui::RichText::new("🔄 Refresh").size(13.0).color(t.text_primary()))).clicked() {
                            if let Some(ref dir) = self.last_folder.clone() {
                                self.load_folder(dir.clone());
                            }
                        }
                    }

                    ui.add_space(4.0);
                        if ui.add(menu_btn(egui::RichText::new("➕ Add Files").size(13.0).color(t.text_primary()))).clicked() {
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

                    ui.add_space(10.0);

                    if ui.add(menu_btn(egui::RichText::new("▼ Mini Mode").size(13.0).color(t.text_primary()))).clicked() {
                        self.mini_mode = !self.mini_mode;
                    }

                    if ui.add(menu_btn(egui::RichText::new("⚙ Settings").size(13.0).color(t.text_primary()))).clicked() {
                        self.show_settings = !self.show_settings;
                    }

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.add(
                            egui::TextEdit::singleline(&mut self.search_query)
                                .hint_text("🔍 Search songs...")
                                .desired_width(200.0)
                                .text_color(t.text_primary())
                                .background_color(t.search_bg()),
                        );
                    });
                });
            });
    }

    fn render_playlist(&mut self, ui: &mut egui::Ui) -> Option<usize> {
        let t = self.theme;
        let mut clicked_index: Option<usize> = None;
        let songs = self.get_filtered_songs();

        if songs.is_empty() {
            ui.add_space(60.0);
            ui.vertical_centered(|ui| {
                ui.label(egui::RichText::new("🎵").size(48.0).color(t.text_dim()));
                ui.add_space(10.0);
                ui.label(egui::RichText::new("No songs in playlist").size(16.0).color(t.text_secondary()));
                ui.add_space(5.0);
                ui.label(egui::RichText::new("Click 'Open Folder' or 'Add Files' to get started").size(12.0).color(t.text_dim()));
            });
        } else {
            egui::Frame::new()
                .fill(t.bg_header())
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.add_space(10.0);
                        ui.label(egui::RichText::new("#").size(12.0).color(t.text_dim()));
                        ui.add_space(30.0);
                        ui.label(egui::RichText::new("TITLE").size(12.0).strong().color(t.text_dim()));
                    });
                });

            for (i, (original_index, song)) in songs.iter().enumerate() {
                let is_current = self.playlist.current_index == Some(*original_index);
                let bg_color = if is_current { t.bg_row_current() } else if i % 2 == 0 { t.bg_row_even() } else { t.bg_row_odd() };

                let response = egui::Frame::new()
                    .fill(bg_color)
                    .show(ui, |ui| {
                        ui.horizontal(|ui| {
                            ui.add_space(10.0);
                            if is_current {
                                ui.label(egui::RichText::new("▶").size(14.0).color(t.accent()));
                            } else {
                                ui.label(egui::RichText::new(format!("{}", original_index + 1)).size(12.0).color(t.text_dim()));
                            }
                            ui.add_space(15.0);

                            ui.vertical(|ui| {
                                ui.label(egui::RichText::new(&song.title).size(14.0).color(if is_current { t.accent() } else { t.text_primary() }));
                                if let Some(artist) = &song.artist {
                                    ui.label(egui::RichText::new(artist).size(11.0).color(t.text_secondary()));
                                }
                            });

                            if song.duration_secs > 0.0 {
                                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                    ui.add_space(10.0);
                                    ui.label(egui::RichText::new(Self::format_time(song.duration_secs)).size(12.0).color(t.text_dim()));
                                });
                            }
                        });
                    });

                if ui.interact(response.response.rect, ui.next_auto_id(), egui::Sense::click()).double_clicked() {
                    clicked_index = Some(*original_index);
                }
            }
        }

        clicked_index
    }

    fn render_settings_window(&mut self, ctx: &egui::Context) {
        let show = &mut self.show_settings;
        let theme_current = self.theme;
        let volume_current = self.volume;
        let has_folder = self.last_folder.is_some();
        let folder_display = self.last_folder.as_ref().map(|f| format!("{}", f.display()));

        let mut selected_theme = theme_current;
        let mut theme_changed = false;
        let mut new_volume = volume_current;
        let mut vol_changed = false;

        egui::Window::new("⚙ Settings")
            .open(show)
            .resizable(true)
            .default_size([400.0, 350.0])
            .frame(egui::Frame {
                fill: theme_current.settings_bg(),
                corner_radius: egui::CornerRadius::same(12),
                inner_margin: egui::Margin::same(24),
                ..Default::default()
            })
            .show(ctx, |ui| {
                ui.label(egui::RichText::new("🎨 Theme").size(16.0).strong().color(theme_current.text_primary()));
                ui.add_space(5.0);

                for theme_variant in Theme::all() {
                    let is_selected = selected_theme == *theme_variant;
                    let label = if is_selected { format!("● {}", theme_variant.name()) } else { format!("○ {}", theme_variant.name()) };
                    if ui.add(egui::Button::new(egui::RichText::new(&label).color(if is_selected { theme_variant.accent() } else { theme_current.text_secondary() })).fill(theme_current.btn_bg()).min_size(egui::vec2(180.0, 28.0))).clicked() {
                        selected_theme = *theme_variant;
                        theme_changed = true;
                        Self::apply_theme(ctx, selected_theme);
                    }
                }

                ui.add_space(15.0);
                ui.separator();
                ui.add_space(10.0);

                ui.label(egui::RichText::new("🔊 Default Volume").size(16.0).strong().color(theme_current.text_primary()));
                ui.add_space(5.0);
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new(format!("{:.0}%", new_volume * 100.0)).color(theme_current.text_secondary()));
                    ui.add_sized(egui::vec2(200.0, 20.0), egui::Slider::new(&mut new_volume, 0.0..=1.0).show_value(false));
                });
                if new_volume != volume_current {
                    vol_changed = true;
                }

                ui.add_space(15.0);
                ui.separator();
                ui.add_space(10.0);

                if has_folder {
                    if let Some(ref display) = folder_display {
                        ui.label(egui::RichText::new("📁 Last Session Folder").size(16.0).strong().color(theme_current.text_primary()));
                        ui.add_space(5.0);
                        ui.label(egui::RichText::new(display).size(12.0).color(theme_current.text_secondary()));
                    }
                }

                ui.add_space(20.0);

                if ui.add(egui::Button::new(egui::RichText::new("💾 Save Preferences").size(14.0)).fill(theme_current.accent_dim()).min_size(egui::vec2(180.0, 32.0))).clicked() {}
            });

        if theme_changed {
            self.theme = selected_theme;
        }
        if vol_changed {
            self.volume = new_volume;
            self.audio_engine.set_volume(self.volume);
        }
        if theme_changed || vol_changed {
            self.save_config();
            self.status_message = "Preferences saved!".to_string();
        }
    }

    fn render_player_bar(&mut self, ctx: &egui::Context) {
        let t = self.theme;
        egui::TopBottomPanel::bottom("player_bar")
            .min_height(90.0)
            .frame(egui::Frame {
                fill: t.bg_player(),
                inner_margin: egui::Margin::symmetric(24, 20),
                ..Default::default()
            })
            .show(ctx, |ui| {
                let total_width = ui.available_width();
                ui.horizontal(|ui| {
                    let left_w = total_width * 0.25;
                    let center_w = total_width * 0.5;
                    let right_w = total_width * 0.25;

                    ui.allocate_ui_with_layout(egui::vec2(left_w, ui.available_height()), egui::Layout::left_to_right(egui::Align::Center), |ui| {
                        ui.set_min_width(left_w);
                        ui.vertical(|ui| {
                            ui.add(egui::Label::new(egui::RichText::new(self.audio_engine.get_current_song().map(|t| Self::truncate_text(&t, 40)).unwrap_or_else(|| "No track selected".to_string())).size(15.0).strong().color(t.text_primary())));
                            ui.add_space(4.0);
                            ui.label(egui::RichText::new(&self.status_message).size(12.0).color(t.text_dim()));
                        });
                    });

                    ui.allocate_ui_with_layout(egui::vec2(center_w, ui.available_height()), egui::Layout::left_to_right(egui::Align::Center), |ui| {
                        ui.set_min_width(center_w);
                        let fixed_w = 230.0 + 8.0 + 90.0;
                        let slider_w = (center_w - fixed_w).clamp(50.0, 400.0);
                        let content_w = fixed_w + slider_w;
                        let padding = (center_w - content_w) / 2.0;
                        ui.add_space(padding.max(0.0));
                        self.render_playback_buttons(ui);
                        ui.add_space(8.0);
                        self.render_progress(ui, slider_w + 90.0);
                    });

                    ui.allocate_ui_with_layout(egui::vec2(right_w, ui.available_height()), egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.set_min_width(right_w);
                        ui.add_space(20.0);
                        self.render_volume(ui);
                    });
                });
            });
    }

    fn render_playback_buttons(&mut self, ui: &mut egui::Ui) {
        let t = self.theme;
        
        // Shuffle
        if ui.add(egui::Button::new(egui::RichText::new("🔀").size(16.0).color(t.text_dim())).fill(egui::Color32::TRANSPARENT).min_size(egui::vec2(32.0, 32.0))).clicked() {
            self.playlist.shuffle = !self.playlist.shuffle;
        }

        ui.add_space(10.0);

        // Previous
        if ui.add(egui::Button::new(egui::RichText::new("⏮").size(20.0).color(t.text_primary())).fill(egui::Color32::TRANSPARENT).min_size(egui::vec2(40.0, 40.0))).clicked() {
            self.play_previous();
        }

        ui.add_space(10.0);

        // Play/Pause
        let play_icon = if self.is_playing && !self.is_paused { "⏸" } else { "▶" };
        if ui.add(egui::Button::new(egui::RichText::new(play_icon).size(24.0).color(t.bg_main())).fill(t.accent()).corner_radius(egui::CornerRadius::same(25)).min_size(egui::vec2(45.0, 45.0))).clicked() {
            self.toggle_play_pause();
        }

        ui.add_space(10.0);

        // Next
        if ui.add(egui::Button::new(egui::RichText::new("⏭").size(20.0).color(t.text_primary())).fill(egui::Color32::TRANSPARENT).min_size(egui::vec2(40.0, 40.0))).clicked() {
            self.play_next();
        }

        ui.add_space(10.0);

        // Repeat
        if ui.add(egui::Button::new(egui::RichText::new("🔁").size(16.0).color(t.text_dim())).fill(egui::Color32::TRANSPARENT).min_size(egui::vec2(32.0, 32.0))).clicked() {
            self.playlist.repeat = !self.playlist.repeat;
        }
    }

    fn render_progress(&mut self, ui: &mut egui::Ui, full_width: f32) {
        let t = self.theme;
        let position = self.audio_engine.get_position();
        let duration = self.audio_engine.get_duration();

        ui.label(egui::RichText::new(Self::format_time(position)).size(12.0).color(t.text_dim()));

        let mut progress = if duration > 0.0 { (position / duration) as f32 } else { 0.0 };
        let slider_w = (full_width - 90.0).max(100.0);
        let orig_w = ui.spacing().slider_width;
        ui.spacing_mut().slider_width = slider_w;
        
        let slider = egui::Slider::new(&mut progress, 0.0..=1.0).show_value(false);
        if ui.add(slider).drag_stopped() {
            self.audio_engine.seek(progress as f64 * duration);
        }
        
        ui.spacing_mut().slider_width = orig_w;

        ui.label(egui::RichText::new(Self::format_time(duration)).size(12.0).color(t.text_dim()));
    }

    fn render_volume(&mut self, ui: &mut egui::Ui) {
        let t = self.theme;
        let volume_icon = if self.volume == 0.0 { "🔇" } else if self.volume < 0.3 { "🔈" } else if self.volume < 0.7 { "🔉" } else { "🔊" };
        ui.label(egui::RichText::new(volume_icon).size(18.0).color(t.text_primary()));
        
        let mut vol = self.volume;
        ui.add_sized(egui::vec2(100.0, 20.0), egui::Slider::new(&mut vol, 0.0..=1.0).show_value(false));
        if vol != self.volume {
            self.volume = vol;
            self.audio_engine.set_volume(self.volume);
        }
    }

    fn render_mini_player(&mut self, ctx: &egui::Context) {
        let t = self.theme;
        
egui::Window::new("🎵 Mini Player")
             .resizable(false)
             .default_size([422.0, 140.0])
             .min_size([422.0, 140.0])
            .frame(egui::Frame {
                fill: t.bg_surface(),
                corner_radius: egui::CornerRadius::same(12),
                inner_margin: egui::Margin::same(16),
                ..Default::default()
            })
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.vertical(|ui| {
                        let song_title = self.audio_engine.get_current_song().unwrap_or_else(|| "No track".to_string());
                        ui.label(egui::RichText::new(song_title).size(16.0).strong().color(t.text_primary()));
                    });

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.add(egui::Button::new(egui::RichText::new("✕").size(14.0).color(t.text_dim())).fill(egui::Color32::TRANSPARENT).min_size(egui::vec2(24.0, 24.0))).clicked() {
                            self.mini_mode = false;
                        }
                    });
                });

                ui.add_space(12.0);
                self.render_progress(ui, ui.available_width());
                ui.add_space(12.0);

                ui.horizontal(|ui| {
                    ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                        self.render_playback_buttons(ui);
                    });
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        self.render_volume(ui);
                    });
                });
            });
    }
}

impl eframe::App for RustMusicApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Resize and decorate window when entering/exiting mini mode
        if self.mini_mode != self.prev_mini_mode {
            if self.mini_mode {
                ctx.send_viewport_cmd(egui::ViewportCommand::InnerSize(egui::vec2(460.0, 200.0)));
                ctx.send_viewport_cmd(egui::ViewportCommand::Decorations(false));
            } else {
                ctx.send_viewport_cmd(egui::ViewportCommand::InnerSize(egui::vec2(1000.0, 700.0)));
                ctx.send_viewport_cmd(egui::ViewportCommand::Decorations(true));
            }
            self.prev_mini_mode = self.mini_mode;
        }

        while let Some(status) = self.audio_engine.check_status() {
            match status {
                AudioStatus::Playing => { self.is_playing = true; self.is_paused = false; }
                AudioStatus::Paused => { self.is_paused = true; }
                AudioStatus::Stopped => { self.is_playing = false; self.is_paused = false; }
                AudioStatus::Finished => {
                    self.is_playing = false;
                    self.is_paused = false;
                    if !self.playlist.is_empty() {
                        if self.playlist.repeat {
                            self.play_next();
                        } else if self.playlist.current_index.is_some() && self.playlist.current_index.unwrap() + 1 < self.playlist.len() {
                            self.play_next();
                        }
                    }
                }
                AudioStatus::Error(msg) => {
                    self.status_message = format!("Error: {}", msg);
                }
            }
        }

        if self.mini_mode {
            self.render_mini_player(ctx);
        } else {
            egui::CentralPanel::default()
                .frame(egui::Frame { fill: self.theme.bg_main(), ..Default::default() })
                .show(ctx, |ui| {
                    self.render_menu_bar(ui);
                    let clicked_index = egui::ScrollArea::vertical().auto_shrink([false; 2]).show(ui, |ui| self.render_playlist(ui)).inner;
                    if let Some(idx) = clicked_index {
                        self.play_song(idx);
                    }
                });
            self.render_player_bar(ctx);
        }

        if self.show_settings {
            self.render_settings_window(ctx);
        }

        let input = ctx.input(|i| i.clone());
        if input.key_pressed(egui::Key::Space) {
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
        if input.key_pressed(egui::Key::ArrowLeft) { self.play_previous(); }
        if input.key_pressed(egui::Key::ArrowRight) { self.play_next(); }
        if input.key_pressed(egui::Key::R) {
            self.playlist.repeat = !self.playlist.repeat;
            self.status_message = format!("Repeat: {}", if self.playlist.repeat { "ON" } else { "OFF" });
        }
        if input.key_pressed(egui::Key::S) {
            self.playlist.shuffle = !self.playlist.shuffle;
            self.status_message = format!("Shuffle: {}", if self.playlist.shuffle { "ON" } else { "OFF" });
        }

        if self.is_playing && !self.is_paused {
            ctx.request_repaint();
        }
    }

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        self.save_config();
    }
}