use crate::config::{ModConfig, ToolConfig};
use crate::resources::modules::{self, Module};
use anyhow::{anyhow, Result};
use clap::Parser;
use std::path::{Path, PathBuf};

/// Create a new Unreal Engine project for modding Code Vein.
#[derive(Parser)]
pub struct Create {
    /// Name of the project
    name: String,
    /// Creation profile to use
    #[arg(short, long, default_value = "default")]
    profile: String,
    /// Additional modules to install
    #[arg(short, long, num_args(0..))]
    modules: Option<Vec<String>>,
}

impl Create {
    /// The name of the project
    pub fn name(&self) -> &str {
        &self.name
    }

    /// The creation profile to use
    pub fn profile(&self) -> &str {
        &self.profile
    }

    /// Additional modules to install
    pub fn modules(&self) -> &Option<Vec<String>> {
        &self.modules
    }

    /// Execute command
    pub fn execute(&self) -> Result<()> {
        important!("Creating mod project...");
        let working_dir = crate::working_dir()?;

        if !name_check(self.name()) {
            return Err(anyhow!("Project name has incorrect format"));
        }

        if check_project_dir(&working_dir, self.name()) {
            return Err(anyhow!(
                "A project with the name \"{}\" already exist in the current directory!",
                self.name()
            ));
        }

        info!("Installing modules...");
        let selected_modules = match self.get_modules_to_install() {
            Ok(ret) => ret,
            Err(err) => {
                return Err(anyhow!("Failed to get modules: {err}"));
            }
        };

        let project_dir = match create_project_dir(&working_dir, self.name()) {
            Ok(ret) => ret,
            Err(err) => return Err(anyhow!("Failed to create project directory: {err}")),
        };

        if let Err(err) = install_modules(&project_dir, self.name(), &selected_modules) {
            failure_cleanup(&project_dir);
            return Err(anyhow!("Failed to install modules: {err}"));
        }

        info!("Creating modconfig & build script...");
        if let Err(err) = create_extra(&project_dir, self.name(), &selected_modules) {
            failure_cleanup(&project_dir);
            return Err(anyhow!("Failed to create modconfig/build script: {err}"));
        }

        info!("Success! Project created at {}", project_dir.display());
        Ok(())
    }

    fn get_modules_to_install(&self) -> Result<Vec<Module>> {
        let smodules = self.get_specified_modules()?;
        let lmodules = modules::load()?;

        let mut out = Vec::with_capacity(smodules.len());

        for smod in smodules {
            match lmodules.iter().find(|lmod| lmod.name() == smod) {
                Some(module) => out.push(module.clone()),
                None => return Err(anyhow!("Module not found: {}", smod)),
            }
        }

        Ok(out)
    }

    fn get_specified_modules(&self) -> Result<Vec<String>> {
        verbose!("Loading profiles...");
        let mut profiles = crate::resources::profiles::load()?;

        // Load user defined profiles
        verbose!("Loading tool config...");
        let config = ToolConfig::load()?;
        profiles.extend(config.profiles().to_owned());

        // Use .remove to take ownership
        if let Some(mut selected) = profiles.remove(self.profile()) {
            if let Some(mseleted) = self.modules() {
                selected.append(&mut mseleted.to_owned());
            }

            return Ok(selected);
        }

        Err(anyhow!("Specified profile was not found!"))
    }
}

fn failure_cleanup<P: AsRef<Path>>(project_dir: P) {
    verbose!("Cleaning up after failure...");
    if let Err(err) = std::fs::remove_dir_all(project_dir) {
        error!("Failed to clean-up after failure: {}", err);
    }
}

fn name_check(name: &str) -> bool {
    if name.is_empty() || name.chars().any(|c| c.is_whitespace()) || name == ".." {
        return false;
    }

    if name.len() > 20 {
        warning!("Project name must not be longer than 20 characters!");
        return false;
    }

    PathBuf::from(name).components().count() == 1
}

fn check_project_dir<P: AsRef<Path>>(wd: P, name: &str) -> bool {
    let project_dir = wd.as_ref().join(name);
    project_dir.exists()
}

fn create_project_dir<P: AsRef<Path>>(wd: P, name: &str) -> Result<PathBuf> {
    let project_dir = wd.as_ref().join(name);
    std::fs::create_dir(&project_dir)?;
    Ok(project_dir)
}

fn install_modules<P: AsRef<Path>>(
    project_dir: P,
    project_name: &str,
    modules: &[Module],
) -> Result<()> {
    for module in modules {
        // Install module
        module.install(&project_dir, project_name)?;

        // Warn for missing dependencies
        for dependency in module.dependencies() {
            if !modules.iter().any(|m| m.name() == dependency) {
                warning!(
                    "Missing dependency for \"{}\" module: \"{}\"",
                    module.name(),
                    dependency
                );
            }
        }
    }

    Ok(())
}

fn create_extra<P: AsRef<Path>>(
    project_dir: P,
    project_name: &str,
    modules: &[Module],
) -> Result<()> {
    const CFG_FILE: &str = "cvmod.toml";
    let modconfig = create_modconfig(project_name, &project_dir, modules)?;
    modconfig.save(project_dir.as_ref().join(CFG_FILE))?;
    create_bat(project_dir, CFG_FILE)?;
    Ok(())
}

fn create_modconfig<P: AsRef<Path>>(
    project_name: &str,
    project_dir: P,
    modules: &[Module],
) -> Result<ModConfig> {
    let mut modconfig = ModConfig::new(project_name, project_dir);

    // Get extra info from modules
    let mut pakincludes = vec![];
    let mut pakcopy = vec![];
    let mut credits = vec![];
    for module in modules {
        pakincludes.append(&mut module.pakinclude().to_owned());
        pakcopy.append(&mut module.pakcopy().to_owned());
        credits.append(&mut module.credits().to_owned());
    }
    pakincludes.sort_unstable();
    pakincludes.dedup();
    pakcopy.sort_unstable();
    pakcopy.dedup();
    credits.sort_unstable();
    credits.dedup();


    let includes = modconfig.includes_mut();
    includes.set_cook(pakincludes);
    includes.set_copy(pakcopy);
    modconfig.set_credits(credits);
    Ok(modconfig)
}

fn create_bat<P: AsRef<Path>>(project_dir: P, cfg: &str) -> Result<()> {
    const BAT_NAME: &str = "build-and-install.bat";
    let bat_target_path = project_dir.as_ref().join(BAT_NAME);
    let bat_ref_path = crate::resources::dir()?.join(BAT_NAME);

    let mut bat_contents = std::fs::read_to_string(bat_ref_path)?;
    bat_contents = bat_contents.replace("{tool}", &std::env::current_exe()?.to_string_lossy());
    bat_contents = bat_contents.replace("{config}", cfg);

    std::fs::write(bat_target_path, bat_contents)?;
    Ok(())
}
