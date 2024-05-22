mod github;

use crate::utils::EXEDIR;

use self::github::{GitHubRelease, GitHubResponse};
use native_tls::TlsConnector;
use once_cell::sync::Lazy;
use semver::Version;
use std::fs::File;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;
use thiserror::Error;
use ureq::{Agent, AgentBuilder};

static AGENT: Lazy<Agent> = Lazy::new(|| {
    AgentBuilder::new()
        .tls_connector(Arc::new(TlsConnector::new().unwrap()))
        .build()
});

pub struct Updater {
    release: GitHubRelease,
}

impl Updater {
    pub fn new(repo: &str) -> Result<Self, UpdaterError> {
        let url = format!("https://api.github.com/repos/{repo}/releases/latest");

        let response: GitHubResponse = AGENT
            .get(&url)
            .set("Accept", "application/vnd.github.v3+json")
            .timeout(Duration::from_secs(10))
            .call()
            .map_err(|e| UpdaterError::NoResponse(Box::new(e)))?
            .into_json()
            .map_err(UpdaterError::InvalidResponse)?;

        match response {
            GitHubResponse::Release(release) => Ok(Self { release }),
            GitHubResponse::Error(e) => Err(UpdaterError::ApiError(e.message)),
        }
    }

    pub fn version(&self) -> Result<Version, UpdaterError> {
        Ok(self.release.version()?)
    }

    /// Downloads the asset into a temporary file and returns the path to it
    pub fn download(&self, asset: &str) -> Result<PathBuf, UpdaterError> {
        let Some(asset) = self.release.assets().iter().find(|a| a.name() == asset) else {
            return Err(UpdaterError::NotFound(asset.to_string()));
        };

        info!(
            "Downloading {} ({:.2} MB)",
            asset.name(),
            asset.size() as f64 / 1_000_000.0
        );
        let mut reader = AGENT
            .get(asset.url())
            .call()
            .map_err(|e| UpdaterError::NoResponse(Box::new(e)))?
            .into_reader();

        let path = EXEDIR.join(format!("{}.dltmp", asset.name()));
        let mut file = File::create(&path).map_err(UpdaterError::File)?;

        std::io::copy(&mut reader, &mut file).map_err(UpdaterError::Download)?;

        Ok(path)
    }
}

#[derive(Debug, Error)]
pub enum UpdaterError {
    #[error("Failed to get a response from GitHub: {0}")]
    NoResponse(#[source] Box<ureq::Error>),
    #[error("Received invalid data: {0}")]
    InvalidResponse(#[source] std::io::Error),
    #[error("GitHub API error: {0}")]
    ApiError(String),
    #[error("Failed to parse version string: {0}")]
    Semver(#[from] semver::Error),
    #[error("Asset '{0}' not found in release.")]
    NotFound(String),
    #[error("Failed to create temporary file. ({0})")]
    File(#[source] std::io::Error),
    #[error("Download failed with {0}.")]
    Download(#[source] std::io::Error),
}
