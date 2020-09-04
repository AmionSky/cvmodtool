use crate::colored::*;
use clap::Clap;
use std::error::Error;
use std::path::Path;
use updater::procedures::{resources, selfexe};
use updater::provider::{GitHubProvider, Provider};
use updater::Version;

pub const PKG_VERSION: &str = env!("CARGO_PKG_VERSION");
const VERSION_FILE: &str = "version";

/// Update the tool and its resources
#[derive(Clap)]
pub struct Update {
    /// Only update the executable
    #[clap(short, long)]
    executable: bool,

    /// Only update the resources
    #[clap(short, long)]
    resources: bool,
}

pub fn execute(opts: &Update) -> Result<(), Box<dyn Error>> {
    important("Installing mod package...");

    let mut executable = opts.executable;
    let mut resources = opts.resources;

    if !executable && !resources {
        executable = true;
        resources = true;
    }

    if executable {
        let data = selfexe::UpdateData::new(
            provider(),
            "cvmodtool.exe".to_string(),
            Version::parse(PKG_VERSION)?,
            std::env::current_exe()?,
        );
        let mut procedure = selfexe::create(data);
        procedure.execute()?;

        info("Successfully updated the executable!");
    }

    if resources {
        let resources_dir = crate::resources::dir()?;
        let version_file = resources_dir.join(VERSION_FILE);

        let version = read_file(&version_file)?;
        let data = resources::UpdateData::new(
            provider(),
            "resources.zip".to_string(),
            version,
            resources_dir,
        );
        let mut procedure = resources::create(data);
        procedure.execute()?;

        std::fs::write(version_file, procedure.data().version.to_string())?;

        info("Successfully updated the resources!");
    }

    Ok(())
}

fn read_file<P: AsRef<Path>>(version_file: P) -> Result<Version, Box<dyn Error>> {
    if version_file.as_ref().exists() {
        let text = std::fs::read_to_string(version_file)?;
        Ok(Version::parse(&text)?)
    } else {
        Ok(Version::new(0, 0, 0))
    }
}

fn provider() -> Box<dyn Provider> {
    Box::new(GitHubProvider::new("AmionSky/cvmodtool"))
}
