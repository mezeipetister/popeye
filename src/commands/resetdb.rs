use uuid::Uuid;

use crate::{
    command::{CommandExt, UserInput},
    context::Context,
    db::Project,
    entry::LogEntry,
};

pub struct ResetDb;

impl CommandExt for ResetDb {
    fn name(&self) -> &'static str {
        "resetdb"
    }

    fn procedure(
        &self,
        db: &mut Project,
        ctx: &Context,
        cmd: &UserInput,
    ) -> Result<String, String> {
        let params = cmd.param_str();
        db.reset()?;
        Ok("Database reseted".to_string())
    }
}
