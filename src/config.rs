use facet::Facet;
use eframe::egui;

#[derive(Facet, Clone)]
pub struct Config {
    #[facet(default = "#FFFFFF".to_string())]
    pub bg_color: String,
    #[facet(default = "#000000".to_string())]
    pub point_color: String,
    #[facet(default = "#FF0000".to_string())]
    pub selected_color: String,
    #[facet(default = "#0000FF".to_string())]
    pub selection_box_color: String,
    #[facet(default = true)]
    pub grid_enabled: bool,
    #[facet(default = 40.0)]
    pub grid_spacing: f32,
    #[facet(default = "#CCCCCC".to_string())]
    pub grid_color: String,
    #[facet(default = 20.0)]
    pub point_radius: f32,
    #[facet(default = 1.0)]
    pub move_step: f32,
    #[facet(default = 20.0)]
    pub move_step_large: f32,
}

impl Config {
    pub fn load() -> Self {
        if let Ok(contents) = std::fs::read_to_string("config.toml") {
            if let Ok(config) = facet_toml::from_str::<Config>(&contents) {
                return config;
            }
        }
        facet_toml::from_str::<Config>("").unwrap()
    }

    pub fn parse_colour(hex: &str) -> egui::Color32 {
        let hex = hex.trim_start_matches('#');
        let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(0);
        let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(0);
        let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(0);
        egui::Color32::from_rgb(r, g, b)
    }
}
