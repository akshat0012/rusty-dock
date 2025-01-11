use tauri::Size;
use tauri::Window;
use tauri::PhysicalSize;


#[tauri::command]
fn get_primary_monitor_info(window: Window) -> Option<(u32, u32)> {
    match window.primary_monitor() {
        Ok(Some(monitor)) => {
            let size = monitor.size();
            println!("Monitor size: {}x{}", size.width, size.height);
            return Some((size.width, size.height));
        },
        Ok(None) => {
            println!("No primary monitor found");
            return None;
        },
        Err(e) => {
            println!("Error getting monitor: {}", e);
            return None;
        }
    }
}

#[tauri::command]
fn set_window_size(window: Window, width: u32, height: u32) -> bool {
    match window.set_size(Size::Physical(PhysicalSize { width: width, height: 40 })) {
        Ok(_) => {
            println!("Successfully Set the size of the bar");
        }
        Err(e) => {
            println!("ERROR:: window.set_size() \n{}", e);
        }
    }
    println!("{}x{}", width, height);
    true
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![get_primary_monitor_info, set_window_size])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
