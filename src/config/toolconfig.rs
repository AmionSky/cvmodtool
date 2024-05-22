use crate::resources::profiles::Profiles;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use thiserror::Error;

const FILE_NAME: &str = "config.toml";

#[derive(Debug, Serialize, Deserialize)]
pub struct ToolConfig {
    engine: PathBuf,
    moddir: PathBuf,
    #[serde(default)]
    profiles: Profiles,
}

impl ToolConfig {
    pub fn new(engine: PathBuf, moddir: PathBuf) -> Self {
        Self {
            engine,
            moddir,
            profiles: Profiles::new(),
        }
    }

    /// Checks if the config file exists
    pub fn check() -> bool {
        config_path().is_file()
    }

    /// Saves the config to file
    pub fn save(&self) -> Result<(), ToolConfigError> {
        let content = toml::to_string_pretty(self)?;
        let path = config_path();
        std::fs::write(path, content).map_err(ToolConfigError::Write)
    }

    /// Loads the config from file
    pub fn load() -> Result<Self, ToolConfigError> {
        let path = config_path();
        let content = std::fs::read_to_string(path).map_err(ToolConfigError::Read)?;
        toml::from_str(&content).map_err(ToolConfigError::Parse)
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

fn config_path() -> PathBuf {
    crate::EXEDIR.join(FILE_NAME)
}

#[derive(Debug, Error)]
pub enum ToolConfigError {
    #[error("Failed to read tool config. ({0})")]
    Read(#[source] std::io::Error),
    #[error("Failed to parse tool config. ({0})")]
    Parse(#[from] toml::de::Error),
    #[error("Failed to serialize tool config. ({0})")]
    Serialize(#[from] toml::ser::Error),
    #[error("Failed to save tool config. ({0})")]
    Write(#[source] std::io::Error),
}
