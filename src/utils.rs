use std::{
    fs::File,
    io::{Read, Result},
    path::{Path, PathBuf},
};

use dirs::config_dir;

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

pub fn find_db_path_or_default() -> PathBuf {
    todo!()
}
