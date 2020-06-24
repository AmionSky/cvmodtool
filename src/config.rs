use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::path::{Path, PathBuf};

const PROFILES_REL_PATH: &str = "resources\\profiles.toml";
const CONFIG_REL_PATH: &str = "resources\\config.toml";

type Profiles = HashMap<String, Vec<String>>;

pub fn load_profiles() -> Result<Profiles, Box<dyn Error>> {
    let executable_dir = crate::executable_dir()?;
    let profiles_file = executable_dir.join(PROFILES_REL_PATH);
    let profiles: Profiles = toml::from_str(&std::fs::read_to_string(profiles_file)?)?;
    Ok(profiles)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    engine: PathBuf,
    moddir: PathBuf,
}

impl Config {
    pub fn load() -> Result<Self, Box<dyn Error>> {
        let executable_dir = crate::executable_dir()?;
        let config_file = executable_dir.join(CONFIG_REL_PATH);
        let config: Self = toml::from_str(&std::fs::read_to_string(config_file)?)?;
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
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ModConfig {
    pakname: String,
    uproject: PathBuf,
}

impl ModConfig {
    pub fn new(name: &str) -> Self {
        Self {
            pakname: format!("{}_P", name),
            uproject: PathBuf::from(format!("{}.uproject", name)),
        }
    }

    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn Error>> {
        let modconfig: Self = toml::from_str(&std::fs::read_to_string(path)?)?;
        Ok(modconfig)
    }

    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<(), Box<dyn Error>> {
        let contents = toml::to_string_pretty(self)?;
        std::fs::write(path, contents)?;
        Ok(())
    }

    pub fn uproject(&self) -> &PathBuf {
        &self.uproject
    }
}
