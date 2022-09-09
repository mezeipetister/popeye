use command::CommandExt;
use commands::*;
use db::Project;

use crate::{command::UserInput, commands::Init, context::Context};

mod command;
mod commands;
mod context;
mod db;
mod display;
mod entry;
mod item;
mod prelude;

fn process_input<T>(
    user_input: &UserInput,
    commands: Vec<Box<T>>,
    db: &mut Project,
    ctx: &Context,
) -> Result<String, String>
where
    T: CommandExt + ?Sized,
{
    if ctx.args().len() == 0 {
        return Ok("Yo :)".to_string());
    }
    for c in commands {
        if let Some(res) = c.try_call(db, ctx, user_input) {
            return res;
        }
    }

    Err("Unkown command".to_string())
}

fn force_init(ctx: &Context, user_input: &UserInput) -> Result<(), String> {
    if let Some(cmd) = user_input.cmd_str() {
        if !ctx.is_project_path() && cmd != "init" {
            return Err("Not a Yo project".to_string());
        }
    }
    Ok(())
}

fn main() -> Result<(), String> {
    // Add commands to work with
    let commands: Vec<Box<dyn CommandExt>> =
        commands![Create, Version, Init, List, Reindex, ResetDb, Set];
    // Init context
    let ctx = Context::new();
    // Get user input
    let user_input = UserInput::new(&ctx);
    // Check if Yo project
    force_init(&ctx, &user_input)?;
    // Init Project DB
    let mut db = match ctx.is_project_path() {
        true => Project::load(&ctx)?,
        false => Project::default(),
    };
    // Process user input
    let res = process_input(&user_input, commands, &mut db, &ctx)?;
    // Return result
    // TODO! Refact! Create a display result struct
    println!("{}", res);
    Ok(())
}
