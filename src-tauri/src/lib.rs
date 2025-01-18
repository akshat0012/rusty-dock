mod utils;
use utils::talk_to_win_api::talk_to_win_api;

// #[tauri::command]
// fn init() {
//     talk_to_win_api();
//        -> hot_reload();
//        -> read_config();
//     clean();
// }

// _DOING
// Implementing hot_reload(); -> [ 17/1/25 ]

// _TODO's
// Error Handling in Config return from the config.rs 
//  -> Set the Default Values for the config files
//




#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()

        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![talk_to_win_api])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
