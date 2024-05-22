use semver::Version;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum GitHubResponse {
    Release(GitHubRelease),
    Error(GitHubError),
}

#[derive(Debug, Deserialize)]
pub struct GitHubError {
    pub message: String,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct GitHubRelease {
    tag_name: String,
    assets: Vec<GitHubAsset>,
}

impl GitHubRelease {
    pub fn version(&self) -> Result<Version, semver::Error> {
        Version::parse(&self.tag_name[1..])
    }

    pub fn assets(&self) -> &[GitHubAsset] {
        &self.assets
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct GitHubAsset {
    name: String,
    size: u64,
    browser_download_url: String,
}

impl GitHubAsset {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn size(&self) -> u64 {
        self.size
    }

    pub fn url(&self) -> &str {
        &self.browser_download_url
    }
}
