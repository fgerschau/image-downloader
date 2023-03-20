use anyhow::Result;
use regex::Regex;
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

#[derive(Clone, Debug)]
pub struct Occurrence {
    pub file: String,
    pub line: usize,
    pub column: usize,
    pub column_end: usize,
    pub text: String,
}

pub fn find_image_urls(files: &Vec<String>) -> Result<Vec<Occurrence>> {
    let mut occurrences: Vec<Occurrence> = vec![];

    for file in files {
        let contents = fs::read_to_string(file.clone())?;

        for (line_number, line) in contents.lines().enumerate() {
            let line_number = line_number + 1;

            let regex = Regex::new(r#"https?://[^\s",^\s']+"#)?;
            let matches = regex.find_iter(line);
            matches.for_each(|m| {
                let occurrence = Occurrence {
                    file: file.clone(),
                    line: line_number,
                    column: m.start(),
                    column_end: m.end(),
                    text: m.as_str().to_string(),
                };

                occurrences.push(occurrence);
            });
        }
    }

    Ok(occurrences)
}
