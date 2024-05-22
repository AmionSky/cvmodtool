use anyhow::{anyhow, Result};
use std::collections::HashMap;
use std::path::PathBuf;

const REL_PATH: &str = "profiles.toml";

pub type Profiles = HashMap<String, Vec<String>>;

pub fn load() -> Result<Profiles> {
    let content = match std::fs::read_to_string(file()) {
        Ok(ret) => ret,
        Err(err) => return Err(anyhow!("Failed to read profiles: {err}")),
    };
    let profiles: Profiles = match toml::from_str(&content) {
        Ok(ret) => ret,
        Err(err) => return Err(anyhow!("Failed to parse profiles: {err}")),
    };

    Ok(profiles)
}

fn file() -> PathBuf {
    let mut path = super::dir();
    path.push(REL_PATH);
    path
}
