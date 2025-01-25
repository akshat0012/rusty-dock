use std::fs::File;
use std::io::Read;
use super::config::Config;

pub fn read_config(config_file_path: Option<&str>) -> Result<Config, String> {
    let config_file_path = config_file_path.unwrap_or(r"C:\ProgramData\rusty-dock\config.json");
    let mut file = File::open(config_file_path).map_err(|e| format!("Error::{}", e))?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).map_err(|e| format!("ERROR::{}", e))?;
    serde_json::from_str::<Config>(&contents).map_err(|e| format!("ERROR::{}", e))
}
