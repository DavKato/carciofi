use super::types::{Tab, TabHeader};
use std::fs;
use std::path::{Path, PathBuf};

fn get_file_path(app: &mut tauri::App) -> PathBuf {
    let dir = app.path_resolver().app_config_dir().unwrap();
    dir.join("tabs.json")
}

pub fn init(app: &mut tauri::App) -> Vec<Tab> {
    let file_path = get_file_path(app);

    if !file_path.exists() {
        let tabs = init_tabs();
        create_file(file_path, &tabs);
        tabs
    } else {
        restore_from_file(file_path)
    }
}

fn init_tabs() -> Vec<Tab> {
    let mut tabs = Vec::new();
    let tab = Tab::new();
    tabs.push(tab);
    tabs
}

fn create_file(path: PathBuf, data: &Vec<Tab>) {
    let dir = Path::new(&path).parent().unwrap();
    if !dir.exists() {
        fs::create_dir_all(dir).unwrap();
    }
    let data = serde_json::to_string(&data).expect("Unable to serialize JSON");
    fs::write(path, data).expect("Unable to write file");
}

pub fn restore_from_file(path: PathBuf) -> Vec<Tab> {
    let json = fs::read_to_string(path).expect("Unable to read file");
    let tabs: Vec<Tab> = serde_json::from_str(&json).expect("Unable to parse JSON");
    tabs
}

pub fn get_headers(tabs: &[Tab]) -> Vec<TabHeader> {
    tabs.iter()
        .map(|tab| TabHeader {
            id: tab.id.clone(),
            title: tab.title.clone(),
        })
        .collect()
}
