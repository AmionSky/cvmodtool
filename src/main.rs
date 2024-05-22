#[macro_use]
mod colored;

mod commands;
mod config;
mod resources;
mod utils;

use self::commands::{Opts, SubCommand};
use self::config::ToolConfig;
use self::utils::{EXE, EXEDIR, WORKDIR};
use anyhow::{anyhow, Error, Result};
use clap::Parser;
use std::path::PathBuf;

fn main() {
    // Parse command line args
    let opts: Opts = Opts::parse();

    // Set verbose logging
    colored::USE_VERBOSE.set(opts.verbose()).unwrap();

    // Check if tool config exist and interactively create it if it doesn't
    if !ToolConfig::check() {
        important!("Creating tool config:");
        if let Err(err) = create_tool_config() {
            error_exit(-10, "Failed to create tool config", err);
        }
    }

    #[cfg(feature = "updater")]
    if resources::dir().is_err() || !resources::dir().unwrap().is_dir() {
        important!("Downloading resources:");
        let cmd = commands::update::Update::setup();
        if let Err(err) = cmd.execute() {
            error_exit(-11, "Failed to download resources", err);
        }
    }

    match opts.subcmd() {
        SubCommand::Create(cmd) => {
            if let Err(err) = cmd.execute() {
                error_exit(-1, "Failed to create the project", err);
            }
        }
        SubCommand::Build(cmd) => {
            if let Err(err) = cmd.execute() {
                error_exit(-2, "Failed to build the project", err);
            }
        }
        SubCommand::Package(cmd) => {
            if let Err(err) = cmd.execute() {
                error_exit(-3, "Failed to package the project", err.into());
            }
        }
        SubCommand::Install(cmd) => {
            if let Err(err) = cmd.execute() {
                error_exit(-4, "Failed to install the package", err);
            }
        }
        #[cfg(feature = "updater")]
        SubCommand::Update(cmd) => {
            if let Err(err) = cmd.execute() {
                error_exit(-5, "Failed to update", err);
            }
        }
    }
}

fn error_exit(code: i32, msg: &str, err: Error) {
    error!("{}: {}", msg, err);
    std::process::exit(code);
}

fn create_tool_config() -> Result<()> {
    let mut buffer = String::new();
    info!("Path to UE 4.18:");
    verbose!("This folder should contain the \"Engine\" directory");
    verbose!(r"Example: C:\Engines\Unreal\UE_4.18");
    std::io::stdin().read_line(&mut buffer)?;
    let engine = PathBuf::from(buffer.trim_end());
    if !engine.is_dir() {
        return Err(anyhow!("Engine directory is not a valid direcoty"));
    }

    buffer.clear();
    info!("Path to Code Vein \"~mods\" folder:");
    verbose!(
        r"Example: C:\Program Files (x86)\Steam\steamapps\common\CODE VEIN\CodeVein\Content\Paks\~mods",
    );
    std::io::stdin().read_line(&mut buffer)?;
    let moddir = PathBuf::from(buffer.trim_end());
    if !moddir.is_dir() {
        return Err(anyhow!("Mods directory is not a valid direcoty"));
    }

    let config = ToolConfig::new(engine, moddir);
    config.save()?;

    Ok(())
}
