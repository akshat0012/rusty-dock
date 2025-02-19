use tauri::AppHandle;
use super::update_frontend::update_frontend;

pub fn toggle_quick_window(app: &AppHandle, quick_window: &tauri::WebviewWindow, action: &str) {
    if action == "\"show\"" {

        match update_frontend(
                app, 
                true, 
                None,
                Some("quick_window")
            ) {
            Ok(_) => {
                // println!("SUCCESS:: toggle_quick_window() -> update_frontend()");
                match quick_window.show() {
                    Ok(_) => {
                        // println!("SUCCESS:: toggle_quick_window::quick_window.show()");
                    }
                    Err(e) => {
                        println!("ERROR:: toggle_quick_window::quick_window.show()\n{}", e);
                    }
                }
            }
            Err(e) => {
                println!("ERROR:: toggle_quick_window() -> update_frontend()\n{}", e);
            }
        }

    } else if action == "\"hide\"" {
        match quick_window.hide() {
            Ok(_) => {
                // println!("SUCCESS:: toggle_quick_window::quick_window.hide()");
            }
            Err(e) => {
                println!("ERROR:: toggle_quick_window::quick_window.hide()\n{}", e);
            }
        }
    }
}
