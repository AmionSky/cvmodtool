use crate::colored::*;
use crate::config::Config;
use clap::Clap;
use std::error::Error;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use walkdir::WalkDir;

/// Package a mod project into a .pak file
#[derive(Clap)]
pub struct Package {
    /// Mod configuration file to use
    #[clap(short, long, default_value = "cvmod.toml")]
    config: PathBuf,
}

impl Package {
    /// Mod configuration file relative path
    pub fn config(&self) -> &PathBuf {
        &self.config
    }
}

pub fn execute(opts: &Package) -> Result<(), Box<dyn Error>> {
    important("Packaging mod project...");

    verbose("Loading mod config...");
    let (modwd, modconfig) = crate::config::load_modconfig(&opts.config())?;
    verbose("Loading tool config...");
    let config = Config::load()?;

    verbose("Generating paths...");
    let packagedir = modwd.join(modconfig.packagedir());
    let pakdir = packagedir.join(modconfig.pakname());
    let pakfile = modconfig.pakfile(&modwd);
    let pak_content_dir = pakdir.join("CodeVein\\Content");
    let cooked_content_dir = modwd.join(format!(
        "Saved\\Cooked\\WindowsNoEditor\\{}\\Content",
        modconfig.project()
    ));

    if !cooked_content_dir.is_dir() {
        return Err("No cooked content was found! Make sure to build the project first.".into());
    }

    // Cleanup
    if pakdir.is_dir() {
        info("Cleaning up old files...");
        if let Err(err) = std::fs::remove_dir_all(&pakdir) {
            return Err(format!("Failed to clean-up old files: {}", err).into());
        }
    }

    if let Err(err) = std::fs::create_dir_all(&pak_content_dir) {
        return Err(format!("Failed to create package directory: {}", err).into());
    }

    info("Copying package files...");
    for entry in WalkDir::new(&cooked_content_dir) {
        if let Ok(entry) = entry {
            let absolute = entry.path();
            let relative = absolute.strip_prefix(&cooked_content_dir)?;

            if absolute.is_file() && modconfig.includes().iter().any(|i| relative.starts_with(i)) {
                verbose(&format!("  Copying file: {}", relative.display()));
                let target = pak_content_dir.join(relative);
                std::fs::create_dir_all(target.parent().ok_or("Path has no parent!")?)?;
                std::fs::copy(absolute, target)?;
            }
        } else {
            warning("Failed to access a package file!");
        }
    }

    info("Running UnrealPak...");
    run_upak(&config.upak(), &pakdir, &pakfile)?;

    info(&format!(
        "Success! Pak file created at {}",
        pakfile.display()
    ));
    Ok(())
}

fn run_upak(upak: &PathBuf, pakdir: &PathBuf, pakfile: &PathBuf) -> Result<(), Box<dyn Error>> {
    let mut child = Command::new(upak)
        .stdin(Stdio::null())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .arg(pakfile)
        .arg(format!("-Create={}", pakdir.display()))
        .arg("-compress")
        .spawn()
        .expect("UnrealPak failed to start");

    let exitcode = child.wait()?;
    if !exitcode.success() {
        return Err("UnrealPak failed!".into());
    }

    Ok(())
}
