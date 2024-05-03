use crate::tab;
use std::sync::Mutex;
use tauri::State;

pub struct InternalStore {
    pub tabs: Mutex<Vec<tab::Tab>>,
}
impl InternalStore {
    pub fn new(tabs: Vec<tab::Tab>) -> Self {
        Self {
            tabs: Mutex::new(tabs),
        }
    }
}
#[derive(serde::Serialize)]
pub struct SyncedStore {
    pub tabs: Vec<tab::TabHeader>,
}
impl SyncedStore {
    pub fn from(state: State<InternalStore>) -> Self {
        let tab_headers = tab::get_headers(&state.tabs.lock().unwrap());
        Self { tabs: tab_headers }
    }
}
