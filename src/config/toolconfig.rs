use crate::resources::profiles::Profiles;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::path::PathBuf;

const REL_PATH: &str = "config.toml";

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    engine: PathBuf,
    moddir: PathBuf,
    #[serde(default)]
    profiles: Profiles,
}

impl Config {
    pub fn load() -> Result<Self, Box<dyn Error>> {
        let config_file = {
            let mut path = crate::executable_dir()?;
            path.push(REL_PATH);
            path
        };

        let content = match std::fs::read_to_string(config_file) {
            Ok(ret) => ret,
            Err(err) => return Err(format!("Failed to read config: {}", err).into()),
        };
        let config: Self = match toml::from_str(&content) {
            Ok(ret) => ret,
            Err(err) => return Err(format!("Failed to parse config: {}", err).into()),
        };

        Ok(config)
    }

    pub fn uat(&self) -> PathBuf {
        self.engine.join("Engine\\Build\\BatchFiles\\RunUAT.bat")
    }

    pub fn upak(&self) -> PathBuf {
        self.engine.join("Engine\\Binaries\\Win64\\UnrealPak.exe")
    }

    pub fn moddir(&self) -> &PathBuf {
        &self.moddir
    }

    pub fn profiles(&self) -> &Profiles {
        &self.profiles
    }
}
