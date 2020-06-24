#![allow(unused)]

mod colored;
mod commands;
mod config;
mod resources;
mod utils;

use clap::Clap;
use colored::*;
use commands::{Opts, SubCommand};
use std::error::Error;
use std::path::PathBuf;

fn main() {
    let opts: Opts = Opts::parse();

    // Set verbose logging
    unsafe {
        USE_VERBOSE = opts.verbose();
    }

    match opts.subcmd() {
        SubCommand::Create(c) => {
            if let Err(err) = commands::create::execute(&c) {
                error(&format!("Failed to create project: {}", err));
            }
        }
        SubCommand::Build(c) => {
            if let Err(err) = commands::build::execute(&c) {
                error(&format!("Failed to build project: {}", err));
            }
        }
    }
}
#[cfg(not(test))]
pub fn executable_dir() -> Result<PathBuf, Box<dyn Error>> {
    let mut path = std::env::current_exe()?;
    path.pop();
    Ok(path)
}

// Support for tests
#[cfg(test)]
pub fn executable_dir() -> Result<PathBuf, Box<dyn Error>> {
    working_dir()
}

pub fn working_dir() -> Result<PathBuf, Box<dyn Error>> {
    let path = std::env::current_dir()?;
    Ok(path)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let tp = PathBuf::from("TestProject");
        if tp.exists() {
            std::fs::remove_dir_all(tp).unwrap();
        }

        let cfg = commands::create::Create::new("TestProject".into());

        if let Err(err) = commands::create::execute(&cfg) {
            error(&format!("Failed to create project: {}", err));
            panic!("FAILED")
        }
    }
}
