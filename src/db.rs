use std::fs::File;
use std::io::Write;
use std::ops::Deref;
use std::str::FromStr;
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

#[derive(Default)]
pub struct Project {
    project_path: PathBuf,
    details: Details,
    items: Vec<Item>,
}

impl Project {
    pub fn init(ctx: &Context) -> Result<Self, String> {
        if ctx.current_project_path().is_some() {
            return Err("Path already a Yo project!".to_string());
        }
        let p = ctx.current_dir().join(".yo");
        std::fs::create_dir_all(&p).unwrap();
        let p = Self {
            project_path: ctx.current_dir().to_owned(),
            details: Details::default(),
            items: Vec::new(),
        };
        p.save_details()?;
        p.save_items()?;

        let log_path = &ctx.current_dir().join(".yo").join("log");
        if !log_path.exists() {
            std::fs::File::create(&log_path).unwrap();
        }

        Ok(p)
    }
    pub fn load(ctx: &Context) -> Result<Self, String> {
        Ok(Self {
            project_path: ctx.current_project_path().unwrap().to_owned(),
            details: Self::load_details(&ctx)?,
            items: Self::load_items(ctx)?,
        })
    }
    pub fn detials(&self) -> &Details {
        &self.details
    }
    pub fn reset(&mut self) -> Result<(), String> {
        self.details = Details::default();
        self.items = Vec::new();
        self.save_db()?;
        Ok(())
    }
    pub fn reindex(&mut self, ctx: &Context) -> Result<(), String> {
        self.reset()?;
        let entries = self.load_entries(ctx)?;
        for e in entries {
            self.add_entry(&e, ctx)?;
        }
        Ok(())
    }
    fn add_entry(&mut self, entry: &LogEntry, ctx: &Context) -> Result<(), String> {
        let entry_kind = entry.entry_kind();
        match entry_kind {
            crate::entry::EntryKind::Create { id } => {
                let item = Item::new(
                    id.to_owned(),
                    entry.date().date_time_utc(),
                    entry.userid().to_string(),
                );
                self.items.push(item);
            }
            crate::entry::EntryKind::Set { kind, params } => {
                match kind {
                    crate::entry::SetKind::Project => (), // Todo! Implement project set methods
                    crate::entry::SetKind::Item(id) => {
                        let mut item = self
                            .items
                            .iter_mut()
                            .find(|i| i.id == *id)
                            .ok_or("Item with given ID not found".to_string())?;
                        item.set_entry(entry)?;
                    }
                }
            }
            _ => (),
        }
        self.save_db()?;
        Ok(())
    }

    pub fn add_entry_public(&mut self, entry: LogEntry, ctx: &Context) -> Result<(), String> {
        self.add_entry(&entry, ctx)?;
        Self::save_log(ctx, &entry)?;
        Ok(())
    }
    pub fn items(&self) -> &Vec<Item> {
        &self.items
    }
    fn save_details(&self) -> Result<(), String> {
        let mut file = std::fs::File::create(&self.project_path.join(".yo").join("details.yo"))
            .map_err(|_| "Error while creating project db".to_string())?;
        let encoded: Vec<u8> = bincode::serialize(&self.details).unwrap();
        file.write_all(&encoded).unwrap();
        Ok(())
    }
    fn load_details(ctx: &Context) -> Result<Details, String> {
        let content = std::fs::read(
            ctx.current_project_path()
                .unwrap()
                .join(".yo")
                .join("details.yo"),
        )
        .unwrap();
        Ok(bincode::deserialize(&content).unwrap())
    }
    fn save_items(&self) -> Result<(), String> {
        let mut file = std::fs::File::create(&self.project_path.join(".yo").join("index.yo"))
            .map_err(|_| "Error while creating index db".to_string())?;
        let encoded: Vec<u8> = bincode::serialize(&self.items).unwrap();
        file.write_all(&encoded).unwrap();
        Ok(())
    }
    fn load_items(ctx: &Context) -> Result<Vec<Item>, String> {
        let content = std::fs::read(
            ctx.current_project_path()
                .unwrap()
                .join(".yo")
                .join("index.yo"),
        )
        .unwrap();
        Ok(bincode::deserialize(&content).unwrap())
    }
    fn save_db(&self) -> Result<(), String> {
        self.save_details()?;
        self.save_items()?;
        Ok(())
    }
    fn save_log(ctx: &Context, entry: &LogEntry) -> Result<(), String> {
        let p = &ctx.current_project_path().unwrap().join(".yo").join("log");
        if !p.exists() {
            std::fs::File::create(&p).unwrap();
        }
        let mut file = std::fs::OpenOptions::new()
            .write(true)
            .append(true)
            .open(&p)
            .unwrap();

        writeln!(file, "{}", entry).map_err(|_| "Error writing log!".to_string())?;
        Ok(())
    }
    fn load_entries(&self, ctx: &Context) -> Result<Vec<LogEntry>, String> {
        let mut entries = Vec::new();
        let content =
            std::fs::read_to_string(ctx.current_project_path().unwrap().join(".yo").join("log"))
                .unwrap();
        for line in content.lines() {
            let entry = LogEntry::from_str(&line)?;
            entries.push(entry);
        }
        Ok(entries)
    }
    pub fn get_item_id_by_pos(&self, pos: usize) -> Option<Uuid> {
        self.items.get(pos).map(|i| i.id)
    }
}
