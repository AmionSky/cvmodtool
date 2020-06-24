pub mod create;

use clap::Clap;
use create::Create;

pub const PKG_VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Clap)]
#[clap(version = PKG_VERSION, author = "István Cs. <icsanyi96@gmail.com>")]
pub struct Opts {
    /// A level of verbosity, and can be used multiple times
    #[clap(short, long, parse(from_occurrences))]
    verbose: i32,
    #[clap(subcommand)]
    pub subcmd: SubCommand,
}

#[derive(Clap)]
pub enum SubCommand {
    Create(Create),
}
