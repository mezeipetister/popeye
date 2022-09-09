use std::fmt::Display;

use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::{context::Context, db::Project, item::Date};

#[derive(Debug)]
pub struct UserInput {
    id: Uuid,
    date: Date,
    userid: String,
    cmd_str: Option<String>,
    param_str: Option<String>,
    params_raw: String,
}

impl UserInput {
    pub fn new(ctx: &Context) -> Self {
        let cmd_tokens = ctx.args().split_whitespace().collect::<Vec<&str>>();
        Self {
            id: Uuid::new_v4(),
            date: Date::now(),
            userid: ctx.username().to_string(),
            cmd_str: cmd_tokens.get(0).map(|cmd| cmd.to_string()),
            param_str: match cmd_tokens.len() > 1 {
                true => Some(cmd_tokens[1..].join(" ")),
                false => None,
            },
            params_raw: cmd_tokens.join(" "),
        }
    }
    pub fn id(&self) -> &Uuid {
        &self.id
    }
    pub fn date(&self) -> &Date {
        &self.date
    }
    pub fn userid(&self) -> &str {
        &self.userid
    }
    pub fn cmd_str(&self) -> Option<&str> {
        self.cmd_str.as_deref()
    }
    pub fn param_str(&self) -> Option<&str> {
        self.param_str.as_deref()
    }
    pub fn param_id(&self) -> Option<&str> {
        match self.param_str() {
            Some(p) => p
                .split_whitespace()
                .collect::<Vec<&str>>()
                .get(0)
                .map(|i| *i),
            None => None,
        }
    }
    pub fn param_list(&self) -> Vec<&str> {
        self.param_str()
            .unwrap_or("")
            .split_whitespace()
            .collect::<Vec<&str>>()
    }
    pub fn params_raw(&self) -> &str {
        &self.params_raw
    }
}

pub trait CommandExt {
    fn name(&self) -> &'static str;
    fn procedure(
        &self,
        db: &mut Project,
        ctx: &Context,
        user_input: &UserInput,
    ) -> Result<String, String>;
    fn try_call(
        &self,
        db: &mut Project,
        ctx: &Context,
        user_input: &UserInput,
    ) -> Option<Result<String, String>> {
        if self.name() == user_input.cmd_str().unwrap_or(&"".to_string()) {
            return Some(self.procedure(db, ctx, user_input));
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_command() {
        let c = UserInput::new(&Context::new());
        println!("{:?}", c);
        assert_eq!(1, 1);
    }
}
