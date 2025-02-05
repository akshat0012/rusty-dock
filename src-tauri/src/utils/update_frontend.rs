use tauri::Emitter;
use tauri::AppHandle;

use super::config::Config;

pub fn update_frontend(app: &AppHandle, current_config: &Config) -> Result<(), String> {
    match app.emit("send_config_data", current_config) {
        Ok(_) => {
            // println!("SUCCESS:: app.emit()");
            Ok(())
        }
        Err(e) => {
            Err(format!("ERROR:: app.emit()\n{}", e))
        }
    }
}

