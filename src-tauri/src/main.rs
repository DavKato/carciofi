#![feature(linked_list_cursors)]
// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Manager;

mod command;
mod ollama;
mod store;
mod tab;
mod utils;

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let tabs = tab::init(app);

            let main_window = app.get_window("main").unwrap();
            let url = main_window.url().to_string();

            // Redirect to the first tab
            main_window
                .eval(&format!(
                    "window.location.replace('{}')",
                    url + &tabs[0].id.to_string()
                ))
                .expect("Unable to set window location");

            app.manage(store::InternalStore::new(tabs));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            command::store::retrieve_store,
            command::tab::create_tab,
            command::tab::update_tab_title,
            command::tab::delete_tab
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
