use crate::utils;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

const MAX_HISTORY: usize = 20;

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub enum SessionMode {
    OneOff,
    Chat,
    File,
}

#[derive(Debug, Serialize)]
pub struct TabHeader {
    pub id: String,
    pub mode: SessionMode,
    pub title: String,
}
impl TabHeader {
    pub fn from(tab: &Tab) -> Self {
        Self {
            id: tab.id.clone(),
            mode: tab.mode,
            title: tab.title.clone(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Tab {
    pub id: String,
    pub mode: SessionMode,
    pub title: String,
    pub history: VecDeque<Content>,
    pub index: u8,
}
impl Tab {
    pub fn new() -> Self {
        let mut history = VecDeque::with_capacity(MAX_HISTORY);
        history.push_back(Content::new());
        Self {
            id: utils::get_current_time(),
            mode: SessionMode::OneOff,
            title: "untitled".to_string(),
            history,
            index: 0,
        }
    }

    pub fn get_content(&self) -> Content {
        self.history.get(self.index as usize).unwrap().clone()
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Content {
    template_id: Option<String>,
    question: String,
    answer: String,
}
impl Content {
    fn new() -> Self {
        Self {
            template_id: None,
            question: String::new(),
            answer: String::new(),
        }
    }
}
