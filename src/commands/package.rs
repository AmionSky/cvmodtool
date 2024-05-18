use crate::config::{ModConfig, ToolConfig};
use anyhow::{anyhow, Result};
use clap::Parser;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use walkdir::WalkDir;

/// Package a mod project into a .pak file
#[derive(Parser)]
pub struct Package {
    /// Mod configuration file to use
    #[arg(short, long, default_value = "cvmod.toml")]
    config: PathBuf,

    /// Don't copy the latest cooked content
    #[arg(long)]
    no_copy: bool,

    /// Don't compress the .pak file
    #[arg(long)]
    no_compress: bool,
}

impl Package {
    /// Mod configuration file relative path
    pub fn config(&self) -> &PathBuf {
        &self.config
    }

    pub fn no_copy(&self) -> bool {
        self.no_copy
    }

    pub fn no_compress(&self) -> bool {
        self.no_compress
    }

    /// Execute command
    pub fn execute(&self) -> Result<()> {
        important!("Packaging mod project...");

        verbose!("Loading mod config...");
        let modconfig = ModConfig::load(self.config())?;
        verbose!("Loading tool config...");
        let config = ToolConfig::load()?;

        verbose!("Generating paths...");
        let packagedir = modconfig.wd().join(modconfig.packagedir());
        let pakdir = packagedir.join(modconfig.pakname());
        let pakfile = modconfig.pakfile();
        let pak_content_dir = pakdir.join("CodeVein\\Content");
        let cooked_content_dir = modconfig.wd().join(format!(
            "Saved\\Cooked\\WindowsNoEditor\\{}\\Content",
            modconfig.project()
        ));
        let copy_content_dir = modconfig.wd().join("Content");

        if !self.no_copy() {
            if !cooked_content_dir.is_dir() {
                return Err(anyhow!(
                    "No cooked content was found! Make sure to build the project first."
                ));
            }

            if pakdir.is_dir() {
                info!("Cleaning up old files...");
                if let Err(err) = std::fs::remove_dir_all(&pakdir) {
                    return Err(anyhow!("Failed to clean-up old files: {}", err));
                }
            }

            if let Err(err) = std::fs::create_dir_all(&pak_content_dir) {
                return Err(anyhow!("Failed to create package directory: {}", err));
            }

            info!("Copying package files...");
            // Copy cooked content
            for entry in WalkDir::new(&cooked_content_dir) {
                if let Ok(entry) = entry {
                    let absolute = entry.path();
                    let relative = absolute.strip_prefix(&cooked_content_dir)?;

                    // Workaround for path starts_with issues
                    let str_relative = relative
                        .to_str()
                        .ok_or_else(|| anyhow!("Failed to convert path to str!"))?
                        .replace('/', "\\");

                    if absolute.is_file()
                        && modconfig.includes().cook().iter().any(|i| {
                            // Workaround for path starts_with issues
                            let str_i = i.to_str().unwrap().replace('/', "\\");
                            str_relative.starts_with(&str_i)
                        })
                    {
                        verbose!("  Copying file: {}", relative.display());
                        let target = pak_content_dir.join(relative);
                        std::fs::create_dir_all(
                            target
                                .parent()
                                .ok_or_else(|| anyhow!("Path has no parent!"))?,
                        )?;
                        std::fs::copy(absolute, target)?;
                    }
                } else {
                    warning!("Failed to access a package file!");
                }
            }
            // Copy raw content
            for entry in WalkDir::new(&copy_content_dir) {
                if let Ok(entry) = entry {
                    let absolute = entry.path();
                    let relative = absolute.strip_prefix(&copy_content_dir)?;

                    // Workaround for path starts_with issues
                    let str_relative = relative
                        .to_str()
                        .ok_or_else(|| anyhow!("Failed to convert path to str!"))?
                        .replace('/', "\\");

                    if absolute.is_file()
                        && modconfig.includes().copy().iter().any(|i| {
                            // Workaround for path starts_with issues
                            let str_i = i.to_str().unwrap().replace('/', "\\");
                            str_relative.starts_with(&str_i)
                        })
                    {
                        verbose!("  Copying file: {}", relative.display());
                        let target = pak_content_dir.join(relative);
                        std::fs::create_dir_all(
                            target
                                .parent()
                                .ok_or_else(|| anyhow!("Path has no parent!"))?,
                        )?;
                        std::fs::copy(absolute, target)?;
                    }
                } else {
                    warning!("Failed to access a package file!");
                }
            }
        }

        info!("Running UnrealPak...");
        run_upak(
            &config.upak(),
            &packagedir,
            &pakdir,
            &pakfile,
            !self.no_compress(),
        )?;

        info!("Success! Pak file created at {}", pakfile.display());
        Ok(())
    }
}

fn run_upak(
    upak: &Path,
    packagedir: &Path,
    pakdir: &Path,
    pakfile: &Path,
    compress: bool,
) -> Result<()> {
    let filelist = packagedir.join("filelist.txt");

    // Create filelist.txt
    if let Err(err) = std::fs::write(
        &filelist,
        format!("\"{}\\*.*\" \"..\\..\\..\\*.*\" ", pakdir.display()),
    ) {
        return Err(anyhow!("Failed to create filelist.txt: {err}"));
    }

    // Run UnrealPak
    let mut command = Command::new(upak);
    command
        .stdin(Stdio::null())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .arg(pakfile)
        .arg(format!("-Create={}", filelist.display()));

    if compress {
        command.arg("-compress");
    }

    let mut child = command
        .spawn()
        .map_err(|_| anyhow!("UnrealPak failed to start!"))?;

    let exitcode = child.wait()?;
    if !exitcode.success() {
        return Err(anyhow!("UnrealPak failed with exit code {exitcode}!"));
    }

    Ok(())
}
