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
        Ok(format!("{:?}", db.items()))
    }
}
