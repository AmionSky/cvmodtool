use crate::resources::REPLACE;
use anyhow::{anyhow, Result};
use serde::Deserialize;
use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

const REL_PATH: &str = "modules";
const CONFIG_FILE: &str = "module.toml";

pub fn load() -> Result<Vec<Module>, std::io::Error> {
    let module_dirs = std::fs::read_dir(dir()?)?;
    let mut modules = vec![];

    for entry in module_dirs.flatten() {
        let module_config_path = {
            let mut path = entry.path();
            path.push(CONFIG_FILE);
            path
        };

        if module_config_path.is_file() {
            match Module::load(module_config_path) {
                Ok(module) => modules.push(module),
                Err(err) => warning!("Failed to load module: {}", err),
            }
        }
    }

    Ok(modules)
}

#[derive(Debug, Clone, Deserialize)]
pub struct Module {
    name: String,
    #[serde(skip)]
    path: PathBuf,
    #[serde(default)]
    dependencies: Vec<String>,
    #[serde(default)]
    modifyfiles: Vec<PathBuf>,
    #[serde(default)]
    excludefiles: Vec<PathBuf>,
    #[serde(default)]
    pakinclude: Vec<PathBuf>,
    #[serde(default)]
    credits: Vec<String>,
}

impl Module {
    /// Load from disk
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self> {
        let content = match std::fs::read_to_string(&path) {
            Ok(ret) => ret,
            Err(err) => return Err(anyhow!("Failed to read module: {err}")),
        };
        let mut module: Self = match toml::from_str(&content) {
            Ok(ret) => ret,
            Err(err) => return Err(anyhow!("Failed to parse module: {err}")),
        };

        module.path = path
            .as_ref()
            .parent()
            .ok_or_else(|| anyhow!("Failed to get module directory!"))?
            .to_owned();

        Ok(module)
    }

    /// Gets the name of the module
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Gets the path to the module
    pub fn path(&self) -> &PathBuf {
        &self.path
    }

    /// Gets the dependencies of the module
    pub fn dependencies(&self) -> &Vec<String> {
        &self.dependencies
    }

    /// Gets the files that needs text replacement
    pub fn modifyfiles(&self) -> &Vec<PathBuf> {
        &self.modifyfiles
    }

    /// Gets the files/directories that should be excluded from install
    pub fn excludefiles(&self) -> &Vec<PathBuf> {
        &self.excludefiles
    }

    /// Gets the files/directories that should be included in the final .pak file
    pub fn pakinclude(&self) -> &Vec<PathBuf> {
        &self.pakinclude
    }

    /// Gets the credits of the module
    pub fn credits(&self) -> &Vec<String> {
        &self.credits
    }

    pub fn install<P: AsRef<Path>>(&self, target: P, project_name: &str) -> Result<()> {
        info!("Installing module: {}", self.name());
        let cfgfile = Some(OsStr::new(CONFIG_FILE));

        for entry in WalkDir::new(self.path()).follow_links(true) {
            let entry = entry?;
            let abs_path = entry.path();
            let rel_path = abs_path.strip_prefix(self.path())?.to_path_buf();

            // Exclude
            if !abs_path.is_file()
                || abs_path.file_name() == cfgfile
                || self
                    .excludefiles()
                    .iter()
                    .any(|ex| rel_path.starts_with(ex))
            {
                continue;
            }

            // Do Modify
            let modify = self.modifyfiles().contains(&rel_path);

            // Install file
            let mut target_path = target.as_ref().join(&rel_path);
            if modify {
                target_path = renamefile(target_path, project_name)?;
            }

            if target_path.is_file() {
                // If the file exists, merge if possible
                match target_path.extension().and_then(OsStr::to_str) {
                    Some("uproject") => {
                        verbose!("  Merging file: {}", &rel_path.display());

                        // UProject file merger
                        use json::Value;

                        let content = modifyfile(abs_path, project_name)?; // TODO: should check if modify is true?

                        let mut a: Value = json::from_str(&std::fs::read_to_string(&target_path)?)?;
                        let b: Value = json::from_str(&content)?;

                        crate::utils::json_merge(&mut a, b);

                        let writer = std::fs::File::create(&target_path)?;
                        json::to_writer_pretty(&writer, &a)?;
                        writer.sync_all()?;
                    }
                    _ => {
                        verbose!("  Replacing file: {}", &rel_path.display());

                        if modify {
                            modifycopy(abs_path, target_path, project_name)?;
                        } else {
                            std::fs::copy(abs_path, target_path)?;
                        }
                    }
                }
            } else {
                // If the file doesn't exist
                let parent = target_path
                    .parent()
                    .ok_or_else(|| anyhow!("Target file has no parent!"))?;
                if !parent.is_dir() {
                    std::fs::create_dir_all(parent)?;
                }

                verbose!("  Copying file: {}", &rel_path.display());

                if modify {
                    modifycopy(abs_path, target_path, project_name)?;
                } else {
                    std::fs::copy(abs_path, target_path)?;
                }
            }
        }

        Ok(())
    }
}

fn renamefile<P: AsRef<Path>>(file: P, replace: &str) -> Result<PathBuf> {
    let filename = file
        .as_ref()
        .file_name()
        .ok_or_else(|| anyhow!("renamefile: Unable to get filename!"))?
        .to_str()
        .ok_or_else(|| anyhow!("renamefile: Unable to convert filename!"))?;
    let rfilename = filename.replace(REPLACE, replace);
    let mut new_path = file.as_ref().to_path_buf();
    new_path.set_file_name(rfilename);
    Ok(new_path)
}

fn modifycopy<P: AsRef<Path>, Q: AsRef<Path>>(
    s: P,
    t: Q,
    replace: &str,
) -> Result<(), std::io::Error> {
    let content = modifyfile(s, replace)?;
    std::fs::write(t, content)?;
    Ok(())
}

fn modifyfile<P: AsRef<Path>>(file: P, replace: &str) -> Result<String, std::io::Error> {
    let content = std::fs::read_to_string(file.as_ref())?;
    let rcontent = content.replace(REPLACE, replace);
    Ok(rcontent)
}

fn dir() -> Result<PathBuf, std::io::Error> {
    let mut path = super::dir()?;
    path.push(REL_PATH);
    Ok(path)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_modules_load() {
        let module_count = std::fs::read_dir(dir().unwrap())
            .unwrap()
            .filter(|e| e.as_ref().unwrap().path().is_dir())
            .count();

        let modules = load().unwrap();

        assert_eq!(modules.len(), module_count);
    }
}
