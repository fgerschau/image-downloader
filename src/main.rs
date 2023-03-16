use anyhow::Result;
use dialoguer::{console::Term, Input};

mod files;

fn read_folder() -> String {
    let folder: String = Input::new()
        .with_prompt("What is the folder you want to update?")
        .default("src".to_string())
        .show_default(true)
        .interact()
        .unwrap();

    folder
}

fn main() -> Result<()> {
    let folder = read_folder();
    let files = files::find_files(folder)?;
    let term = Term::stdout();
    term.write_line(&format!("Files: {:?}", files))?;

    Ok(())
}
