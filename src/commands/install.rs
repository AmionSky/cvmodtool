use crate::colored::*;
use crate::config::Config;
use clap::Clap;
use std::error::Error;
use std::path::PathBuf;

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
    important("Installing mod package...");

    verbose("Loading mod config...");
    let (modwd, modconfig) = crate::config::load_modconfig(&opts.config())?;

    let pakfile = modconfig.pakfile(&modwd);
    if !pakfile.is_file() {
        return Err("Package file was not found! Make sure to package the project first.".into());
    }

    verbose("Loading tool config...");
    let config = Config::load()?;

    let pakfilename = pakfile
        .file_name()
        .ok_or("Failed to get the .pak file name")?;
    let target = config.moddir().join(pakfilename);
    std::fs::copy(pakfile, &target)?;

    info(&format!(
        "Success! Pak file installed to {}",
        target.display()
    ));
    Ok(())
}
