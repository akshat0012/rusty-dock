use tauri::Window;
use std::path::Path;
use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};

use super::update_frontend::update_frontend;
use super::talk_to_win_api::talk_to_win_api;


pub fn hot_reload(window: Window, path: &str) -> notify::Result<()> {
    let config_path = Path::new(path);
    let (tx, rx) = std::sync::mpsc::channel();

    let mut watcher = RecommendedWatcher::new(tx, Config::default())?;

    watcher.watch(config_path.as_ref(), RecursiveMode::Recursive)?;

    for res in rx {
        match res {
            Ok(event) => {
                if event.kind.is_modify() == true {
                    println!("hot_reloaded now");
                    update_frontend(); // _FIXEME :: Implementation
                    talk_to_win_api(&window);
                }
            }
            Err(error) => eprintln!("Error: {error:?}"),
        }
    }

    Ok(())
}
