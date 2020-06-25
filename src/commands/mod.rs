pub mod build;
pub mod create;
pub mod install;
pub mod package;

use clap::Clap;

pub const PKG_VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Clap)]
#[clap(version = PKG_VERSION, author = "AmionSky <icsanyi96@gmail.com>")]
pub struct Opts {
    /// A level of verbosity
    #[clap(short, long)]
    verbose: bool,

    #[clap(subcommand)]
    subcmd: SubCommand,
}

impl Opts {
    pub fn verbose(&self) -> bool {
        self.verbose
    }

    pub fn subcmd(&self) -> &SubCommand {
        &self.subcmd
    }
}

#[derive(Clap)]
pub enum SubCommand {
    Create(create::Create),
    Build(build::Build),
    Package(package::Package),
    Install(install::Install),
}
