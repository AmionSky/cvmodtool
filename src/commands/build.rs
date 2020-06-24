use crate::colored::*;
use crate::config::{Config, ModConfig};
use clap::Clap;
use std::error::Error;
use std::path::PathBuf;
use std::process::{Command, Stdio};

const UAT_ARGS: [&str; 11] = [
    "BuildCookRun",
    "-nocompile",
    "-nocompileeditor",
    "-nodebuginfo",
    "-installed",
    "-nop4",
    "-cook",
    "-skipstage",
    "-ue4exe=UE4Editor-Cmd.exe",
    "-targetplatform=Win64",
    "-utf8output",
];

/// Build/Cook a Code Vein mod project
#[derive(Clap)]
pub struct Build {
    /// Mod configuration file to use
    #[clap(short, long, default_value = "cvmod.toml")]
    config: PathBuf,
}

impl Build {
    pub fn config(&self) -> &PathBuf {
        &self.config
    }
}

pub fn execute(opts: &Build) -> Result<(), Box<dyn Error>> {
    info("Loading mod config...");
    let (modwd, modconfig) = load_modconfig(&opts)?;

    info("Loading tool config...");
    let config = Config::load()?;

    info("Running Unreal Automation Tool (UAT)...");
    run_uat(&modwd, &modconfig, &config.uat())?;

    info("Done!");
    Ok(())
}

fn load_modconfig(opts: &Build) -> Result<(PathBuf, ModConfig), Box<dyn Error>> {
    let wd = crate::working_dir()?;
    let modconfig_path = wd.join(opts.config());
    if !modconfig_path.is_file() {
        return Err(format!("Mod config file ({}) not found!", opts.config().display()).into());
    }

    let modconfig = ModConfig::load(&modconfig_path)?;
    let modwd = {
        let mut modwd = modconfig_path;
        modwd.pop();
        modwd
    };

    Ok((modwd, modconfig))
}

fn run_uat(modwd: &PathBuf, modconfig: &ModConfig, uat: &PathBuf) -> Result<(), Box<dyn Error>> {
    let uproject = modwd.join(modconfig.uproject());
    let mut uat_child = Command::new(uat)
        .stdin(Stdio::null())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .args(UAT_ARGS.iter())
        .arg(format!("-project=\"{}\"", uproject.display()))
        .spawn()
        .expect("UAT failed to start");

    let uat_exitcode = uat_child.wait()?;
    if !uat_exitcode.success() {
        return Err("UAT failed!".into());
    }

    Ok(())
}
