use std::io::Write;
use uuid::Uuid;

use crate::{
    command::{CommandExt, UserInput},
    context::Context,
    db::Project,
    entry::LogEntry,
};

pub struct Details;

impl CommandExt for Details {
    fn name(&self) -> &'static str {
        "details"
    }

    fn procedure(
        &self,
        db: &mut Project,
        ctx: &Context,
        cmd: &UserInput,
    ) -> Result<String, String> {
        let position = cmd
            .param_list()
            .get(0)
            .ok_or("Not item ID provided".to_string())?
            .parse::<usize>()
            .map_err(|_| "Item id is not a number")?;
        let item = db
            .items()
            .get(position)
            .ok_or("No item found".to_string())?;
        Ok(format!("{}", item))
    }
}
