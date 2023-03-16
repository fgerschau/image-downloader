use anyhow::Result;
use std::fs;
use std::io::{Error, ErrorKind};

pub fn find_files(folder: String) -> Result<Vec<String>> {
    let entries = fs::read_dir(folder)?;

    let mut files: Vec<String> = vec![];

    for entry in entries {
        let path = entry?.path();
        let meta = path.metadata()?;

        let path_string = match path.to_str() {
            None => {
                return Err(Error::new(ErrorKind::Other, "Invalid path").into());
            }
            Some(path) => path.to_string(),
        };

        if meta.is_file() {
            files.push(path_string);
        } else if meta.is_dir() {
            let folder_files = find_files(path_string)?;
            files.extend(folder_files);
        }
    }

    Ok(files)
}
