use crate::config::{ModConfig, ModConfigError, ToolConfig, ToolConfigError};
use anyhow::{anyhow, Result};
use clap::Parser;
use std::path::{Path, PathBuf, StripPrefixError};
use std::process::{Command, Stdio};
use thiserror::Error;
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
    pub fn execute(&self) -> Result<(), PackageError> {
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
        let precooked_content_dir = modconfig.wd().join("ContentPreCooked");

        if !self.no_copy() {
            if !cooked_content_dir.is_dir() {
                return Err(PackageError::NoCookedContent);
            }

            if pakdir.is_dir() {
                info!("Cleaning up old files...");
                std::fs::remove_dir_all(&pakdir).map_err(PackageError::CleanFailed)?;
            }

            std::fs::create_dir_all(&pak_content_dir).map_err(PackageError::PkgDirCreateFailed)?;

            info!("Copying package files...");

            // Copy cooked content
            let walker = WalkDir::new(&cooked_content_dir).into_iter();
            for entry in walker.filter_map(|e| e.ok()) {
                let absolute = entry.path();
                if !absolute.is_file() {
                    continue;
                }

                let relative = absolute.strip_prefix(&cooked_content_dir)?;
                let str_relative = path_same_separator(relative); // Workaround for path starts_with issues

                if modconfig.includes().iter().any(|i| {
                    let str_i = path_same_separator(i); // Workaround for path starts_with issues
                    str_relative.starts_with(&str_i)
                }) {
                    verbose!("  Copying file: {}", relative.display());
                    let target = pak_content_dir.join(relative);
                    let parent = target.parent().ok_or(PackageError::NoParent)?;
                    std::fs::create_dir_all(parent).map_err(PackageError::ParentCreateFailed)?;
                    std::fs::copy(absolute, target).map_err(PackageError::CopyFailed)?;
                }
            }

            // Copy raw content
            let walker = WalkDir::new(&precooked_content_dir).into_iter();
            for entry in walker.filter_map(|e| e.ok()) {
                let absolute = entry.path();
                if !absolute.is_file() {
                    continue;
                }

                let relative = absolute.strip_prefix(&precooked_content_dir)?;

                verbose!("  Copying file: {}", relative.display());
                let target = pak_content_dir.join(relative);
                let parent = target.parent().ok_or(PackageError::NoParent)?;
                std::fs::create_dir_all(parent).map_err(PackageError::ParentCreateFailed)?;
                std::fs::copy(absolute, target).map_err(PackageError::CopyFailed)?;
            }
        }

        info!("Running UnrealPak...");
        run_upak(
            &config.upak(),
            &packagedir,
            &pakdir,
            &pakfile,
            !self.no_compress(),
        )
        .map_err(PackageError::UnrealPak)?;

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
        return Err(anyhow!("Exited with not successful exit code: {exitcode}!"));
    }

    Ok(())
}

fn path_same_separator<P: AsRef<Path>>(path: P) -> String {
    path.as_ref()
        .to_str()
        .expect("Path was not a valid string!")
        .replace('/', "\\")
}

#[derive(Debug, Error)]
pub enum PackageError {
    #[error("Failed to load mod config: {0}")]
    ModConfig(#[from] ModConfigError),
    #[error("Failed to load tool config: {0}")]
    ToolConfig(#[from] ToolConfigError),
    #[error("No cooked content was found! Make sure to build the project first.")]
    NoCookedContent,
    #[error("Failed to clean-up old files: {0}")]
    CleanFailed(#[source] std::io::Error),
    #[error("Failed to create package directory: {0}")]
    PkgDirCreateFailed(#[source] std::io::Error),
    #[error("Failed to get relative path ({0})")]
    PrefixStripFailed(#[from] StripPrefixError),
    #[error("File path has no parent!")]
    NoParent,
    #[error("Failed to create parent directory for file! ({0})")]
    ParentCreateFailed(#[source] std::io::Error),
    #[error("Failed to copy file! ({0})")]
    CopyFailed(#[source] std::io::Error),
    #[error("UnrealPak failed: {0}")]
    UnrealPak(#[source] anyhow::Error),
}
