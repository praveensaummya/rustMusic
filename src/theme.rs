use eframe::egui::Color32;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Theme {
    Dark,
    Light,
    Midnight,
    Ocean,
    Forest,
}

impl Theme {
    pub fn all() -> &'static [Theme] {
        &[
            Theme::Dark,
            Theme::Light,
            Theme::Midnight,
            Theme::Ocean,
            Theme::Forest,
        ]
    }

    pub fn name(&self) -> &'static str {
        match self {
            Theme::Dark => "Dark",
            Theme::Light => "Light",
            Theme::Midnight => "Midnight",
            Theme::Ocean => "Ocean",
            Theme::Forest => "Forest",
        }
    }

    pub fn from_name(name: &str) -> Self {
        match name {
            "Light" => Theme::Light,
            "Midnight" => Theme::Midnight,
            "Ocean" => Theme::Ocean,
            "Forest" => Theme::Forest,
            _ => Theme::Dark,
        }
    }

    // Background colors
    pub fn bg_main(&self) -> Color32 {
        match self {
            Theme::Dark => Color32::from_rgb(18, 18, 18),
            Theme::Light => Color32::from_rgb(245, 245, 245),
            Theme::Midnight => Color32::from_rgb(10, 10, 30),
            Theme::Ocean => Color32::from_rgb(15, 25, 35),
            Theme::Forest => Color32::from_rgb(15, 25, 15),
        }
    }

    pub fn bg_surface(&self) -> Color32 {
        match self {
            Theme::Dark => Color32::from_rgb(30, 30, 30),
            Theme::Light => Color32::from_rgb(220, 220, 220),
            Theme::Midnight => Color32::from_rgb(20, 20, 45),
            Theme::Ocean => Color32::from_rgb(20, 35, 50),
            Theme::Forest => Color32::from_rgb(20, 35, 20),
        }
    }

    pub fn bg_player(&self) -> Color32 {
        match self {
            Theme::Dark => Color32::from_rgb(40, 40, 40),
            Theme::Light => Color32::from_rgb(200, 200, 200),
            Theme::Midnight => Color32::from_rgb(25, 25, 55),
            Theme::Ocean => Color32::from_rgb(25, 45, 60),
            Theme::Forest => Color32::from_rgb(25, 45, 25),
        }
    }

    pub fn bg_row_even(&self) -> Color32 {
        match self {
            Theme::Dark => Color32::from_rgb(25, 25, 25),
            Theme::Light => Color32::from_rgb(235, 235, 235),
            Theme::Midnight => Color32::from_rgb(18, 18, 40),
            Theme::Ocean => Color32::from_rgb(22, 38, 52),
            Theme::Forest => Color32::from_rgb(22, 38, 22),
        }
    }

    pub fn bg_row_odd(&self) -> Color32 {
        match self {
            Theme::Dark => Color32::from_rgb(30, 30, 30),
            Theme::Light => Color32::from_rgb(225, 225, 225),
            Theme::Midnight => Color32::from_rgb(22, 22, 45),
            Theme::Ocean => Color32::from_rgb(26, 42, 56),
            Theme::Forest => Color32::from_rgb(26, 42, 26),
        }
    }

    pub fn bg_row_current(&self) -> Color32 {
        match self {
            Theme::Dark => Color32::from_rgb(0, 80, 60),
            Theme::Light => Color32::from_rgb(180, 230, 210),
            Theme::Midnight => Color32::from_rgb(0, 50, 80),
            Theme::Ocean => Color32::from_rgb(0, 60, 80),
            Theme::Forest => Color32::from_rgb(0, 70, 40),
        }
    }

    pub fn bg_header(&self) -> Color32 {
        match self {
            Theme::Dark => Color32::from_rgb(35, 35, 35),
            Theme::Light => Color32::from_rgb(210, 210, 210),
            Theme::Midnight => Color32::from_rgb(28, 28, 50),
            Theme::Ocean => Color32::from_rgb(30, 48, 62),
            Theme::Forest => Color32::from_rgb(30, 48, 30),
        }
    }

    // Accent / primary color
    pub fn accent(&self) -> Color32 {
        match self {
            Theme::Dark => Color32::from_rgb(0, 200, 150),
            Theme::Light => Color32::from_rgb(0, 150, 100),
            Theme::Midnight => Color32::from_rgb(80, 140, 255),
            Theme::Ocean => Color32::from_rgb(60, 180, 220),
            Theme::Forest => Color32::from_rgb(80, 200, 80),
        }
    }

    pub fn accent_dim(&self) -> Color32 {
        match self {
            Theme::Dark => Color32::from_rgb(0, 180, 130),
            Theme::Light => Color32::from_rgb(0, 130, 80),
            Theme::Midnight => Color32::from_rgb(60, 110, 220),
            Theme::Ocean => Color32::from_rgb(40, 150, 190),
            Theme::Forest => Color32::from_rgb(60, 170, 60),
        }
    }

    // Text colors
    pub fn text_primary(&self) -> Color32 {
        match self {
            Theme::Dark => Color32::WHITE,
            Theme::Light => Color32::BLACK,
            Theme::Midnight => Color32::from_rgb(200, 210, 255),
            Theme::Ocean => Color32::from_rgb(200, 220, 240),
            Theme::Forest => Color32::from_rgb(200, 240, 200),
        }
    }

    pub fn text_secondary(&self) -> Color32 {
        match self {
            Theme::Dark => Color32::from_rgb(140, 140, 140),
            Theme::Light => Color32::from_rgb(100, 100, 100),
            Theme::Midnight => Color32::from_rgb(130, 140, 190),
            Theme::Ocean => Color32::from_rgb(130, 160, 180),
            Theme::Forest => Color32::from_rgb(130, 180, 130),
        }
    }

    pub fn text_dim(&self) -> Color32 {
        match self {
            Theme::Dark => Color32::from_rgb(120, 120, 120),
            Theme::Light => Color32::from_rgb(140, 140, 140),
            Theme::Midnight => Color32::from_rgb(110, 120, 160),
            Theme::Ocean => Color32::from_rgb(110, 140, 160),
            Theme::Forest => Color32::from_rgb(110, 150, 110),
        }
    }

    // Button / control colors
    pub fn btn_bg(&self) -> Color32 {
        match self {
            Theme::Dark => Color32::from_rgb(50, 50, 50),
            Theme::Light => Color32::from_rgb(180, 180, 180),
            Theme::Midnight => Color32::from_rgb(35, 35, 65),
            Theme::Ocean => Color32::from_rgb(40, 55, 70),
            Theme::Forest => Color32::from_rgb(40, 55, 40),
        }
    }

    pub fn btn_play(&self) -> Color32 {
        match self {
            Theme::Dark => Color32::from_rgb(0, 180, 130),
            Theme::Light => Color32::from_rgb(0, 150, 100),
            Theme::Midnight => Color32::from_rgb(60, 120, 230),
            Theme::Ocean => Color32::from_rgb(40, 160, 200),
            Theme::Forest => Color32::from_rgb(60, 180, 60),
        }
    }

    pub fn search_bg(&self) -> Color32 {
        match self {
            Theme::Dark => Color32::from_rgb(50, 50, 50),
            Theme::Light => Color32::from_rgb(200, 200, 200),
            Theme::Midnight => Color32::from_rgb(35, 35, 60),
            Theme::Ocean => Color32::from_rgb(40, 55, 70),
            Theme::Forest => Color32::from_rgb(40, 55, 40),
        }
    }

    // Settings window
    pub fn settings_bg(&self) -> Color32 {
        match self {
            Theme::Dark => Color32::from_rgb(40, 40, 45),
            Theme::Light => Color32::from_rgb(230, 230, 235),
            Theme::Midnight => Color32::from_rgb(30, 30, 55),
            Theme::Ocean => Color32::from_rgb(30, 45, 60),
            Theme::Forest => Color32::from_rgb(30, 45, 30),
        }
    }
}