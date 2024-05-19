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
    // Newest format should always be first
    DetailedV2 {
        #[serde(default)]
        cook: Vec<PathBuf>,
        #[serde(default)]
        copy: Vec<PathBuf>,
    },
    Detailed {
        #[serde(default)]
        cooked: Vec<PathBuf>,
        #[serde(default)]
        raw: Vec<PathBuf>,
    },
    Simple(Vec<PathBuf>),
    
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(from = "IncludesBC")]
pub struct Includes {
    /// Files and folders to cook and then include
    cook: Vec<PathBuf>,
    /// Files and folders to just include as-is
    copy: Vec<PathBuf>,
}

impl Includes {
    pub fn cook(&self) -> &Vec<PathBuf> {
        &self.cook
    }

    pub fn copy(&self) -> &Vec<PathBuf> {
        &self.copy
    }

    pub fn set_cook(&mut self, value: Vec<PathBuf>) {
        self.cook = value;
    }

    pub fn set_copy(&mut self, value: Vec<PathBuf>) {
        self.copy = value;
    }
}

impl From<IncludesBC> for Includes {
    fn from(value: IncludesBC) -> Self {
        match value {
            IncludesBC::DetailedV2 { cook, copy } => Self { cook, copy },
            IncludesBC::Detailed { cooked, raw } => Self {
                cook: cooked,
                copy: raw,
            },
            IncludesBC::Simple(cook) => Self { cook, copy: vec![] },
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
    pub fn includes(&self) -> &Includes {
        &self.includes
    }

    /// Package includes (mutable)
    pub fn includes_mut(&mut self) -> &mut Includes {
        &mut self.includes
    }

    /// Credits
    #[allow(dead_code)]
    pub fn credits(&self) -> &Vec<String> {
        &self.credits
    }

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
