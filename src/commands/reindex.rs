use uuid::Uuid;

use crate::{
    command::{CommandExt, UserInput},
    context::Context,
    db::Project,
    entry::LogEntry,
};

pub struct Reindex;

impl CommandExt for Reindex {
    fn name(&self) -> &'static str {
        "reindex"
    }

    fn procedure(
        &self,
        db: &mut Project,
        ctx: &Context,
        cmd: &UserInput,
    ) -> Result<String, String> {
        let params = cmd.param_str();
        db.reindex(ctx)?;
        Ok("Reindex done".to_string())
    }
}
