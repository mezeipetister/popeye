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
        "create"
    }

    fn procedure(
        &self,
        db: &mut Project,
        ctx: &Context,
        cmd: &UserInput,
    ) -> Result<String, String> {
        let params = cmd.param_str();
        let entry = LogEntry::from_user_input(
            &cmd,
            &format!("create {}", Uuid::new_v4().as_simple().to_string()),
        )?;
        db.add_entry_public(entry, ctx)?;
        Ok(format!("Created: {}", db.items().len() - 1))
    }
}
