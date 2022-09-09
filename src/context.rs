use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct Context {
    username: String,
    current_dir: PathBuf,
    is_project_path: bool,
    current_project_path: Option<PathBuf>,
}

impl Context {
    pub fn new() -> Self {
        let current_dir = std::env::current_dir().unwrap();
        let current_project_path = get_project_dir(&current_dir);
        Self {
            username: "mezeipetister".to_string(),
            current_dir: current_dir,
            is_project_path: current_project_path.is_ok(),
            current_project_path: current_project_path.map(|p| Some(p)).unwrap_or(None),
        }
    }
    pub fn username(&self) -> &str {
        &self.username
    }
    pub fn current_dir(&self) -> &PathBuf {
        &self.current_dir
    }
}

// Try to get Yo project root path
fn get_project_dir(dir: &Path) -> Result<PathBuf, String> {
    let p = dir.join(".yo");
    match p.exists() && p.is_dir() {
        true => Ok(dir.to_path_buf()),
        false => get_project_dir(
            dir.parent()
                .ok_or("Given directory is not inside a Yo Project".to_string())?,
        ),
    }
}
