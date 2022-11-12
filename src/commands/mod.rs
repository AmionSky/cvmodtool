pub mod build;
pub mod create;
pub mod install;
pub mod package;
#[cfg(feature = "updater")]
pub mod update;

pub const PKG_VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(clap::Parser)]
#[command(version = PKG_VERSION, author = "Amion <icsanyi96@gmail.com>")]
pub struct Opts {
    /// A level of verbosity
    #[arg(short, long)]
    verbose: bool,

    #[command(subcommand)]
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

#[derive(clap::Subcommand)]
pub enum SubCommand {
    Create(create::Create),
    Build(build::Build),
    Package(package::Package),
    Install(install::Install),
    #[cfg(feature = "updater")]
    Update(update::Update),
}
