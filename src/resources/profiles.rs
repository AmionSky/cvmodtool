use std::collections::HashMap;
use std::error::Error;
use std::path::PathBuf;

const REL_PATH: &str = "profiles.toml";

pub type Profiles = HashMap<String, Vec<String>>;

pub fn load() -> Result<Profiles, Box<dyn Error>> {
    let content = match std::fs::read_to_string(file()?) {
        Ok(ret) => ret,
        Err(err) => return Err(format!("Failed to read profiles: {}", err).into()),
    };
    let profiles: Profiles = match toml::from_str(&content) {
        Ok(ret) => ret,
        Err(err) => return Err(format!("Failed to parse profiles: {}", err).into()),
    };

    Ok(profiles)
}

fn file() -> Result<PathBuf, Box<dyn Error>> {
    let mut path = super::dir()?;
    path.push(REL_PATH);
    Ok(path)
}
