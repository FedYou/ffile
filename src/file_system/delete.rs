use std::{
    fs::{remove_dir_all, remove_file, symlink_metadata},
    io::Error,
    path::PathBuf,
};

pub fn delete(path: PathBuf) -> Result<(), Error> {
    let metadata = symlink_metadata(&path)?;

    if metadata.is_dir() {
        remove_dir_all(path)
    } else {
        remove_file(path)
    }
}
