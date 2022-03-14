use crate::colored::info;
use std::fs::File;
use std::path::PathBuf;
use updater::extract::{self, ExtractResult};
use updater::provider::{Asset, DownloadResult, Provider};
use updater::{State, StepAction, StepResult, Updater, Version};

pub struct UpdateData {
    pub provider: Box<dyn Provider>,
    pub asset_name: String,
    pub directory: PathBuf,
    pub version: Version,
    pub asset: Option<Box<dyn Asset>>,
    pub file: Option<File>,
    pub success: bool,
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
            success: false,
        }
    }
}

fn step_check_version(state: &mut State, data: &mut UpdateData) -> StepResult {
    state.set_label("Checking for latest version...".into());

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

    // Update data
    data.version = latest;
    data.asset = Some(asset);

    Ok(StepAction::Continue)
}

fn step_download(state: &mut State, data: &mut UpdateData) -> StepResult {
    state.set_label(format!(
        "Downloading {:.2} MB",
        data.asset.as_ref().unwrap().size() as f64 / 1_000_000.0
    ));

    info(&format!(
        "Downloading resources v{} ({:.2} MB)",
        &data.version,
        data.asset.as_ref().unwrap().size() as f64 / 1_000_000.0
    ));

    let dl_result = data
        .asset
        .as_ref()
        .unwrap()
        .download(state.progress().clone());

    let file = match dl_result {
        DownloadResult::Complete(file) => file,
        DownloadResult::Cancelled => return Ok(StepAction::Cancel),
        DownloadResult::Error(e) => return Err(format!("Asset download failed: {}", e).into()),
    };

    data.file = Some(file);

    Ok(StepAction::Continue)
}

fn step_install(state: &mut State, data: &mut UpdateData) -> StepResult {
    state.set_label("Unpacking...".into());

    info("Unpacking resources");

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
        state.progress().clone(),
    )? == ExtractResult::Cancelled
    {
        return Ok(StepAction::Cancel);
    }

    data.success = true;
    Ok(StepAction::Continue)
}

pub fn create(data: UpdateData) -> Updater<UpdateData> {
    let mut updater = Updater::new(data);
    updater.set_title("Resource Updater".into());
    updater.add_step(step_check_version);
    updater.add_step(step_download);
    updater.add_step(step_install);
    updater
}
