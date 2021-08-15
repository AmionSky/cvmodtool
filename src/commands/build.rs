use crate::colored::*;
use crate::config::{Config, ModConfig};
use clap::Clap;
use std::error::Error;
use std::path::{Path, PathBuf};
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

/// Build/Cook the mod project
#[derive(Clap)]
pub struct Build {
    /// Mod configuration file to use
    #[clap(short, long, default_value = "cvmod.toml")]
    config: PathBuf,
}

impl Build {
    /// Mod configuration file relative path
    pub fn config(&self) -> &PathBuf {
        &self.config
    }
}

pub fn execute(opts: &Build) -> Result<(), Box<dyn Error>> {
    important("Building mod project...");

    verbose("Loading mod config...");
    let (modwd, modconfig) = crate::config::load_modconfig(&opts.config())?;
    verbose("Loading tool config...");
    let config = Config::load()?;

    info("Running Unreal Automation Tool (UAT)...");
    run_uat(&modwd, &modconfig, &config.uat())?;

    info("Success!");
    Ok(())
}

fn run_uat(modwd: &Path, modconfig: &ModConfig, uat: &Path) -> Result<(), Box<dyn Error>> {
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
