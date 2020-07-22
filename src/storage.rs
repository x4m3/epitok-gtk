use crate::strings::{APPLICATION, ORGANIZATION, QUALIFIER};
use directories_next::ProjectDirs;
use serde_derive::Serialize;
use std::{
    error::Error,
    fs::{create_dir_all, File},
    io::Write,
    path::PathBuf,
};

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
        if let Some(mut path) = get_config_path() {
            append_filename(&mut path);
            // read the file and deserialize content
        }
    }

    pub fn save(&mut self) -> Result<(), Box<dyn Error>> {
        // Get base path
        let mut path = get_config_path().ok_or("Failed to get base configuration path")?;

        // Serialize struct to TOML
        let output = toml::to_string(&self)?;

        // Check if folder exists, otherwise create parent folders
        create_dir_all(&path)?;

        // Add filename to path
        append_filename(&mut path);

        // Create file if does not exist
        let mut file = File::create(path)?;

        // Write to file
        file.write_all(output.as_bytes())?;

        Ok(())
    }
}

fn append_filename(path: &mut PathBuf) {
    path.push(APPLICATION);
    path.set_extension("toml");
}

fn get_config_path() -> Option<PathBuf> {
    match ProjectDirs::from(QUALIFIER, ORGANIZATION, APPLICATION) {
        Some(proj_dirs) => Some(proj_dirs.config_dir().into()),
        None => None,
    }
}
