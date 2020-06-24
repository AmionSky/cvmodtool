use std::collections::HashMap;
use std::error::Error;

const REL_PATH: &str = "resources\\profiles.toml";

type Profiles = HashMap<String, Vec<String>>;

pub fn load() -> Result<Profiles, Box<dyn Error>> {
    let executable_dir = crate::executable_dir()?;
    let profiles_file = executable_dir.join(REL_PATH);
    let profiles: Profiles = toml::from_str(&std::fs::read_to_string(profiles_file)?)?;
    Ok(profiles)
}
