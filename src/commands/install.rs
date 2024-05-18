use crate::config::{ModConfig, ToolConfig};
use anyhow::{anyhow, Result};
use clap::Parser;
use std::path::PathBuf;

/// Copy the mod's pak file into the game's content directory
#[derive(Parser)]
pub struct Install {
    /// Pak file to install. Using this the config won't be used.
    pak: Option<PathBuf>,

    /// Mod configuration file to use
    #[arg(short, long, default_value = "cvmod.toml")]
    config: PathBuf,
}

impl Install {
    /// Pak file
    pub fn pak(&self) -> &Option<PathBuf> {
        &self.pak
    }

    /// Mod configuration file relative path
    pub fn config(&self) -> &PathBuf {
        &self.config
    }

    /// Execute command
    pub fn execute(&self) -> Result<()> {
        important!("Installing mod package...");

        let pakfile = {
            if let Some(pak) = self.pak() {
                pak.to_owned()
            } else {
                verbose!("Loading mod config...");
                let modconfig = ModConfig::load(self.config())?;
                modconfig.pakfile()
            }
        };

        if !pakfile.is_file() {
            return Err(anyhow!(
                "Package file was not found! Make sure to package the project first."
            ));
        }

        verbose!("Loading tool config...");
        let config = ToolConfig::load()?;

        let pakfilename = pakfile
            .file_name()
            .ok_or_else(|| anyhow!("Failed to get the .pak file name"))?;
        let target = config.moddir().join(pakfilename);
        if let Err(err) = std::fs::copy(pakfile, &target) {
            return Err(anyhow!("Failed to copy .pak file: {}", err));
        }

        info!("Success! Pak file installed to {}", target.display());
        Ok(())
    }
}
