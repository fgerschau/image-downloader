use dialoguer::{theme, Input, Select};

pub fn read_folder() -> String {
    let folder: String = Input::with_theme(&theme::ColorfulTheme::default())
        .with_prompt("What is the folder you want to update?")
        .default("src".to_string())
        .show_default(true)
        .interact()
        .unwrap();

    folder
}

pub fn download_and_replace() -> bool {
    let download_and_replace = Select::with_theme(&theme::ColorfulTheme::default())
        .with_prompt("Do you want to download the images?")
        .items(&["Yes", "No"])
        .default(0)
        .interact()
        .unwrap();

    match download_and_replace {
        0 => true,
        _ => false,
    }
}

pub fn ask_for_target_folder() -> String {
    let target_folder: String = Input::with_theme(&theme::ColorfulTheme::default())
        .with_prompt("What is the target folder?")
        .default("public".to_string())
        .show_default(true)
        .interact()
        .unwrap();

    target_folder
}
