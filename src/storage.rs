use crate::strings::{APPLICATION, ORGANIZATION, QUALIFIER};
use directories_next::ProjectDirs;
use std::path::PathBuf;

#[derive(Debug)]
struct Storage {
    autologin: Option<String>,
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

pub fn load_autologin() {}

pub fn save_settings(autologin: &Option<String>) {
    let storage = Storage {
        autologin: match autologin {
            Some(autologin) => Some(autologin.to_string()),
            None => None,
        },
    };

    if let Some(path) = get_config_path() {
        println!("saving to path {:?}", path);
    }
}
