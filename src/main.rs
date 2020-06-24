mod colored;
mod commands;
mod profiles;
mod resources;
mod utils;

use clap::Clap;
use colored::*;
use commands::{create, Opts, SubCommand};
use std::error::Error;
use std::path::PathBuf;

fn main() {
    let opts: Opts = Opts::parse();

    match &opts.subcmd {
        SubCommand::Create(c) => {
            if let Err(err) = create::execute(&c) {
                error(&format!("Failed to create project: {}", err));
            }
        }
    }
}

pub fn executable_dir() -> Result<PathBuf, Box<dyn Error>> {
    let mut path = std::env::current_exe()?;
    path.pop();
    Ok(path)
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

        let cfg = create::Create::new("TestProject".into());

        if let Err(err) = create::execute(&cfg) {
            error(&format!("Failed to create project: {}", err));
            panic!("FAILED")
        }
    }
}
