use crate::colored::*;
use crate::config::ModConfig;
use crate::resources::modules::{self, Module};
use clap::Clap;
use std::error::Error;
use std::path::{Path, PathBuf};

/// Create a new Unreal Engine project for modding Code Vein.
#[derive(Clap)]
pub struct Create {
    /// Name of the project
    name: String,
    /// Profile to use
    #[clap(short, long, default_value = "default")]
    profile: String,
    /// Overwrite module includes
    #[clap(short, long)]
    modules: Option<Vec<String>>,
}

impl Create {
    pub fn new(name: String) -> Self {
        Self {
            name,
            profile: "default".to_string(),
            modules: None,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn profile(&self) -> &str {
        &self.profile
    }

    pub fn modules(&self) -> &Option<Vec<String>> {
        &self.modules
    }
}

pub fn execute(opts: &Create) -> Result<(), Box<dyn Error>> {
    important("Creating mod project...");
    let working_dir = crate::working_dir()?;

    info("Installing modules...");
    let modules_to_install = match get_modules_to_install(&opts) {
        Ok(ret) => ret,
        Err(err) => {
            return Err(format!("Failed to get modules to install: {}", err).into());
        }
    };

    let project_dir = match create_project_dir(&working_dir, opts.name()) {
        Ok(ret) => ret,
        Err(err) => return Err(format!("Failed to create project directory: {}", err).into()),
    };

    match install_modules(&project_dir, &modules_to_install, opts.name()) {
        Ok(ret) => ret,
        Err(err) => return Err(format!("Failed to install modules: {}", err).into()),
    }

    info("Creating build configuration...");
    const CFG_FILE: &str = "cvmod.toml";
    let modconfig = create_modconfig(opts.name(), &modules_to_install)?;
    modconfig.save(project_dir.join(CFG_FILE))?;
    create_bat(&project_dir, CFG_FILE)?;

    info(&format!(
        "Success! Project created at {}",
        project_dir.display()
    ));
    Ok(())
}

fn create_project_dir<P: AsRef<Path>>(wd: P, name: &str) -> Result<PathBuf, Box<dyn Error>> {
    let project_dir = wd.as_ref().join(name);
    std::fs::create_dir(&project_dir)?;
    Ok(project_dir)
}

fn get_modules_to_install(c: &Create) -> Result<Vec<Module>, Box<dyn Error>> {
    let smodules = get_specified_modules(&c)?;
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

fn get_specified_modules(c: &Create) -> Result<Vec<String>, Box<dyn Error>> {
    if c.modules().is_some() {
        return Ok(c.modules().as_ref().unwrap().clone());
    }

    let mut profiles = crate::config::load_profiles()?;
    if profiles.contains_key(c.profile()) {
        return Ok(profiles.remove(c.profile()).unwrap());
    }

    Err("Specified profile was not found!".into())
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
