use crate::colored::*;
use crate::config::Config;
use clap::Clap;
use std::error::Error;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use walkdir::WalkDir;

/// Copy the mod's pak file into the Code Vein content directory
#[derive(Clap)]
pub struct Install {
    /// Mod configuration file to use
    #[clap(short, long, default_value = "cvmod.toml")]
    config: PathBuf,
}

impl Install {
    pub fn config(&self) -> &PathBuf {
        &self.config
    }
}

pub fn execute(opts: &Install) -> Result<(), Box<dyn Error>> {
    info("Loading mod config...");
    let (modwd, modconfig) = crate::config::load_modconfig(&opts.config())?;

    info("Loading tool config...");
    let config = Config::load()?;

    let pakfile = modconfig.pakfile(&modwd);
    let pakfilename = pakfile.file_name().ok_or("Failed to get .pak file name")?;
    let target = config.moddir().join(pakfilename);
    std::fs::copy(pakfile, target)?;

    Ok(())
}
