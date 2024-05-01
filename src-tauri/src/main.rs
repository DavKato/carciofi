#![feature(linked_list_cursors)]
// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod ollama;
mod tab;
mod utils;

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let tabs = tab::init(app);
            println!("{:?}", tabs);
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
