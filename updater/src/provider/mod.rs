pub mod github;

pub use github::GitHubProvider;

use crate::Progress;
use semver::Version;
use std::error::Error;
use std::fs::File;
use std::sync::Arc;

#[derive(Debug)]
pub enum DownloadResult {
    Complete(File),
    Cancelled,
    Error(Box<dyn Error>),
}

pub trait Provider {
    /// Gets the name of the provider.
    fn name(&self) -> &'static str;

    /// Fetches all necessary data for the provider.
    fn fetch(&mut self) -> Result<(), Box<dyn Error>>;

    /// Returns the latest downloadable asset with the specified name.
    fn latest(&self, name: &str) -> Result<(Version, Box<dyn Asset>), Box<dyn Error>>;
}

pub trait Asset: Send {
    /// Gets the name of the asset
    fn name(&self) -> &str;

    /// Gets the size of the asset in bytes
    fn size(&self) -> u64;

    /// Gets the url of the asset
    fn url(&self) -> &str;

    /// Clone into a Box
    fn box_clone(&self) -> Box<dyn Asset>;

    /// Download the asset into a temprary file on a separate thread
    fn download(&self, progress: Arc<Progress>) -> DownloadResult {
        use std::io::{ErrorKind, Read, Seek, SeekFrom, Write};

        log::info!(
            "Downloading {} - {:.2}MB",
            self.name(),
            self.size() as f64 / 1_000_000.0
        );

        // Setup progress
        progress.set_maximum(self.size());
        progress.set_indeterminate(false);

        // Send request message
        let resp = ureq::get(self.url()).timeout_connect(5_000).call();
        if !resp.ok() {
            return DownloadResult::Error("Response not OK".into());
        }

        // Init reader and temp file
        let mut reader = resp.into_reader();
        let mut out = match tempfile::tempfile() {
            Ok(file) => file,
            Err(e) => return DownloadResult::Error(e.into()),
        };

        // Copy received data into temo file
        const BUF_SIZE: usize = 4096;
        let mut buf: [u8; BUF_SIZE] = [0; BUF_SIZE];
        loop {
            if progress.cancelled() {
                return DownloadResult::Cancelled;
            }

            let len = match reader.read(&mut buf) {
                Ok(0) => break,
                Ok(len) => len,
                Err(ref e) if e.kind() == ErrorKind::Interrupted => continue,
                Err(e) => return DownloadResult::Error(e.into()),
            };

            if let Err(e) = out.write_all(&buf[..len]) {
                return DownloadResult::Error(e.into());
            };
            progress.add_current(len as u64);
        }

        // Flush and reset temp file
        if let Err(e) = out.flush() {
            return DownloadResult::Error(e.into());
        };
        if let Err(e) = out.seek(SeekFrom::Start(0)) {
            return DownloadResult::Error(e.into());
        };

        DownloadResult::Complete(out)
    }
}
