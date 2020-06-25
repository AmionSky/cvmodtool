pub mod modules;

pub const REPLACE: &str = "PROJECTNAME";

pub fn dir() -> Result<std::path::PathBuf, Box<dyn std::error::Error>> {
    Ok(crate::executable_dir()?.join("resources"))
}
