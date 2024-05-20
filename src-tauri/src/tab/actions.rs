use super::types::Tab;
use std::path::{Path, PathBuf};
use std::{fs, io};

fn get_file_path(app: &mut tauri::App) -> PathBuf {
    let dir = app.path_resolver().app_config_dir().unwrap();
    dir.join("tabs.json")
}

pub fn init(app: &mut tauri::App) -> Vec<Tab> {
    let file_path = get_file_path(app);

    match restore_from_file(&file_path) {
        Ok(tabs) => tabs,
        Err(e) => {
            println!("Unable to restore from file: {}", e);
            println!("Creating new file at: {}", file_path.to_string_lossy());
            let tabs = init_tabs();
            create_file(&file_path, &tabs);
            tabs
        }
    }
}

fn init_tabs() -> Vec<Tab> {
    let mut tabs = Vec::new();
    let tab = Tab::new();
    tabs.push(tab);
    tabs
}

fn create_file(path: &PathBuf, data: &Vec<Tab>) {
    let dir = Path::new(&path).parent().unwrap();
    if !dir.exists() {
        fs::create_dir_all(dir).unwrap();
    }
    let data = serde_json::to_string(&data).expect("Unable to serialize JSON");
    fs::write(path, data).expect("Unable to write file");
}

fn restore_from_file(path: &PathBuf) -> Result<Vec<Tab>, io::Error> {
    let json = fs::read_to_string(path)?;
    let tabs: Vec<Tab> = serde_json::from_str(&json)?;
    Ok(tabs)
}
