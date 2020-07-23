use crate::strings::{APPLICATION, ORGANIZATION, QUALIFIER};
use directories_next::ProjectDirs;
use serde_derive::{Deserialize, Serialize};
use std::{
    error::Error,
    fs::{create_dir_all, File},
    io::{Read, Write},
    path::PathBuf,
};

#[derive(Serialize, Deserialize)]
pub struct Storage {
    pub autologin: Option<String>,
}

impl Storage {
    pub fn new() -> Self {
        Self { autologin: None }
    }

    pub fn load() -> Result<Self, Box<dyn Error>> {
        let mut path = get_config_path().ok_or("Failed to get base configuration path")?;

        // Add filename to path
        append_filename(&mut path);

        // Read config file into string
        let mut file = File::open(path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;

        // Deserialize into struct
        let new = toml::from_str(&content)?;

        Ok(new)
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
