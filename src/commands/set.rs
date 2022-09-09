use std::process::id;

use uuid::Uuid;

use crate::{
    command::{CommandExt, UserInput},
    context::Context,
    db::Project,
    entry::LogEntry,
};

pub struct Set;

impl CommandExt for Set {
    fn name(&self) -> &'static str {
        "set"
    }

    fn procedure(
        &self,
        db: &mut Project,
        ctx: &Context,
        cmd: &UserInput,
    ) -> Result<String, String> {
        let params = cmd.params_raw();
        let mut params: Vec<String> = params.split_whitespace().map(|p| p.to_string()).collect();
        // Try to transpile item ID to UUID
        if let Some(id_pos_str) = params.get(1) {
            if let Ok(res) = id_pos_str.parse::<usize>() {
                let id = db
                    .get_item_id_by_pos(res)
                    .ok_or("Item with pos not found".to_string())?;
                params[1] = id.as_simple().to_string();
            }
        }
        let params = params.join(" ");
        let entry = LogEntry::from_user_input(&cmd, &params)?;
        db.add_entry_public(entry, ctx)?;
        Ok("Ok".to_string())
    }
}
