use std::{fmt::Display, path::PathBuf};

use crate::{
    context::Context,
    entry::LogEntry,
    item::{Item, ItemParameter, LogParameter},
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Details {
    title: String,
    description: String,
}

impl Details {
    pub fn title(&self) -> &str {
        &self.title
    }
    pub fn description(&self) -> &str {
        &self.description
    }
}

pub struct Project {
    details: Details,
    items: Vec<Item>,
}

impl Project {
    pub fn init(ctx: &Context) -> Result<Self, String> {
        if ctx.is_project_path() {
            return Err("Path already a Yo project!".to_string());
        }
        todo!()
    }
    pub fn load(ctx: &Context) -> Result<Self, String> {
        Ok(Self {
            details: Details::default(),
            items: Vec::new(),
        })
    }
    pub fn detials(&self) -> &Details {
        &self.details
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
    fn save_details(&self) -> Result<(), String> {
        todo!()
    }
    fn save_items(&self) -> Result<(), String> {
        todo!()
    }
}
