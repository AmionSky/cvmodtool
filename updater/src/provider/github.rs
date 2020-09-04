use super::{Asset, Provider};
use crate::version;
use semver::Version;
use serde::Deserialize;
use std::error::Error;
use std::time::Duration;

#[derive(Debug)]
pub struct GitHubProvider {
    url: String,
    releases: Option<Vec<GitHubRelease>>,
}

impl GitHubProvider {
    /// Creates a new GitHubProvider.
    ///
    /// * `repo` should be "*user*/*repository*".
    pub fn new(repo: &str) -> Self {
        Self {
            url: format!("https://api.github.com/repos/{}/releases", repo),
            releases: None,
        }
    }

    /// Gets the fetched data and returns it or Err if not.
    fn releases(&self) -> Result<&Vec<GitHubRelease>, Box<dyn Error>> {
        match self.releases.as_ref() {
            Some(rel) => Ok(&rel),
            None => Err("No fetched content found!".into()),
        }
    }
}

impl Provider for GitHubProvider {
    fn name(&self) -> &'static str {
        "GitHub"
    }

    fn fetch(&mut self) -> Result<(), Box<dyn Error>> {
        let resp = ureq::get(&self.url)
            .set("Accept", "application/vnd.github.v3+json")
            .timeout(Duration::from_secs(10))
            .call();

        let release: GitHubResponse = resp.into_json_deserialize()?;

        match release {
            GitHubResponse::Release(release) => {
                self.releases = Some(release);
                Ok(())
            }
            GitHubResponse::Error(err) => Err(err.message.into()),
        }
    }

    fn latest(&self, name: &str) -> Result<(Version, Box<dyn Asset>), Box<dyn Error>> {
        let releases = self.releases()?;

        let mut latest_version = Version::new(0, 0, 0);
        let mut latest_asset = None;

        for release in releases {
            let version = release.version()?;
            if version > latest_version {
                let asset = release.assets.iter().find(|a| a.name() == name);
                if asset.is_some() {
                    latest_version = version;
                    latest_asset = asset;
                }
            }
        }

        if let Some(asset) = latest_asset {
            Ok((latest_version, asset.box_clone()))
        } else {
            Err("Asset not found".into())
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum GitHubResponse {
    Release(Vec<GitHubRelease>),
    Error(GitHubError),
}

#[derive(Debug, Deserialize)]
struct GitHubError {
    message: String,
}

#[derive(Debug, Deserialize)]
struct GitHubRelease {
    name: String,
    tag_name: String,
    prerelease: bool,
    assets: Vec<GitHubAsset>,
}

impl GitHubRelease {
    pub fn version(&self) -> Result<Version, Box<dyn Error>> {
        version::extract(&self.tag_name)
    }
}

#[derive(Debug, Clone, Deserialize)]
struct GitHubAsset {
    name: String,
    size: u64,
    browser_download_url: String,
}

impl Asset for GitHubAsset {
    fn name(&self) -> &str {
        &self.name
    }

    fn size(&self) -> u64 {
        self.size
    }

    fn url(&self) -> &str {
        &self.browser_download_url
    }

    fn box_clone(&self) -> Box<dyn Asset> {
        Box::new(self.clone())
    }
}
