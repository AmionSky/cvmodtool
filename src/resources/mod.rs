pub mod modules;
pub mod profiles;

pub const REPLACE: &str = "PROJECTNAME";

pub fn dir() -> std::path::PathBuf {
    crate::EXEDIR.join("resources")
}
