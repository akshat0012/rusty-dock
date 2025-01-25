use tauri::{ AppHandle, Window };
use std::path::Path;
use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};

use super::read_config::read_config;
use super::config::Config as MyConfig;
use super::update_frontend::update_frontend;
use super::talk_to_win_api::talk_to_win_api;

pub fn hot_reload(app: AppHandle, window: Window, current_config: &mut MyConfig, path: &str) -> notify::Result<()> {
    let config_path = Path::new(path);
    let (tx, rx) = std::sync::mpsc::channel();

    // Use MyConfig::default() instead of Config::default()
    let mut watcher = RecommendedWatcher::new(tx, Config::default())?;
    watcher.watch(config_path.as_ref(), RecursiveMode::Recursive)?;

    for res in rx {
        match res {
            Ok(event) => {
                if event.kind.is_modify() {
                    // Pass an immutable reference to the functions that need it
                    match read_config(None) {
                        Ok(new_config) => {
                            *current_config = new_config;
                        }
                        Err(e) => {
                            // send an event to frontend to check the config.json file
                            println!("ERORR:: hot_reload()::read_config()\n{}", e);
                        }
                    }

                    match update_frontend(&app, &current_config) {
                        Ok(_) => {
                            // println!("SUCCESS:: hot_reload()::update_frontend()");
                        }
                        Err(e) => {
                            println!("{}", e);
                        }
                    }
                    talk_to_win_api(&window, &current_config);
                }
            }
            Err(error) => eprintln!("Error: {error:?}"),
        }
    }

    Ok(())
}

