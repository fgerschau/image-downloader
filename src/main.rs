use dialoguer::Input;
use std::fs;

fn read_folder() -> String {
    let folder: String = Input::new()
        .with_prompt("What is the folder you want to update?")
        .default("src".to_string())
        .show_default(true)
        .interact()
        .unwrap();

    folder
}

fn find_files(folder: String) -> Vec<String> {
    let entries = match fs::read_dir(folder) {
        Ok(entries) => entries,
        Err(_) => {
            println!("Folder not found");
            return vec![];
        }
    };

    let mut files: Vec<String> = vec![];

    for entry in entries {
        let path = match entry {
            Ok(entry) => entry.path(),
            Err(_) => {
                println!("Error reading entry");
                return vec![];
            }
        };

        let meta = match path.metadata() {
            Ok(meta) => meta,
            Err(_) => {
                println!("Error reading metadata");
                return vec![];
            }
        };

        if meta.is_file() {
            match path.to_str() {
                Some(path) => files.push(path.to_string()),
                None => {
                    println!("Error reading path");
                }
            };
        } else if meta.is_dir() {
            let folder_files = match path.to_str() {
                Some(path) => find_files(path.to_string()),
                None => {
                    println!("Error reading path");
                    return vec![];
                }
            };

            files = [files, folder_files].concat();
        }
    }

    files
}

fn main() {
    let folder = read_folder();
    let files = find_files(folder);
    println!("{:?}", files);
}
