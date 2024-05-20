use crate::tab;
use std::sync::Mutex;
use tauri::State;

#[derive(Debug)]
pub struct InternalState {
    pub tabs: Mutex<Vec<tab::Tab>>,
}
impl InternalState {
    pub fn new(tabs: Vec<tab::Tab>) -> Self {
        Self {
            tabs: Mutex::new(tabs),
        }
    }
}
#[derive(Debug, serde::Serialize)]
pub struct SyncedState {
    pub tabs: Vec<tab::TabHeader>,
}
impl SyncedState {
    pub fn from(state: &State<InternalState>) -> Self {
        let tabs = state.tabs.lock().unwrap();
        Self {
            tabs: tabs.iter().map(tab::TabHeader::from).collect(),
        }
    }
}
