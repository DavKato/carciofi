use crate::utils;
use serde::{de, ser::SerializeStruct};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::collections::LinkedList;
use std::fs;
use std::path::{Path, PathBuf};

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

fn get_file_path(app: &mut tauri::App) -> PathBuf {
    let dir = app.path_resolver().app_config_dir().unwrap();
    dir.join("tabs.json")
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

#[derive(Debug, Serialize, Deserialize)]
pub struct Conversation {
    template_id: Option<String>,
    question: String,
    answer: String,
}
impl Conversation {
    fn new() -> Self {
        Self {
            template_id: None,
            question: "".to_string(),
            answer: "".to_string(),
        }
    }
}

#[derive(Debug)]
pub struct Tab {
    pub id: String,
    pub title: String,
    pub history: LinkedList<Conversation>,
    pub cursor_index: usize,
}
impl Tab {
    pub fn new() -> Self {
        let mut history = LinkedList::new();
        history.push_back(Conversation::new());
        Self {
            id: utils::get_current_time(),
            title: "untitled".to_string(),
            history,
            cursor_index: 0,
        }
    }
}
impl Serialize for Tab {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Tab", 4)?;
        state.serialize_field("id", &self.id)?;
        state.serialize_field("title", &self.title)?;
        state.serialize_field("history", &self.history.iter().collect::<Vec<_>>())?;
        state.serialize_field("cursor_index", &self.cursor_index)?;
        state.end()
    }
}
impl<'de> Deserialize<'de> for Tab {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(field_identifier, rename_all = "snake_case")]
        enum Field {
            Id,
            Title,
            History,
            CursorIndex,
        }

        struct TabVisitor;
        impl<'de> de::Visitor<'de> for TabVisitor {
            type Value = Tab;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("struct Tab")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Tab, A::Error>
            where
                A: de::SeqAccess<'de>,
            {
                let id = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                let title = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(1, &self))?;
                let history: Vec<Conversation> = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(2, &self))?;
                let cursor_index = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(3, &self))?;

                Ok(Tab {
                    id,
                    title,
                    history: history.into_iter().collect::<LinkedList<Conversation>>(),
                    cursor_index,
                })
            }

            fn visit_map<A>(self, mut map: A) -> Result<Tab, A::Error>
            where
                A: de::MapAccess<'de>,
            {
                let mut id = None;
                let mut title = None;
                let mut history: Option<Vec<Conversation>> = None;
                let mut cursor_index = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Id => {
                            if id.is_some() {
                                return Err(de::Error::duplicate_field("id"));
                            }
                            id = Some(map.next_value()?);
                        }
                        Field::Title => {
                            if title.is_some() {
                                return Err(de::Error::duplicate_field("title"));
                            }
                            title = Some(map.next_value()?);
                        }
                        Field::History => {
                            if history.is_some() {
                                return Err(de::Error::duplicate_field("history"));
                            }
                            history = Some(map.next_value()?);
                        }
                        Field::CursorIndex => {
                            if cursor_index.is_some() {
                                return Err(de::Error::duplicate_field("cursor_index"));
                            }
                            cursor_index = Some(map.next_value()?);
                        }
                    }
                }
                let id = id.ok_or_else(|| de::Error::missing_field("id"))?;
                let title = title.ok_or_else(|| de::Error::missing_field("title"))?;
                let history = history.ok_or_else(|| de::Error::missing_field("history"))?;
                let cursor_index =
                    cursor_index.ok_or_else(|| de::Error::missing_field("cursor_index"))?;
                Ok(Tab {
                    id,
                    title,
                    history: history.into_iter().collect::<LinkedList<Conversation>>(),
                    cursor_index,
                })
            }
        }

        const FIELDS: &[&str] = &["id", "title", "history", "cursor_index"];
        deserializer.deserialize_struct("Tab", FIELDS, TabVisitor)
    }
}

#[derive(Serialize)]
pub struct TabHeader {
    id: String,
    title: String,
}
impl TabHeader {
    pub fn from(tab: &Tab) -> Self {
        Self {
            id: tab.id.clone(),
            title: tab.title.clone(),
        }
    }
}
