use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use thiserror::Error;

#[derive(Debug, Serialize, Deserialize)]
pub struct ModConfig {
    /// Name of the Unreal project
    project: String,
    /// Desired name of the .pak file
    pakname: String,
    /// Files/folders to include in the package
    includes: Includes,
    /// Relative path/name of the directory to do packaging in
    #[serde(skip_serializing, default = "default_packagedir")]
    packagedir: PathBuf,
    /// Credits of the included modules
    #[serde(default)]
    credits: Vec<String>,
    /// Config working directory
    #[serde(skip)]
    wd: PathBuf,
}

#[derive(Deserialize)]
#[serde(untagged)]
enum IncludesBC {
    Simple(Vec<PathBuf>),
    Detailed {
        #[serde(default, alias = "cooked")]
        cook: Vec<PathBuf>,
        //#[serde(default, alias = "raw")]
        //copy: Vec<PathBuf>,
    },
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(from = "IncludesBC")]
pub struct Includes(Vec<PathBuf>);

impl From<IncludesBC> for Includes {
    fn from(value: IncludesBC) -> Self {
        match value {
            IncludesBC::Simple(includes) => Self(includes),
            IncludesBC::Detailed { cook } => Self(cook),
        }
    }
}

fn default_packagedir() -> PathBuf {
    PathBuf::from("Package")
}

impl ModConfig {
    pub fn new<P: AsRef<Path>>(name: &str, wd: P) -> Self {
        Self {
            pakname: format!("Z_{}_P", name),
            project: name.to_string(),
            packagedir: default_packagedir(),
            includes: Includes::default(),
            credits: vec![],
            wd: wd.as_ref().to_path_buf(),
        }
    }

    /// Load from disk
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self, ModConfigError> {
        let wd = crate::working_dir().map_err(ModConfigError::WD)?;
        let mut path = wd.join(path.as_ref());

        if !path.is_file() {
            return Err(ModConfigError::NotFound(path.display().to_string()));
        }

        let content = std::fs::read_to_string(&path).map_err(ModConfigError::Read)?;
        let mut config: Self = toml::from_str(&content).map_err(ModConfigError::Parse)?;
        config.wd = {
            path.pop();
            path
        };

        Ok(config)
    }

    /// Save to disk
    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<(), ModConfigError> {
        let contents = toml::to_string_pretty(self)?;
        std::fs::write(path, contents).map_err(ModConfigError::Write)
    }

    /// Pak file name
    pub fn pakname(&self) -> &String {
        &self.pakname
    }

    /// Pak file absolute path
    pub fn pakfile(&self) -> PathBuf {
        let mut path = self.wd().join(self.packagedir());
        path.push(format!("{}.pak", self.pakname()));
        path
    }

    /// Project name
    pub fn project(&self) -> &String {
        &self.project
    }

    /// UProject absolute path
    pub fn uproject(&self) -> PathBuf {
        self.wd().join(format!("{}.uproject", &self.project()))
    }

    /// Package directory relative path
    pub fn packagedir(&self) -> &PathBuf {
        &self.packagedir
    }

    /// Package includes
    pub fn includes(&self) -> &Vec<PathBuf> {
        &self.includes.0
    }

    /// Set package includes
    pub fn set_includes(&mut self, includes: Vec<PathBuf>) {
        self.includes = Includes(includes);
    }

    /// Set credits
    pub fn set_credits(&mut self, data: Vec<String>) {
        self.credits = data;
    }

    /// Mod working directory
    pub fn wd(&self) -> &PathBuf {
        &self.wd
    }
}

#[derive(Debug, Error)]
pub enum ModConfigError {
    #[error("Mod config file ({0}) not found!")]
    NotFound(String),
    #[error("Failed to read mod config. ({0})")]
    Read(#[source] std::io::Error),
    #[error("Failed to parse mod config. ({0})")]
    Parse(#[from] toml::de::Error),
    #[error("Failed to serialize mod config. ({0})")]
    Serialize(#[from] toml::ser::Error),
    #[error("Failed to save mod config. ({0})")]
    Write(#[source] std::io::Error),
    #[error("Failed to get the current working directory. ({0})")]
    WD(#[source] std::io::Error),
}
