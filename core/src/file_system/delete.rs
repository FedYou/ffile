use std::{
    fs::{remove_dir_all, remove_file, symlink_metadata},
    path::PathBuf,
};

use crate::processes::{ProcessEvent, ProcessId};

pub async fn delete(
    path: PathBuf,
    tx: tokio::sync::mpsc::Sender<ProcessEvent>,
    process_id: ProcessId,
) -> Result<(), Option<Vec<String>>> {
    let mut errors: Vec<String> = vec![];

    let _ = tx
        .send(ProcessEvent::Progress {
            id: process_id,
            progress: None,
            bytes_done: None,
            bytes_total: None,
            speed: None,
            eta: None,
            message: Some("Deleting files...".to_string()),
            entries_total: None,
            entries_done: None,
            errors: None,
        })
        .await;

    match symlink_metadata(&path) {
        Ok(metadata) => {
            if metadata.is_dir() {
                if let Err(err) = remove_dir_all(path) {
                    errors.push(err.to_string());
                }
            } else {
                if let Err(err) = remove_file(path) {
                    errors.push(err.to_string());
                }
            }
        }
        Err(err) => {
            errors.push(err.to_string());
        }
    }

    if !errors.is_empty() {
        return Err(Some(errors));
    }

    Ok(())
}
