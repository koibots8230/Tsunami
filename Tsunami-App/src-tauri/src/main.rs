// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::error::Error;

use confy::{load, store};
use ds::{DriverStation, Alliance};
use serde_derive::{Serialize, Deserialize};
use tauri::{Menu, Manager};

static mut SETTINGS: Option<TsunamiConfig> = None;
static mut DRIVER_STATION: Option<DriverStation> = None;

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


fn try_main() -> Result<(), Box<dyn Error>> {

    unsafe { 
        SETTINGS = load("Tsunami", "Global")?;
        DRIVER_STATION = Some(DriverStation::new_team(
            SETTINGS.as_ref().unwrap().team_number,
            if SETTINGS.as_ref().unwrap().alliance / 3 == 1 {
                Alliance::new_blue(SETTINGS.as_ref().unwrap().alliance % 3)
            } else {
                Alliance::new_red(SETTINGS.as_ref().unwrap().alliance % 3)
            }));
    }

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
        .invoke_handler(tauri::generate_handler![
            // Add tauri commands here.

        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

        unsafe {
             store("Tsunami", "Global", SETTINGS.as_ref().unwrap())?;
        }

    Ok(())
}

fn main() {
    try_main().unwrap();
}
