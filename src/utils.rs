use std::{
    fs::{self, File},
    io::{Read, Result},
    path::{Path, PathBuf},
};

use dirs::config_dir;

use crate::Config;

pub fn read_file_to_string(path: &Path) -> Result<String> {
    let mut file = File::open(path)?;
    let mut string = String::new();
    file.read_to_string(&mut string)?;
    Ok(string)
}

pub fn read_toml<S>(toml_string: &str) -> Result<S>
where
    for<'de> S: serde::de::Deserialize<'de>,
{
    let new_struct: S = toml::from_str(toml_string)?;
    Ok(new_struct)
}

pub fn find_default_config() -> Option<Config> {
    if let Some(mut dir) = config_dir() {
        dir.push("config.toml");
        if dir.exists() {
            return read_config_or_none(dir);
        }
    }

    None
}

fn read_config_or_none(path: PathBuf) -> Option<Config> {
    let toml = match read_file_to_string(&path) {
        Ok(t) => t,
        Err(e) => {
            println!(
                "Cant read configuration file, using defaults. Error was {}",
                e.to_string()
            );
            return None;
        }
    };

    match read_toml(&toml) {
        Ok(cfg) => Some(cfg),
        Err(e) => {
            println!(
                "Cant read configuration file, using defaults. Error was {}",
                e.to_string()
            );
            None
        }
    }
}

pub fn init_default_data_dir() -> PathBuf {
    let data_path = "keys/db";

    let mut dir = config_dir().unwrap();
    dir.push(data_path);

    if !dir.exists() {
        fs::create_dir_all(data_path).expect("FIXME: Failed to create data dir");
    }
    dir
}
