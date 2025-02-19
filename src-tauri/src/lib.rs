use std::thread;
use tauri::LogicalSize;
use tauri::{ Window, AppHandle, Listener, Emitter };

mod utils;
use utils::config::Config;
use utils::hot_reload::hot_reload;
use utils::read_config::read_config;
use utils::talk_to_win_api::talk_to_win_api;
use utils::update_frontend::update_frontend;
use utils::toggle_quick_window::toggle_quick_window;


#[tauri::command]
async fn init(app: AppHandle, window: Window) -> bool {

    let mut config: Config = match read_config(None) {
        Ok(config) => {
            // println!("SUCCESS:: init()::read_config()");
            config
        }
        Err(e) => {
            println!("ERROR:: init() -> read_config\n{}", e);
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

            let app_clone = app.clone();

            app.listen("toggle_quick_menu", move |event| {
                let _ = quick_window.set_focus();
                let _ = quick_window.set_size(LogicalSize::new(400.0, 200.0));
                let _ = quick_window.set_position(tauri::LogicalPosition::new(400.0, 820.0));
                toggle_quick_window(&app_clone, &quick_window, event.payload());
            });

            let app_clone = app.clone();
            app.listen("quick_menu_data", move |event| {
                println!("Data -> {}", event.payload());
                let _ = app_clone.emit_to("quick_window", "quick_window_data", event.payload());
            });
        }
        Err(e) => {
            println!("ERROR:: tauri::WebviewWindowBuilder::new(...)\n{}", e);
        }
    };



    talk_to_win_api(&window, &config);

    let app_clone = app.clone();
    let config_clone = config.clone();

    app.listen("frontend_ready", move |event| {
        if event.payload() == "\"frontend_ready\"" {
            match update_frontend(&app_clone, false, Some(&config_clone), None) {
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
        hot_reload(app, window, &mut config, "C:/ProgramData/rusty-dock/config.json")
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
