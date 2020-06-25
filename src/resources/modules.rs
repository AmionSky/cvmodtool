use crate::colored::*;
use crate::resources::REPLACE;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

const REL_PATH: &str = "resources\\modules";
const CONFIG_FILE: &str = "module.toml";

#[derive(Debug, Clone)]
pub struct Module {
    name: String,
    path: PathBuf,
    dependencies: Vec<String>,
    modifyfiles: Vec<PathBuf>,
    excludefiles: Vec<PathBuf>,
    pakinclude: Vec<PathBuf>,
    credits: Vec<String>,
}

impl Module {
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

    pub fn install<P: AsRef<Path>>(
        &self,
        target: P,
        project_name: &str,
    ) -> Result<(), Box<dyn Error>> {
        info(&format!("Installing module: {}", self.name()));
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
                target_path = renamefile(target_path, &project_name)?;
            }

            if target_path.is_file() {
                // If the file exists, merge if possible
                match target_path.extension().and_then(OsStr::to_str) {
                    Some("uproject") => {
                        verbose(&format!("  Merging file: {}", &rel_path.display()));

                        // UProject file merger
                        use json::Value;

                        let content = modifyfile(abs_path, &project_name)?;

                        let mut a: Value = json::from_str(&std::fs::read_to_string(&target_path)?)?;
                        let b: Value = json::from_str(&content)?;

                        crate::utils::json_merge(&mut a, b);

                        let writer = std::fs::File::create(&target_path)?;
                        json::to_writer_pretty(&writer, &a)?;
                        writer.sync_all()?;
                    }
                    _ => {
                        verbose(&format!("  Replacing file: {}", &rel_path.display()));

                        if modify {
                            modifycopy(abs_path, target_path, &project_name)?;
                        } else {
                            std::fs::copy(abs_path, target_path)?;
                        }
                    }
                }
            } else {
                // If the file doesn't exist
                let parent = target_path.parent().ok_or("Target file has no parent!")?;
                if !parent.is_dir() {
                    std::fs::create_dir_all(parent)?;
                }

                verbose(&format!("  Copying file: {}", &rel_path.display()));

                if modify {
                    modifycopy(abs_path, target_path, &project_name)?;
                } else {
                    std::fs::copy(abs_path, target_path)?;
                }
            }
        }

        Ok(())
    }
}

fn renamefile<P: AsRef<Path>>(file: P, replace: &str) -> Result<PathBuf, Box<dyn Error>> {
    let filename = file
        .as_ref()
        .file_name()
        .ok_or("renamefile: Unable to get filename!")?
        .to_str()
        .ok_or("renamefile: Unable to convert filename!")?;
    let rfilename = filename.replace(REPLACE, replace);
    let mut new_path = file.as_ref().to_path_buf();
    new_path.set_file_name(rfilename);
    Ok(new_path)
}

fn modifycopy<P: AsRef<Path>, Q: AsRef<Path>>(
    s: P,
    t: Q,
    replace: &str,
) -> Result<(), Box<dyn Error>> {
    let content = modifyfile(s, replace)?;
    std::fs::write(t, content)?;
    Ok(())
}

fn modifyfile<P: AsRef<Path>>(file: P, replace: &str) -> Result<String, Box<dyn Error>> {
    let content = std::fs::read_to_string(file.as_ref())?;
    let rcontent = content.replace(REPLACE, replace);
    Ok(rcontent)
}

pub fn load() -> Result<Vec<Module>, Box<dyn Error>> {
    let executable_dir = crate::executable_dir()?;
    let modules_path = executable_dir.join(REL_PATH);
    let mod_dirs = std::fs::read_dir(modules_path)?;

    let mut modules = vec![];

    for mod_dir_entry in mod_dirs {
        if let Ok(mod_dir_entry) = mod_dir_entry {
            let mod_dir = mod_dir_entry.path();
            if mod_dir.is_dir() {
                if let Ok(module) = load_module(mod_dir) {
                    modules.push(module);
                }
            }
        }
    }

    Ok(modules)
}

#[derive(Serialize, Deserialize, Debug)]
struct ModuleConfig {
    name: String,
    dependencies: Option<Vec<String>>,
    modifyfiles: Option<Vec<PathBuf>>,
    excludefiles: Option<Vec<PathBuf>>,
    pakinclude: Option<Vec<PathBuf>>,
    credits: Option<Vec<String>>,
}

fn load_module(path: PathBuf) -> Result<Module, Box<dyn Error>> {
    let config_path = path.join(CONFIG_FILE);
    let config: ModuleConfig = toml::from_str(&std::fs::read_to_string(config_path)?)?;

    Ok(Module {
        name: config.name,
        path,
        dependencies: config.dependencies.unwrap_or_default(),
        modifyfiles: config.modifyfiles.unwrap_or_default(),
        excludefiles: config.excludefiles.unwrap_or_default(),
        pakinclude: config.pakinclude.unwrap_or_default(),
        credits: config.credits.unwrap_or_default(),
    })
}
