use std::thread;
use tauri::LogicalSize;
use tauri::{ Window, AppHandle, Listener };

mod utils;
use utils::config::Config;
use utils::hot_reload::hot_reload;
use utils::read_config::read_config;
use utils::talk_to_win_api::talk_to_win_api;
use utils::update_frontend::update_frontend;
use utils::toggle_quick_window::toggle_quick_window;


#[tauri::command]
async fn init(app: AppHandle, window: Window) -> bool {

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
    
    let quick_window;
    match tauri::WebviewWindowBuilder::new (
        &app,
        "quick_window",
        tauri::WebviewUrl::App("index.html#/quick-window".into())
    )
    .focused(false)
    .shadow(false)
    .visible(false)
    .resizable(false)
    .transparent(true)
    .maximizable(false)
    .minimizable(false)
    .decorations(false)
    .skip_taskbar(true)
    .always_on_top(true)
    .build() {
        Ok(window) => {
            quick_window = window;
            app.listen("toggle_quick_menu", move |event| {
                let _ = quick_window.set_focus();
                let _ = quick_window.set_size(LogicalSize::new(900.0, 100.0));
                let _ = quick_window.set_position(tauri::LogicalPosition::new(200.0, 200.0));
                toggle_quick_window(&quick_window, event.payload());
            });
            app.listen("quick_menu_data", move |event| {
                println!("Data -> {}", event.payload());
            });
        }
        Err(e) => {
            println!("ERROR:: tauri::WebviewWindowBuilder::new(...)\n{}", e);
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
