use tauri::{ AppHandle, Window };
use std::path::Path;
use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};

use super::config::Config as MyConfig;
use super::update_frontend::update_frontend;
use super::talk_to_win_api::talk_to_win_api;

pub fn hot_reload(app: AppHandle, window: Window, current_config: &mut MyConfig, path: &str) -> notify::Result<()> {
    let config_path = Path::new(path);
    let (tx, rx) = std::sync::mpsc::channel();

    // using MyConfig::default() instead of Config::default()
    let mut watcher = RecommendedWatcher::new(tx, Config::default())?;
    watcher.watch(config_path.as_ref(), RecursiveMode::Recursive)?;

    for res in rx {
        match res {
            Ok(event) => {
                if event.kind.is_modify() {
                    match update_frontend(&app, true, None, None) {
                        Ok(updated_config) => {
                            // println!("SUCCESS:: hot_reload()::update_frontend()");
                            talk_to_win_api(&window, &updated_config);
                        }
                        Err(e) => {
                            println!("{}", e);
                        }
                    }
                    
                }
            }
            Err(error) => eprintln!("Error: {error:?}"),
        }
    }

    Ok(())
}

