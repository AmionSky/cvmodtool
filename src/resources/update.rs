use crate::colored::info;
use std::error::Error;
use std::fs::File;
use std::path::PathBuf;
use std::sync::Arc;
use updater::extract::{self, ExtractResult};
use updater::provider::{Asset, DownloadResult, Provider};
use updater::update::{StepAction, UpdateProcedure, UpdateStep};
use updater::{Progress, Version};

pub struct UpdateData {
    pub provider: Box<dyn Provider>,
    pub asset_name: String,
    pub directory: PathBuf,
    pub version: Version,
    pub asset: Option<Box<dyn Asset>>,
    pub file: Option<File>,
}

impl UpdateData {
    pub fn new(
        provider: Box<dyn Provider>,
        asset_name: String,
        version: Version,
        directory: PathBuf,
    ) -> Self {
        UpdateData {
            provider,
            asset_name,
            directory,
            version,
            asset: None,
            file: None,
        }
    }
}

pub struct StepCheckVersion;
impl UpdateStep<UpdateData> for StepCheckVersion {
    fn exec(&self, data: &mut UpdateData, _: &Arc<Progress>) -> Result<StepAction, Box<dyn Error>> {
        info(&format!(
            "Checking for latest version via {}",
            data.provider.name()
        ));
        data.provider.fetch()?;

        let latest = data.provider.latest()?;
        if latest <= data.version {
            info("Resources are up-to-date");
            return Ok(StepAction::Complete);
        }

        let asset = data.provider.asset(&latest, &data.asset_name)?;
        info(&format!("Updating to v{} (from v{})", latest, data.version));

        // Update data
        data.version = latest;
        data.asset = Some(asset);

        Ok(StepAction::Continue)
    }

    fn label(&self, _: &UpdateData) -> String {
        "Checking for latest version...".to_string()
    }
}

pub struct StepDownload;
impl UpdateStep<UpdateData> for StepDownload {
    fn exec(
        &self,
        data: &mut UpdateData,
        progress: &Arc<Progress>,
    ) -> Result<StepAction, Box<dyn Error>> {
        info(&format!("Downloading resources v{}", &data.version));

        let dl_result = data.asset.as_ref().unwrap().download(progress.clone());

        let file = match dl_result {
            DownloadResult::Complete(file) => file,
            DownloadResult::Cancelled => return Ok(StepAction::Cancel),
            DownloadResult::Error(e) => return Err(format!("Asset download failed: {}", e).into()),
        };

        data.file = Some(file);
        info("Download finished!");

        Ok(StepAction::Continue)
    }

    fn label(&self, data: &UpdateData) -> String {
        format!(
            "Downloading {:.2} MB",
            data.asset.as_ref().unwrap().size() as f64 / 1_000_000.0
        )
    }
}

pub struct StepInstall;
impl UpdateStep<UpdateData> for StepInstall {
    fn exec(
        &self,
        data: &mut UpdateData,
        progress: &Arc<Progress>,
    ) -> Result<StepAction, Box<dyn Error>> {
        info("Starting install");

        // (Re)Create install folder
        let install_path = &data.directory;
        if install_path.is_dir() {
            std::fs::remove_dir_all(&install_path)?;
        }
        std::fs::create_dir(&install_path)?;

        // Unpack asset
        if extract::asset(
            data.asset.as_ref().unwrap().name(),
            data.file.take().unwrap(),
            &install_path,
            progress.clone(),
        )? == ExtractResult::Cancelled
        {
            return Ok(StepAction::Cancel);
        }

        Ok(StepAction::Continue)
    }

    fn label(&self, _: &UpdateData) -> String {
        "Installing...".to_string()
    }
}

pub fn create(data: UpdateData) -> UpdateProcedure<UpdateData> {
    let mut procedure = UpdateProcedure::new("Resource Updater".to_string(), data);
    procedure.add_step(Box::new(StepCheckVersion));
    procedure.add_step(Box::new(StepDownload));
    procedure.add_step(Box::new(StepInstall));
    procedure
}
