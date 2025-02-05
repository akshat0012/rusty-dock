pub fn toggle_quick_window(quick_window: &tauri::WebviewWindow, action: &str) {
    if action == "\"show\"" {
        match quick_window.show() {
            Ok(_) => {
                // println!("SUCCESS:: toggle_quick_window::quick_window.show()");
            }
            Err(e) => {
                println!("ERROR:: toggle_quick_window::quick_window.show()\n{}", e);
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
