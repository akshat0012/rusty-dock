use tauri::Emitter;
use tauri::AppHandle;

use super::config::Config;
use super::read_config::read_config;

pub fn update_frontend(
        app: &AppHandle, 
        read_: bool, 
        current_config: Option<&Config>,
        window_label: Option<&str>
) -> Result<Config, String> {

    let updated_config = if read_ {
        match read_config(None) {
            Ok(new_config) => {
                // println!("SUCCESS:: update_frontend() -> read_config()");
                new_config
            }
            Err(e) => {
                return Err(format!("ERROR:: update_frontend() -> read_config()\n{}", e));
            }
        }
    } else {
        match current_config {
            Some(config) => config.clone(),
            None => return Err("No current config available".to_string()),
        }
    };


    if let Some(window_) = window_label {
        if let Err(e) = app.emit_to(window_, &["send_config_data_to_", window_].concat(), &updated_config) {
            return Err(format!("ERROR:: app.emit_to()\n{}", e));
        }
    } else {
        if let Err(e) = app.emit("send_config_data", &updated_config) {
            return Err(format!("ERROR:: app.emit()\n{}", e));
        }
    }

    Ok(updated_config)
}
