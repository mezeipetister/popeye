use uuid::Uuid;

use crate::{
    command::{CommandExt, UserInput},
    context::Context,
    db::Project,
    entry::LogEntry,
};

pub struct Create;

impl CommandExt for Create {
    fn name(&self) -> &'static str {
        "set"
    }

    fn procedure(
        &self,
        db: &mut Project,
        ctx: &Context,
        cmd: &UserInput,
    ) -> Result<String, String> {
        let params = cmd.param_str();
        let entry = LogEntry::from_user_input(&cmd, &cmd.param_str().ok_or("")?)?;
        db.add_entry(entry)?;
        Ok("Ok".to_string())
    }
}
