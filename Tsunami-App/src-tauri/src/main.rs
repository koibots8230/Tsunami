// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{error::Error, sync::Mutex};

use confy::{load, store};
use ds::{DriverStation, Alliance};
use serde_derive::{Serialize, Deserialize};
use tauri::{Menu, Manager};

struct DS(Mutex<DriverStation>);
struct Settings(Mutex<TsunamiConfig>);

#[derive(Debug, Serialize, Deserialize)]
struct TsunamiConfig {
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
fn enable(ds: tauri::State<'_, DS>) -> () {
    let _ = &ds.0.lock().unwrap().enable();
}

#[tauri::command]
fn save_settings(cfg: tauri::State<Settings>) -> () {
    let _ = store("Tsunami", "Global", cfg.0.lock().as_deref().unwrap().to_owned());

    
} 

fn try_main() -> Result<(), Box<dyn Error>> {
    let menu: Menu = Menu::os_default("Tsunami");

    tauri::Builder::default()
        //.menu(menu)
        .enable_macos_default_menu(false) // Already added with Menu::os_default(). 
        .menu(menu)
        .on_window_event(|event| match event.event() {
            /* 
            tauri::WindowEvent::CloseRequested { api, .. } => todo!(),
            tauri::WindowEvent::Destroyed{} => todo!(),
            tauri::WindowEvent::FileDrop(file_dropped) => todo!(),
            tauri::WindowEvent::Focused(focused) => todo!(),
            tauri::WindowEvent::Moved(position) => todo!(),
            tauri::WindowEvent::Resized(size) => todo!(),
            tauri::WindowEvent::ThemeChanged(theme) => todo!(),
            */
            _ => {}
        })
        .on_menu_event(|event| {
            match event.menu_item_id() {
              _ => {}
            }
        })
        .setup(|app| {
            app.emit_all("Global-Settings-Loaded", "project found").unwrap();

            Ok(())
        })
        .manage(DS(Mutex::new(DriverStation::new("172.0.0.1", Alliance(1), 8230))))
        .manage(Settings(load("Tsunami", "Global")?))
        .invoke_handler(tauri::generate_handler![
            // Add tauri commands here.
            enable,
            save_settings,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}

fn main() {
    try_main().unwrap();
}
