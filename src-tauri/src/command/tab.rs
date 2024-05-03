use crate::store::InternalStore;
use crate::tab::{Tab, TabHeader};
use tauri::State;

#[tauri::command]
pub fn create_tab(state: State<InternalStore>) -> TabHeader {
    let mut tabs = state.tabs.lock().unwrap();
    let new_tab = Tab::new();
    let header = TabHeader::from(&new_tab);
    tabs.push(new_tab);
    println!("tab created: {:?}", &tabs);
    header
}

#[tauri::command]
pub fn delete_tab(state: State<InternalStore>, id: String) -> TabHeader {
    let mut tabs = state.tabs.lock().unwrap();
    let index = tabs
        .iter()
        .position(|tab| tab.id == id)
        .unwrap_or_else(|| panic!("Tab with id {} does not exist in the InternalStore.", id));
    let removed_tab = tabs.remove(index);
    TabHeader::from(&removed_tab)
}

#[tauri::command]
pub fn update_tab_title(state: State<InternalStore>, id: String, title: String) -> TabHeader {
    let mut tabs = state.tabs.lock().unwrap();
    let tab = tabs
        .iter_mut()
        .find(|tab| tab.id == id)
        .unwrap_or_else(|| panic!("Tab with id {} does not exist in the InternalStore.", id));
    tab.title = title;
    TabHeader::from(tab)
}
