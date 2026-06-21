mod audio;
mod playlist;
mod config;
mod theme;
mod ui;

use eframe::egui;
use ui::RustMusicApp;

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