#![allow(dead_code)]

mod colored;
mod commands;
mod config;
mod resources;
mod utils;

use clap::Parser;
use colored::*;
use commands::{Opts, SubCommand};
use config::Config;
use std::error::Error;
use std::path::PathBuf;

fn main() {
    if !Config::check() {
        if let Err(err) = create_tool_config() {
            error_exit(-10, "Failed to create tool config", err);
        }
    }

    let opts: Opts = Opts::parse();

    // Set verbose logging
    unsafe {
        USE_VERBOSE = opts.verbose();
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
                error_exit(-3, "Failed to package the project", err);
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

fn error_exit(code: i32, msg: &str, err: Box<dyn Error>) {
    error(&format!("{}: {}", msg, err));
    std::process::exit(code);
}

pub fn executable_dir() -> Result<PathBuf, Box<dyn Error>> {
    #[cfg(test)]
    return working_dir();

    #[cfg(not(test))]
    {
        let mut path = std::env::current_exe()?;
        path.pop();
        Ok(path)
    }
}

pub fn working_dir() -> Result<PathBuf, Box<dyn Error>> {
    Ok(std::env::current_dir()?)
}

fn create_tool_config() -> Result<(), Box<dyn Error>> {
    let mut buffer = String::new();

    important("Creating tool config:");
    info("Path to UE 4.18:");
    verbose("This folder should contain the \"Engine\" directory");
    verbose(r"Example: C:\Engines\Unreal\UE_4.18");
    std::io::stdin().read_line(&mut buffer)?;
    let engine = PathBuf::from(buffer.trim_end());
    if !engine.is_dir() {
        return Err("Engine directory is not a valid direcoty".into());
    }

    buffer.clear();
    info("Path to Code Vein \"~mods\" folder:");
    verbose(
        r"Example: C:\Program Files (x86)\Steam\steamapps\common\CODE VEIN\CodeVein\Content\Paks\~mods",
    );
    std::io::stdin().read_line(&mut buffer)?;
    let moddir = PathBuf::from(buffer.trim_end());
    if !moddir.is_dir() {
        return Err("Mods directory is not a valid direcoty".into());
    }

    let config = Config::new(engine, moddir);
    config.save()?;

    Ok(())
}
