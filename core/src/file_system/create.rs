use std::{
    fs::{File, create_dir_all},
    path::PathBuf,
};

use crate::{
    file_system::directories::get_parent_dir,
    processes::{ProcessEvent, ProcessId},
};

pub async fn create(
    path: PathBuf,
    is_dir: bool,
    tx: tokio::sync::mpsc::Sender<ProcessEvent>,
    process_id: ProcessId,
) -> Result<(), Option<Vec<String>>> {
    let mut errors: Vec<String> = vec![];

    if is_dir {
        let _ = tx
            .send(ProcessEvent::Progress {
                id: process_id,
                progress: None,
                bytes_done: None,
                bytes_total: None,
                speed: None,
                eta: None,
                message: Some("Creating directory...".to_string()),
                entries_total: None,
                entries_done: None,
                errors: None,
            })
            .await;

        if let Err(err) = create_dir_all(&path) {
            errors.push(err.to_string());
            return Err(Some(errors));
        }

        return Ok(());
    }

    let _ = tx
        .send(ProcessEvent::Progress {
            id: process_id,
            progress: None,
            bytes_done: None,
            bytes_total: None,
            speed: None,
            eta: None,
            message: Some("Creating file...".to_string()),
            entries_total: None,
            entries_done: None,
            errors: None,
        })
        .await;

    let parent = get_parent_dir(path.clone());

    if let Err(err) = create_dir_all(parent) {
        errors.push(err.to_string());
    }

    if let Err(err) = File::create(path) {
        errors.push(err.to_string());
    }

    if !errors.is_empty() {
        return Err(Some(errors));
    }

    Ok(())
}
