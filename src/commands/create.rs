use crate::colored::*;
use crate::config::{Config, ModConfig};
use crate::resources::modules::{self, Module};
use std::error::Error;
use std::path::{Path, PathBuf};

/// Create a new Unreal Engine project for modding Code Vein.
#[derive(clap::Parser)]
pub struct Create {
    /// Name of the project
    name: String,
    /// Creation profile to use
    #[clap(short, long, default_value = "default")]
    profile: String,
    /// Additional modules to install
    #[clap(short, long)]
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
    pub fn execute(&self) -> Result<(), Box<dyn Error>> {
        important("Creating mod project...");
        let working_dir = crate::working_dir()?;

        if !name_check(self.name()) {
            return Err("Project name has incorrect format".into());
        }

        if check_project_dir(&working_dir, self.name()) {
            return Err(format!(
                "A project with the name \"{}\" already exist in the current directory!",
                self.name()
            )
            .into());
        }

        info("Installing modules...");
        let modules_to_install = match self.get_modules_to_install() {
            Ok(ret) => ret,
            Err(err) => {
                return Err(format!("Failed to get modules to install: {}", err).into());
            }
        };

        let project_dir = match create_project_dir(&working_dir, self.name()) {
            Ok(ret) => ret,
            Err(err) => return Err(format!("Failed to create project directory: {}", err).into()),
        };

        if let Err(err) = install_modules(&project_dir, &modules_to_install, self.name()) {
            failure_cleanup(&project_dir);
            return Err(format!("Failed to install modules: {}", err).into());
        }

        info("Creating modconfig & build script...");
        if let Err(err) = create_extra(&project_dir, self.name(), &modules_to_install) {
            failure_cleanup(&project_dir);
            return Err(format!("Failed to create modconfig/build script: {}", err).into());
        }

        info(&format!(
            "Success! Project created at {}",
            project_dir.display()
        ));
        Ok(())
    }

    fn get_modules_to_install(&self) -> Result<Vec<Module>, Box<dyn Error>> {
        let smodules = self.get_specified_modules()?;
        let lmodules = modules::load()?;

        let mut out = vec![];

        for sm in smodules {
            let mut found = false;
            for lm in &lmodules {
                if lm.name() == sm {
                    out.push(lm.clone());
                    found = true;
                    break;
                }
            }

            if !found {
                warning(&format!("Module not found: {}", sm));
            }
        }

        Ok(out)
    }

    fn get_specified_modules(&self) -> Result<Vec<String>, Box<dyn Error>> {
        verbose("Loading profiles...");
        let mut profiles = crate::resources::profiles::load()?;

        // Load user defined profiles
        verbose("Loading tool config...");
        let config = Config::load()?;
        profiles.extend(config.profiles().to_owned());

        if let Some(mut selected) = profiles.remove(self.profile()) {
            if let Some(mseleted) = self.modules() {
                selected.append(&mut mseleted.to_owned());
            }

            return Ok(selected);
        }

        Err("Specified profile was not found!".into())
    }
}

fn failure_cleanup<P: AsRef<Path>>(pd: P) {
    verbose("Cleaning up after failure...");
    if let Err(err) = std::fs::remove_dir_all(pd) {
        error(&format!("Failed to clean-up after failure: {}", err));
    }
}

fn name_check(name: &str) -> bool {
    if name.is_empty() || name.chars().any(|c| c.is_whitespace()) || name == ".." {
        return false;
    }
    PathBuf::from(name).components().count() == 1
}

fn check_project_dir<P: AsRef<Path>>(wd: P, name: &str) -> bool {
    let project_dir = wd.as_ref().join(name);
    project_dir.exists()
}

fn create_project_dir<P: AsRef<Path>>(wd: P, name: &str) -> Result<PathBuf, Box<dyn Error>> {
    let project_dir = wd.as_ref().join(name);
    std::fs::create_dir(&project_dir)?;
    Ok(project_dir)
}

fn install_modules<P: AsRef<Path>>(
    pd: P,
    modules: &[Module],
    project_name: &str,
) -> Result<(), Box<dyn Error>> {
    let mut installed = vec![];

    for module in modules {
        // Check dependencies
        for dependency in module.dependencies() {
            if !installed.contains(dependency) {
                warning(&format!(
                    "Missing dependency for \"{}\" module: \"{}\"",
                    module.name(),
                    dependency
                ));
            }
        }

        // Install module
        module.install(&pd, project_name)?;
        installed.push(module.name().to_string());
    }

    Ok(())
}

fn create_extra<P: AsRef<Path>>(
    pd: P,
    name: &str,
    modules: &[Module],
) -> Result<(), Box<dyn Error>> {
    const CFG_FILE: &str = "cvmod.toml";
    let modconfig = create_modconfig(name, modules)?;
    modconfig.save(pd.as_ref().join(CFG_FILE))?;
    create_bat(pd, CFG_FILE)?;
    Ok(())
}

fn create_modconfig(name: &str, modules: &[Module]) -> Result<ModConfig, Box<dyn Error>> {
    let mut modconfig = ModConfig::new(name);

    // Get extra info from modules
    let mut pakincludes = vec![];
    let mut credits = vec![];
    for module in modules {
        pakincludes.append(&mut module.pakinclude().to_owned());
        credits.append(&mut module.credits().to_owned())
    }
    pakincludes.sort_unstable();
    pakincludes.dedup();
    credits.sort_unstable();
    credits.dedup();

    modconfig.set_includes(pakincludes);
    modconfig.set_credits(credits);
    Ok(modconfig)
}

fn create_bat<P: AsRef<Path>>(pd: P, cfg: &str) -> Result<(), Box<dyn Error>> {
    const BAT_NAME: &str = "build-and-install.bat";
    let bat_target_path = pd.as_ref().join(BAT_NAME);
    let bat_ref_path = crate::resources::dir()?.join(BAT_NAME);

    let mut bat_contents = std::fs::read_to_string(bat_ref_path)?;
    bat_contents = bat_contents.replace("{tool}", &std::env::current_exe()?.to_string_lossy());
    bat_contents = bat_contents.replace("{config}", cfg);

    std::fs::write(bat_target_path, bat_contents)?;
    Ok(())
}
