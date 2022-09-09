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
    for c in commands {
        if let Some(res) = c.try_call(db, ctx, user_input) {
            return res;
        }
    }

    Err("Unkown command".to_string())
}

fn main() -> Result<(), String> {
    let commands: Vec<Box<dyn CommandExt>> = commands![Create, Version, Init];
    let ctx = Context::new();
    let mut db = Project::load(&ctx)?;
    let user_input = UserInput::new(&ctx);

    let res = process_input(&user_input, commands, &mut db, &ctx)?;
    println!("{}", res);
    Ok(())
}
