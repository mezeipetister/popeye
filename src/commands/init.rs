use uuid::Uuid;

use crate::{
    command::{CommandExt, UserInput},
    context::Context,
    db::Project,
    entry::LogEntry,
};

pub struct Init;

impl CommandExt for Init {
    fn name(&self) -> &'static str {
        "init"
    }

    fn procedure(
        &self,
        db: &mut Project,
        ctx: &Context,
        cmd: &UserInput,
    ) -> Result<String, String> {
        // let res = db.init()?;
        Ok(format!("Ok"))
    }
}
