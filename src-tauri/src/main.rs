// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{ fmt::Display, error::Error };
use tauri::Manager;

mod commands;
mod installer;

#[derive(Debug)]
pub struct FailedInit;

impl Display for FailedInit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Failed to init the ui")
    }
}

impl Error for FailedInit {}

fn main() {
    tauri::Builder
        ::default()
        .invoke_handler(
            tauri::generate_handler![
                commands::get_available_studio,
                commands::studio_installed,
                commands::install_studio,
                commands::get_valid_clients,
                commands::get_client_manifest,
                commands::download_zip,
                commands::extract_zip,
                commands::client_installed,
                commands::prepare_client,
                commands::get_client_folder,
                commands::get_latest_version,
                commands::get_bootstrapper_info,
                commands::get_cli,
                commands::create_uri,
                commands::create_shortcuts,
                commands::launch_studio,
                commands::launch_client
            ]
        )
        .setup(|app| {
            let Some(window) = app.get_window("TurkBlox") else {
                return Err(FailedInit.into());
            };

            /* Focus and center */
            window.set_focus()?;
            window.center()?;
            window.set_always_on_top(true)?;

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
