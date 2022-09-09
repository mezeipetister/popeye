use crate::{
    command::{CommandExt, UserInput},
    db::Project,
};

pub struct Version;
const VERSION: &str = env!("CARGO_PKG_VERSION");

impl CommandExt for Version {
    fn name(&self) -> &'static str {
        "version"
    }

    fn procedure(&self, db: &mut Project, cmd: &UserInput) -> Result<String, String> {
        Ok(format!("{:?}", VERSION))
    }
}
