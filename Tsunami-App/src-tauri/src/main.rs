// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod settings;
mod robot;

use std::error::Error;
use tauri::{Menu, Manager};

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

            Ok(())
        })
        .manage(robot::DriverStationState)
        .manage(settings::GlobalSettings)
        .invoke_handler(tauri::generate_handler![
         // Add tauri commands here.
            robot::disable,
            robot::enable,
            robot::estop,
            robot::restart_code,
            robot::battery_voltage,
            //robot::ds_mode,
            robot::enabled,
            robot::estopped,
            //robot::mode,
            robot::team_number,
            //robot::udp_queue,
            robot::set_team_number,
            robot::set_use_usb,
            robot::restart_roborio,
            settings::save_settings,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}

fn main() {
    try_main().unwrap(); // Panics if there is an error.
}
