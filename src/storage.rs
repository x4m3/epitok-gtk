use crate::strings::{APPLICATION, ORGANIZATION, QUALIFIER};
use directories_next::ProjectDirs;
use serde_derive::Serialize;
use std::path::PathBuf;

#[derive(Serialize)]
pub struct Storage {
    pub online_status: bool,
    pub autologin: Option<String>,
}

impl Storage {
    pub fn new() -> Self {
        Self {
            online_status: false,
            autologin: None,
        }
    }

    pub fn load(&mut self) {
        if let Some(path) = get_config_path() {
            println!("loading from path {:?}", path);
            // read the file and deserialize content
        }
    }

    pub fn save(&mut self) {
        if let Some(path) = get_config_path() {
            println!("saving to path {:?}", path);
            let _output = match toml::to_string(&self) {
                Ok(output) => output,
                Err(e) => {
                    eprintln!("error: {}", e);
                    return;
                }
            };
            // write to file
        }
    }
}

fn get_config_path() -> Option<PathBuf> {
    match ProjectDirs::from(QUALIFIER, ORGANIZATION, APPLICATION) {
        Some(proj_dirs) => {
            // Folder
            let mut path: PathBuf = proj_dirs.config_dir().into();

            // File
            path.push(APPLICATION);
            path.set_extension("toml");

            Some(path)
        }
        None => None,
    }
}
