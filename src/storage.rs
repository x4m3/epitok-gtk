use crate::strings::{APPLICATION, ORGANIZATION, QUALIFIER};
use directories_next::ProjectDirs;
use serde_derive::Serialize;
use std::{
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
            println!("loading from path {:?}", path);
            println!("does {:?} exist: {}", path, path.exists());
            // read the file and deserialize content
        }
    }

    pub fn save(&mut self) {
        if let Some(mut path) = get_config_path() {
            // Create output to write
            let output = match toml::to_string(&self) {
                Ok(output) => output,
                Err(e) => {
                    eprintln!("error when converting to TOML: {}", e);
                    return;
                }
            };

            println!("saving to path {:?}", path);

            // Check if folder exists, otherwise create parent folders
            match create_dir_all(&path) {
                Ok(()) => (),
                Err(e) => {
                    eprintln!("error when creating folder: {}", e);
                    return;
                }
            }

            // Add filename to path
            append_filename(&mut path);

            // Create file if does not exist
            let mut file = match File::create(path) {
                Ok(file) => file,
                Err(e) => {
                    eprintln!("error when creating file: {}", e);
                    return;
                }
            };

            // Write to file
            match file.write_all(output.as_bytes()) {
                Ok(()) => (),
                Err(e) => {
                    eprintln!("error when writing to file: {}", e);
                    return;
                }
            }
        }
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
