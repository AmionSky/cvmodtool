use serde::{Deserialize, Serialize};
use std::error::Error;
use std::path::{Path, PathBuf};

#[derive(Debug, Serialize, Deserialize)]
pub struct ModConfig {
    project: String,
    pakname: String,
    includes: Vec<PathBuf>,
    #[serde(skip_serializing, default = "default_packagedir")]
    packagedir: PathBuf,
    #[serde(default)]
    credits: Vec<String>,
}

fn default_packagedir() -> PathBuf {
    PathBuf::from("Package")
}

impl ModConfig {
    pub fn new(name: &str) -> Self {
        Self {
            pakname: format!("{}_P", name),
            project: name.to_string(),
            packagedir: default_packagedir(),
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
