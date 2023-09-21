use crate::colored::*;
use crate::resources::update as resupdate;
use clap::Parser;
use std::error::Error;
use std::path::Path;
use updater::procedures::selfexe;
use updater::provider::{GitHubProvider, Provider};
use updater::Version;

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
    pub fn execute(&self) -> Result<(), Box<dyn Error>> {
        important("Running updater...");

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

fn update_executable() -> Result<(), Box<dyn Error>> {
    let data = selfexe::UpdateData::new(
        provider(),
        Version::parse(PKG_VERSION)?,
        "cvmodtool.exe".to_string(),
    );
    let mut procedure = selfexe::create(data);
    procedure.execute()?;

    info("Successfully updated the executable!");
    Ok(())
}

fn update_resources() -> Result<(), Box<dyn Error>> {
    let resources_dir = crate::resources::dir()?;
    let version_file = resources_dir.join(VERSION_FILE);

    let version = read_file(&version_file)?;
    let data = resupdate::UpdateData::new(
        provider(),
        "resources.zip".to_string(),
        version,
        resources_dir,
    );
    let mut procedure = resupdate::create(data);
    procedure.execute()?;

    if procedure.data().success {
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
