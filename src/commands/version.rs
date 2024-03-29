use crate::{
    command::{CommandExt, UserInput},
    context::Context,
    db::Project,
};

pub struct Version;

impl CommandExt for Version {
    fn name(&self) -> &'static str {
        "version"
    }

    fn procedure(
        &self,
        db: &mut Project,
        ctx: &Context,
        cmd: &UserInput,
    ) -> Result<String, String> {
        Ok(format!("{}", ctx.yo_version()))
    }
}
