
use std::sync::Mutex;
use serde_derive::{Serialize, Deserialize};
use confy::{store, load};

pub struct GlobalSettings(pub Mutex<TsunamiConfig>);

impl Default for GlobalSettings {
    fn default() -> Self {
        return GlobalSettings(Mutex::new(load("Tsunami", "Global").unwrap()));
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TsunamiConfig {
    team_number: u32,
    alliance: u8
}

impl Default for TsunamiConfig {
    fn default() -> Self {
        TsunamiConfig {
            team_number: 8230,
            alliance: 1,
        }
    }
}

#[tauri::command]
pub fn save_settings(cfg: tauri::State<GlobalSettings>) -> () {
    let _ = store("Tsunami", "Global", cfg.0.lock().as_deref().unwrap().to_owned());
} 
