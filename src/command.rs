use std::fmt::Display;

use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::db::Project;

pub struct Context {
    username: String,
}

impl Context {
    pub fn new() -> Self {
        Self {
            username: "mezeipetister".to_string(),
        }
    }
    pub fn username(&self) -> &str {
        &self.username
    }
}

#[derive(Debug)]
pub struct UserInput {
    id: Uuid,
    date: DateTime<Utc>,
    userid: String,
    cmd_str: String,
    param_str: String,
}

impl UserInput {
    pub fn new(cmd_line: &str, ctx: &Context) -> Self {
        let cmd_tokens = cmd_line.split_whitespace().collect::<Vec<&str>>();
        Self {
            id: Uuid::new_v4(),
            date: Utc::now(),
            userid: ctx.username().to_string(),
            cmd_str: cmd_tokens.get(0).unwrap_or_else(|| &"").to_string(),
            param_str: cmd_tokens[1..].join(" "),
        }
    }
    pub fn id(&self) -> &Uuid {
        &self.id
    }
    pub fn date(&self) -> &DateTime<Utc> {
        &self.date
    }
    pub fn userid(&self) -> &str {
        &self.userid
    }
    pub fn cmd_str(&self) -> &str {
        &self.cmd_str
    }
    pub fn param_str(&self) -> &str {
        &self.param_str
    }
    pub fn param_id(&self) -> Option<&str> {
        self.param_str
            .split_whitespace()
            .collect::<Vec<&str>>()
            .get(0)
            .map(|i| *i)
    }
    pub fn param_list(&self) -> Vec<&str> {
        self.param_str.split_whitespace().collect::<Vec<&str>>()
    }
}

pub trait CommandExt {
    fn name() -> &'static str;
    fn procedure(db: &mut Project, user_input: UserInput) -> Result<String, String>;
    fn try_call(user_input: UserInput, db: &mut Project) -> Option<Result<String, String>> {
        if Self::name() == user_input.cmd_str() {
            return Some(Self::procedure(db, user_input));
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_command() {
        let c = UserInput::new("set 7 owner mezeipetister priority 3", &Context::new());
        println!("{:?}", c);
        assert_eq!(1, 1);
    }
}
