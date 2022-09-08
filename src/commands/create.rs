use crate::{
    command::{CommandExt, UserInput},
    db::Project,
};

pub struct Create;

impl CommandExt for Create {
    fn name() -> &'static str {
        "create"
    }

    fn procedure(db: &mut Project, cmd: UserInput) -> Result<String, String> {
        let params = cmd.param_str();
        Ok(format!("{:?}", Self::name()))
    }
}
