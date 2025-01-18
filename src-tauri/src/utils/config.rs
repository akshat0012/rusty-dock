use std::fs::File;
use std::io::Read;
use serde_json;
use serde::{Deserialize};

#[derive(Deserialize, Debug)]
pub struct Config {
    pub height: i32,
    pub theme: String,
    pub top_offset: i32,
    pub position: String,
    pub left_offset: i32,
    pub right_offset: i32,
    pub bottom_offset: i32
}

pub fn read_config(config_file_path: Option<&str>) -> Result<Config, String> {
    let config_file_path = config_file_path.unwrap_or(r"C:\ProgramData\widget-test\config.json");
    let mut file = File::open(config_file_path).map_err(|e| format!("Error::{}", e))?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).map_err(|e| format!("ERROR::{}", e))?;
    serde_json::from_str::<Config>(&contents).map_err(|e| format!("ERROR::{}", e))
}
