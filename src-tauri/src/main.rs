#![feature(linked_list_cursors)]
// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Mutex;
use tauri::{Manager, State};

mod ollama;
mod tab;
mod utils;

struct InternalState {
    tabs: Mutex<Vec<tab::Tab>>,
}
impl InternalState {
    fn new(tabs: Vec<tab::Tab>) -> Self {
        Self {
            tabs: Mutex::new(tabs),
        }
    }
}
#[derive(serde::Serialize)]
struct SyncedState {
    tabs: Vec<tab::TabHeader>,
}
impl SyncedState {
    fn from(state: State<InternalState>) -> Self {
        let tab_headers = tab::get_headers(&state.tabs.lock().unwrap());
        Self { tabs: tab_headers }
    }
}

#[tauri::command]
fn retrieve_state(state: State<InternalState>) -> SyncedState {
    println!("retrieve_state called");
    SyncedState::from(state)
}

#[tauri::command]
fn create_tab(state: State<InternalState>) -> tab::TabHeader {
    let mut tabs = state.tabs.lock().unwrap();
    let new_tab = tab::Tab::new();
    let header = tab::TabHeader::from(&new_tab);
    tabs.push(new_tab);
    println!("tab created: {:?}", &tabs);
    header
}

#[tauri::command]
fn delete_tab(state: State<InternalState>, id: String) -> tab::TabHeader {
    let mut tabs = state.tabs.lock().unwrap();
    let index = tabs
        .iter()
        .position(|tab| tab.id == id)
        .unwrap_or_else(|| panic!("Tab with id {} does not exist in the InternalState.", id));
    let removed_tab = tabs.remove(index);
    tab::TabHeader::from(&removed_tab)
}

#[tauri::command]
fn update_tab_title(state: State<InternalState>, id: String, title: String) -> tab::TabHeader {
    let mut tabs = state.tabs.lock().unwrap();
    let tab = tabs
        .iter_mut()
        .find(|tab| tab.id == id)
        .unwrap_or_else(|| panic!("Tab with id {} does not exist in the InternalState.", id));
    tab.title = title;
    tab::TabHeader::from(tab)
}

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

            app.manage(InternalState::new(tabs));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            retrieve_state,
            create_tab,
            update_tab_title,
            delete_tab
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
