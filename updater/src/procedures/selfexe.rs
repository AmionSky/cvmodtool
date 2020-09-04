use crate::provider::{Asset, DownloadResult, Provider};
use crate::update::{StepAction, UpdateProcedure, UpdateStep};
use crate::Progress;
use log::{error, info};
use semver::Version;
use std::error::Error;
use std::fs::File;
use std::io::{BufReader, BufWriter, Write};
use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;

pub struct UpdateData {
    provider: Box<dyn Provider>,
    current_exe: PathBuf,
    new_exe: PathBuf,
    temp_exe: PathBuf,
    version: Version,
    asset_name: String,
    asset: Option<Box<dyn Asset>>,
    file: Option<File>,
}

impl UpdateData {
    pub fn new(
        provider: Box<dyn Provider>,
        asset_name: String,
        version: Version,
        current_exe: PathBuf,
    ) -> Self {
        let new_exe = current_exe.with_extension("new");
        let temp_exe = current_exe.with_extension("old");

        Self {
            provider,
            current_exe,
            new_exe,
            temp_exe,
            version,
            asset_name,
            asset: None,
            file: None,
        }
    }
}

pub fn create(data: UpdateData) -> UpdateProcedure<UpdateData> {
    let mut procedure = UpdateProcedure::new("Self-Updater".to_string(), data);
    procedure.add_step(Box::new(StepCleanUp));
    procedure.add_step(Box::new(StepCheckVersion));
    procedure.add_step(Box::new(StepDownload));
    procedure.add_step(Box::new(StepInstall));
    procedure
}

pub struct StepCleanUp;
impl UpdateStep<UpdateData> for StepCleanUp {
    fn exec(&self, data: &mut UpdateData, _: &Arc<Progress>) -> Result<StepAction, Box<dyn Error>> {
        if data.temp_exe.is_file() {
            std::fs::remove_file(&data.temp_exe)?;
        }

        Ok(StepAction::Continue)
    }

    fn label(&self, _: &UpdateData) -> String {
        "Cleaning up...".to_string()
    }
}

pub struct StepCheckVersion;
impl UpdateStep<UpdateData> for StepCheckVersion {
    fn exec(&self, data: &mut UpdateData, _: &Arc<Progress>) -> Result<StepAction, Box<dyn Error>> {
        info!("Checking for latest version via {}", data.provider.name());
        data.provider.fetch()?;

        let (latest, asset) = data.provider.latest(&data.asset_name)?;
        if latest <= data.version {
            info!("Up-to-date");
            return Ok(StepAction::Complete);
        }

        info!("Updating to v{} (from v{})", latest, data.version);

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
        let dl_result = data.asset.as_ref().unwrap().download(progress.clone());

        let file = match dl_result {
            DownloadResult::Complete(file) => file,
            DownloadResult::Cancelled => return Ok(StepAction::Cancel),
            DownloadResult::Error(e) => return Err(format!("Asset download failed: {}", e).into()),
        };

        data.file = Some(file);
        info!("Download finished!");

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
    fn exec(&self, data: &mut UpdateData, _: &Arc<Progress>) -> Result<StepAction, Box<dyn Error>> {
        info!("Starting install");

        // Copy new updater exe
        copy_file(&data.file.as_ref().unwrap(), &data.new_exe)?;

        // Swap updater exe
        replace_temp(&data.new_exe, &data.current_exe, &data.temp_exe)?;

        Ok(StepAction::Continue)
    }

    fn label(&self, _: &UpdateData) -> String {
        "Installing...".to_string()
    }
}

fn copy_file<P: AsRef<Path>>(file: &File, target_path: P) -> Result<(), Box<dyn Error>> {
    let mut target_file = File::create(target_path)?;

    // Copy
    {
        let mut reader = BufReader::new(file);
        let mut writer = BufWriter::new(&target_file);
        std::io::copy(&mut reader, &mut writer)?;
    }

    target_file.flush()?;

    Ok(())
}

/// Replace file by renaming it to a temp name
fn replace_temp<P: AsRef<Path>>(replacement: P, target: P, temp: P) -> Result<(), Box<dyn Error>> {
    // First make sure the replacement exist before doing any work
    if std::fs::metadata(&replacement).is_err() {
        return Err("Replacement file does not exist!".into());
    }

    // Rename files
    if let Err(e) = std::fs::rename(&target, &temp) {
        error!("replace_temp: Failed to move target(original) to temp!");
        return Err(e.into());
    }

    if let Err(e) = std::fs::rename(&replacement, &target) {
        error!("replace_temp: Failed to move replacement to target!");

        // In case of error, undo the previous rename
        if std::fs::rename(&temp, &target).is_err() {
            error!("replace_temp: Failed to recover from error!");
        }

        return Err(e.into());
    }

    Ok(())
}
