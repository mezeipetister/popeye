use crate::{
    command::{CommandExt, UserInput},
    db::Project,
};

pub struct Version;
const VERSION: &str = env!("CARGO_PKG_VERSION");

impl CommandExt for Version {
    fn name() -> &'static str {
        VERSION
    }

    fn procedure(db: &mut Project, cmd: UserInput) -> Result<String, String> {
        Ok(format!("{:?}", Self::name()))
    }
}
