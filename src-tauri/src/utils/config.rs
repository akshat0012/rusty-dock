use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Clone, Serialize)]
pub struct DockSettings {
    pub border_radius: i32,
    pub position: String,
    pub bg_color: String,
    pub height: i32,
    pub top_padding: i32,
    pub left_padding: i32,
    pub right_padding: i32,
    pub bottom_padding: i32,
}

#[derive(Deserialize, Debug, Clone, Serialize)]
pub struct ShortcutsConfig {

}

#[derive(Deserialize, Debug, Clone, Serialize)]
pub struct ClockConfig {

}

#[derive(Deserialize, Debug, Clone, Serialize)]
pub struct Config {
    pub dock_settings: DockSettings
}
