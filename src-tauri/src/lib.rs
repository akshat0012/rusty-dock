use std::thread;

use tauri::Window;

mod utils;
use utils::talk_to_win_api::talk_to_win_api;
use utils::hot_reload::hot_reload;

#[tauri::command]
fn init(window: Window) -> bool {
    let hot_reload_window_handle_clone = window.clone();
    let _handle = thread::spawn(move || {
        hot_reload(hot_reload_window_handle_clone, "C:/ProgramData/Widget-test/config.json")
    });

    talk_to_win_api(&window);
    true
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()

        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![init])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
