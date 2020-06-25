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
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ModConfig {
    pakname: String,
    project: String,
    packagedir: PathBuf,
    includes: Vec<PathBuf>,
    credits: Vec<String>,
}

impl ModConfig {
    pub fn new(name: &str) -> Self {
        Self {
            pakname: format!("{}_P", name),
            project: name.to_string(),
            packagedir: PathBuf::from("Package"),
            includes: vec![],
            credits: vec![],
        }
    }

    /// Load from disk
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn Error>> {
        let content = match std::fs::read_to_string(path) {
            Ok(ret) => ret,
            Err(err) => return Err(format!("Failed to read mod config: {}", err).into()),
        };
        let modconfig: Self = match toml::from_str(&content) {
            Ok(ret) => ret,
            Err(err) => return Err(format!("Failed to parse mod config: {}", err).into()),
        };
        Ok(modconfig)
    }

    /// Save to disk
    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<(), Box<dyn Error>> {
        let contents = toml::to_string_pretty(self)?;
        std::fs::write(path, contents)?;
        Ok(())
    }

    /// Pak file name
    pub fn pakname(&self) -> &String {
        &self.pakname
    }

    /// Project name
    pub fn project(&self) -> &String {
        &self.project
    }

    /// UProject relative path
    pub fn uproject(&self) -> PathBuf {
        PathBuf::from(format!("{}.uproject", &self.project))
    }

    /// Package directory
    pub fn packagedir(&self) -> &PathBuf {
        &self.packagedir
    }

    /// Package includes
    pub fn includes(&self) -> &Vec<PathBuf> {
        &self.includes
    }

    /// Credits
    pub fn credits(&self) -> &Vec<String> {
        &self.credits
    }

    pub fn set_includes(&mut self, data: Vec<PathBuf>) {
        self.includes = data;
    }

    pub fn set_credits(&mut self, data: Vec<String>) {
        self.credits = data;
    }

    /// Pak file absolute path
    pub fn pakfile<P: AsRef<Path>>(&self, wd: P) -> PathBuf {
        let abs_packagedir = wd.as_ref().join(self.packagedir());
        abs_packagedir.join(format!("{}.pak", self.pakname()))
    }
}

/// Loads the mod config and returns both the config and the direcotry containing it.
pub fn load_modconfig<P: AsRef<Path>>(path: P) -> Result<(PathBuf, ModConfig), Box<dyn Error>> {
    let wd = crate::working_dir()?;
    let modconfig_path = wd.join(path.as_ref());
    if !modconfig_path.is_file() {
        return Err(format!("Mod config file ({}) not found!", path.as_ref().display()).into());
    }

    let modconfig = ModConfig::load(&modconfig_path)?;
    let modwd = {
        let mut modwd = modconfig_path;
        modwd.pop();
        modwd
    };

    Ok((modwd, modconfig))
}
