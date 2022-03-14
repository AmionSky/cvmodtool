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
    pub fn new(engine: PathBuf, moddir: PathBuf) -> Self {
        Self {
            engine,
            moddir,
            profiles: Profiles::new(),
        }
    }

    /// Checks if the config file exists
    pub fn check() -> bool {
        if let Ok(file) = config_path() {
            file.is_file()
        } else {
            false
        }
    }

    /// Saves the config to file
    pub fn save(&self) -> Result<(), Box<dyn Error>> {
        let content = toml::to_string_pretty(self)?;
        if let Err(e) = std::fs::write(config_path()?, content) {
            return Err(format!("Failed to save config: {}", e).into());
        }
        Ok(())
    }

    /// Loads the config from file
    pub fn load() -> Result<Self, Box<dyn Error>> {
        let content = match std::fs::read_to_string(config_path()?) {
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

fn config_path() -> Result<PathBuf, Box<dyn Error>> {
    let mut path = crate::executable_dir()?;
    path.push(REL_PATH);
    Ok(path)
}
