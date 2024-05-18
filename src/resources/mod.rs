pub mod modules;
pub mod profiles;
#[cfg(feature = "updater")]
pub mod update;

pub const REPLACE: &str = "PROJECTNAME";

pub fn dir() -> Result<std::path::PathBuf, std::io::Error> {
    Ok(crate::executable_dir()?.join("resources"))
}
