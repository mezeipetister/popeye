use crate::{
    entry::LogEntry,
    item::{Item, ItemParameter, LogParameter},
};
use chrono::{DateTime, Utc};
use uuid::Uuid;

pub struct Project {
    name: String,
    items: Vec<Item>,
}

impl Project {
    pub fn new() -> Self {
        Self {
            name: "Demo project".to_string(),
            items: Vec::new(),
        }
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    fn reset(&mut self) {
        self.items = Vec::new();
    }
    pub fn reindex(&mut self) {
        self.reset();
    }
    pub fn add_entry(&mut self, entry: LogEntry) -> Result<(), String> {
        todo!()
    }
}
