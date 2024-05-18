use crate::config::{ModConfig, ToolConfig};
use anyhow::{anyhow, Result};
use clap::Parser;
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
#[derive(Parser)]
pub struct Build {
    /// Mod configuration file to use
    #[arg(short, long, default_value = "cvmod.toml")]
    config: PathBuf,
}

impl Build {
    /// Mod configuration file relative path
    pub fn config(&self) -> &PathBuf {
        &self.config
    }

    /// Execute command
    pub fn execute(&self) -> Result<()> {
        important!("Building mod project...");

        verbose!("Loading mod config...");
        let modconfig = ModConfig::load(self.config())?;
        verbose!("Loading tool config...");
        let config = ToolConfig::load()?;

        info!("Running Unreal Automation Tool (UAT)...");
        run_uat(&modconfig, &config.uat())?;

        info!("Success!");
        Ok(())
    }
}

fn run_uat(modconfig: &ModConfig, uat: &Path) -> Result<()> {
    let mut uat_child = Command::new(uat)
        .stdin(Stdio::null())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .args(UAT_ARGS.iter())
        .arg(format!("-project=\"{}\"", modconfig.uproject().display()))
        .spawn()
        .map_err(|_| anyhow!("UAT failed to start!"))?;

    let uat_exitcode = uat_child.wait()?;
    if !uat_exitcode.success() {
        return Err(anyhow!("UAT failed with exit code {uat_exitcode}!"));
    }

    Ok(())
}
