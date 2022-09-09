use command::{CommandExt, Context};
use commands::{version::*, Create};
use db::Project;

use crate::command::UserInput;

mod command;
mod commands;
mod db;
mod display;
mod entry;
mod item;

fn process_input<T>(
    user_input: &UserInput,
    commands: Vec<Box<T>>,
    db: &mut Project,
) -> Result<String, String>
where
    T: CommandExt + ?Sized,
{
    for c in commands {
        if let Some(res) = c.try_call(user_input, db) {
            return res;
        }
    }

    Err("Unkown command".to_string())
}

fn main() -> Result<(), String> {
    let user_args = std::env::args().collect::<Vec<String>>();
    if user_args.len() == 1 {
        println!("yo");
        return Ok(());
    }
    let user_args = user_args[1..].join(" ");
    let commands: Vec<Box<dyn CommandExt>> = vec![Box::new(Create), Box::new(Version)];
    let mut db = Project::new();
    let ctx = Context::new();
    let user_input = UserInput::new(&user_args, &ctx);

    let res = process_input(&user_input, commands, &mut db)?;
    println!("{}", res);
    Ok(())
}
