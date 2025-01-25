use std::thread;
use tauri::{ Window, AppHandle, Listener };

mod utils;
use utils::config::Config;
use utils::hot_reload::hot_reload;
use utils::read_config::read_config;
use utils::talk_to_win_api::talk_to_win_api;
use utils::update_frontend::update_frontend;

#[tauri::command]
fn init(app: AppHandle, window: Window) -> bool {
    // _FIXME 
    // Perform error handling here

    let mut current_config: Config = match read_config(None) {
        Ok(config) => {
            // println!("SUCCESS:: init()::read_config()");
            config
        }
        Err(e) => {
            println!("Err read_config\n{}", e);
            return false;
        }
    };
    

    talk_to_win_api(&window, &current_config);

    let app_clone = app.clone();
    let current_config_clone = current_config.clone();

    app.listen("frontend_ready", move |event| {
        // println!("Data -> {}", event.payload());
        if event.payload() == "\"frontend_ready\"" {
            match update_frontend(&app_clone, &current_config_clone) {
                Ok(_) => {
                    // println!("SUCCESS:: init()::update_frontend()");
                }
                Err(e) => {
                    println!("{}", e);
                }
            }
        }

    });

    let _handle = thread::spawn(move || {
        hot_reload(app, window, &mut current_config, "C:/ProgramData/rusty-dock/config.json")
    });



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
