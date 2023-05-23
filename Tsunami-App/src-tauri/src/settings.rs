
use std::sync::Mutex;
use serde_derive::{Serialize, Deserialize};
use confy::{store, load};

pub struct GlobalSettings(pub Mutex<TsunamiGlobalConfig>);

impl Default for GlobalSettings {
    fn default() -> Self {
        return GlobalSettings(Mutex::new(load("Tsunami", "Global").unwrap_or_default()));
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TsunamiGlobalConfig {
    team_number: u32,
    alliance: u8
}

impl Default for TsunamiGlobalConfig {
    fn default() -> Self {
        TsunamiGlobalConfig {
            team_number: 8230,
            alliance: 1,
        }
    }
}

#[tauri::command]
pub fn save_settings(cfg: tauri::State<GlobalSettings>) -> () {
    let _ = store("Tsunami", "Global", cfg.0.lock().as_deref().unwrap().to_owned());
} 

#[tauri::command]
pub fn get_global_team_number(cfg: tauri::State<GlobalSettings>) -> u32 {
    return cfg.0.lock().as_deref().unwrap().to_owned().team_number;
}


/* 
 tauri.conf.json for settings window 

       {
        "label": "settings",
        "fullscreen": false,
        "center": true,
        "resizable": false,
        "title": "Tsunami: Settings",
        "focus": true,
        "decorations": false,
        "titleBarStyle": "Transparent"
      }

 */
