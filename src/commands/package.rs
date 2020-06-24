use crate::colored::*;
use crate::config::Config;
use clap::Clap;
use std::error::Error;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use walkdir::WalkDir;

/// Package a Code Vein mod project into a .pak file
#[derive(Clap)]
pub struct Package {
    /// Mod configuration file to use
    #[clap(short, long, default_value = "cvmod.toml")]
    config: PathBuf,
}

impl Package {
    pub fn config(&self) -> &PathBuf {
        &self.config
    }
}

pub fn execute(opts: &Package) -> Result<(), Box<dyn Error>> {
    info("Loading mod config...");
    let (modwd, modconfig) = crate::config::load_modconfig(&opts.config())?;

    info("Loading tool config...");
    let config = Config::load()?;

    let packagedir = modwd.join(modconfig.packagedir());
    let pakdir = packagedir.join(modconfig.pakname());
    let pakfile = packagedir.join(format!("{}.pak", modconfig.pakname()));
    let pak_content_dir = pakdir.join("CodeVein\\Content");
    let cooked_content_dir = modwd.join(format!(
        "Saved\\Cooked\\WindowsNoEditor\\{}\\Content",
        modconfig.project()
    ));

    if !cooked_content_dir.is_dir() {
        return Err("No cooked content was found! Make sure to build the project first.".into());
    }

    info("Copying package files...");
    if !packagedir.is_dir() {
        std::fs::create_dir(&packagedir)?;
    }

    // Cleanup
    if pakdir.is_dir() {
        std::fs::remove_dir_all(&pakdir)?;
    }
    std::fs::create_dir_all(&pak_content_dir)?;

    // Copy necessary files
    for entry in WalkDir::new(&cooked_content_dir) {
        let entry = entry?;
        let absolute = entry.path();
        let relative = absolute.strip_prefix(&cooked_content_dir)?;

        if absolute.is_file() && modconfig.includes().iter().any(|i| relative.starts_with(i)) {
            let target = pak_content_dir.join(relative);
            std::fs::create_dir_all(target.parent().ok_or("Path has no parent!")?)?;
            std::fs::copy(absolute, target)?;
        }
    }

    info("Packaging...");
    run_upak(&config.upak(), &pakdir, &pakfile)?;

    info("Done!");
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
