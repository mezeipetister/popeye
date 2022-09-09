use std::io::Write;
use uuid::Uuid;

use crate::{
    command::{CommandExt, UserInput},
    context::Context,
    db::Project,
    entry::LogEntry,
};

pub struct List;

impl CommandExt for List {
    fn name(&self) -> &'static str {
        "ls"
    }

    fn procedure(
        &self,
        db: &mut Project,
        ctx: &Context,
        cmd: &UserInput,
    ) -> Result<String, String> {
        let mut res = Vec::new();
        for (index, item) in db.items().iter().enumerate() {
            res.push(format!("{} {}", index, item.title().unwrap_or("-")));
        }
        Ok(match res.len() > 0 {
            true => res.join("\n"),
            false => "Project is empty".to_string(),
        })
    }
}
