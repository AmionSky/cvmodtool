use crate::colored::*;
use crate::config::Config;
use clap::Parser;
use std::error::Error;
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
    pub fn execute(&self) -> Result<(), Box<dyn Error>> {
        important("Installing mod package...");

        let pakfile = {
            if let Some(pak) = self.pak() {
                pak.to_owned()
            } else {
                verbose("Loading mod config...");
                let (modwd, modconfig) = crate::config::load_modconfig(self.config())?;
                modconfig.pakfile(modwd)
            }
        };

        if !pakfile.is_file() {
            return Err(
                "Package file was not found! Make sure to package the project first.".into(),
            );
        }

        verbose("Loading tool config...");
        let config = Config::load()?;

        let pakfilename = pakfile
            .file_name()
            .ok_or("Failed to get the .pak file name")?;
        let target = config.moddir().join(pakfilename);
        if let Err(err) = std::fs::copy(pakfile, &target) {
            return Err(format!("Failed to copy .pak file: {}", err).into());
        }

        info(&format!(
            "Success! Pak file installed to {}",
            target.display()
        ));
        Ok(())
    }
}
