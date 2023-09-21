use clap::builder::Styles;
use clap::{Parser, Subcommand};

pub mod build;
pub mod create;
pub mod install;
pub mod package;
#[cfg(feature = "updater")]
pub mod update;

pub const PKG_VERSION: &str = env!("CARGO_PKG_VERSION");

fn clap_styles() -> Styles {
    use clap::builder::styling::*;
    Styles::styled()
        .header(AnsiColor::Yellow.on_default())
        .usage(AnsiColor::Yellow.on_default())
        .literal(AnsiColor::Cyan.on_default())
        .placeholder(AnsiColor::White.on_default() | Effects::ITALIC)
        .error(AnsiColor::Red.on_default() | Effects::BOLD)
        .valid(AnsiColor::Cyan.on_default() | Effects::BOLD)
        .invalid(AnsiColor::Yellow.on_default() | Effects::BOLD)
}

#[derive(Parser)]
#[command(
    version = PKG_VERSION,
    styles = clap_styles(),
    about = format!("Code Vein Modding Tool v{PKG_VERSION}")
)]
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

#[derive(Subcommand)]
pub enum SubCommand {
    Create(create::Create),
    Build(build::Build),
    Package(package::Package),
    Install(install::Install),
    #[cfg(feature = "updater")]
    Update(update::Update),
}
