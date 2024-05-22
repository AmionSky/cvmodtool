use crate::updater::Updater;
use anyhow::{anyhow, Result};
use clap::Parser;
use semver::Version;
use std::path::{Path, PathBuf};

pub const PKG_VERSION: &str = env!("CARGO_PKG_VERSION");
const VERSION_FILE: &str = "version";

/// Update the tool and its resources
#[derive(Parser)]
pub struct Update {
    /// Only update the executable
    #[arg(short, long)]
    executable: bool,

    /// Only update the resources
    #[arg(short, long)]
    resources: bool,
}

impl Update {
    pub fn setup() -> Self {
        Self {
            executable: false,
            resources: true,
        }
    }

    /// Execute command
    pub fn execute(&self) -> Result<()> {
        let mut executable = self.executable;
        let mut resources = self.resources;

        if !executable && !resources {
            executable = true;
            resources = true;
        }

        if executable {
            update_executable()?;
        }

        if resources {
            update_resources()?;
        }

        Ok(())
    }
}

fn update_executable() -> Result<()> {
    important!("Checking for executable updates");

    let updater = match Updater::new("AmionSky/cvmodtool") {
        Ok(updater) => updater,
        Err(error) => return Err(anyhow!("Failed to check for updates! ({error})")),
    };

    let current = Version::parse(PKG_VERSION)?;
    let latest = updater.version()?;

    if latest <= current {
        info!("Executable is up-to-date!");
        return Ok(());
    }

    info!("Found new version v{latest} (currently on v{current})");

    let asset_path = match updater.download("cvmodtool.exe") {
        Ok(path) => path,
        Err(error) => return Err(anyhow!("Failed to download update! ({error})")),
    };

    if let Err(error) = replace_exe(&asset_path) {
        return Err(anyhow!("Failed to replace executable! ({error})"));
    }

    info!("Executable has been updated successfully!");
    Ok(())
}

fn update_resources() -> Result<()> {
    important!("Checking for resources updates");

    let updater = match Updater::new("AmionSky/cvmodtool") {
        Ok(updater) => updater,
        Err(error) => return Err(anyhow!("Failed to check for updates! ({error})")),
    };

    let resources_dir = crate::resources::dir();
    let version_file = resources_dir.join(VERSION_FILE);

    let current = read_version(&version_file)?;
    let latest = updater.version()?;

    if latest <= current {
        info!("Resources are up-to-date!");
        return Ok(());
    }

    info!("Found new version v{latest} (currently on v{current})");

    let asset_path = match updater.download("resources.zip") {
        Ok(path) => path,
        Err(error) => return Err(anyhow!("Failed to download update! ({error})")),
    };

    if let Err(error) = replace_resources(&resources_dir, &asset_path) {
        return Err(anyhow!("Failed to replace resources! ({error})"));
    }

    if let Err(error) = std::fs::remove_file(&asset_path) {
        warning!("Failed to delete resources.zip.dltmp! ({error})");
    }

    if let Err(error) = std::fs::write(version_file, latest.to_string()) {
        return Err(anyhow!("Failed to write version file! ({error})"));
    }

    info!("Resources have been updated successfully!");
    Ok(())
}

fn read_version<P: AsRef<Path>>(version_file: P) -> Result<Version> {
    if version_file.as_ref().exists() {
        let text = std::fs::read_to_string(version_file)?;
        Ok(Version::parse(&text)?)
    } else {
        Ok(Version::new(0, 0, 0))
    }
}

fn replace_exe(new: &PathBuf) -> Result<()> {
    let current = &*crate::EXE;
    let old = current.with_extension("old");

    std::fs::rename(current, old)?;
    std::fs::rename(new, current)?;

    Ok(())
}

fn replace_resources(dir: &Path, asset: &Path) -> Result<()> {
    info!("Unpacking resources");

    // Clean resources directory
    if dir.is_dir() {
        std::fs::remove_dir_all(dir)?;
    }
    std::fs::create_dir(dir)?;

    extract(asset, dir)?;
    Ok(())
}

fn extract(asset: &Path, target: &Path) -> Result<()> {
    let file = std::fs::File::open(asset)?;
    let mut archive = zip::ZipArchive::new(file)?;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let outpath = match file.enclosed_name() {
            Some(path) => target.join(path),
            None => continue,
        };

        if file.is_dir() {
            std::fs::create_dir_all(&outpath)?;
        } else {
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    std::fs::create_dir_all(p)?;
                }
            }
            let mut outfile = std::fs::File::create(&outpath)?;
            std::io::copy(&mut file, &mut outfile)?;
        }

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;

            if let Some(mode) = file.unix_mode() {
                fs::set_permissions(&outpath, fs::Permissions::from_mode(mode))?;
            }
        }
    }

    Ok(())
}
