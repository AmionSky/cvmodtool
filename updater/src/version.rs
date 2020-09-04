use semver::Version;
use std::error::Error;

pub fn extract(version: &str) -> Result<Version, Box<dyn Error>> {
    Ok(Version::parse(version[1..].into())?)
}
