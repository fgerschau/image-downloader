use anyhow::{Error, Result};
use dialoguer::{theme, Input, Select};
use tokio::fs;

async fn check_folder_exists(folder: &String, create: bool) -> Result<()> {
    match fs::read_dir(&folder).await {
        Ok(_) => &folder,
        Err(_) => {
            if create {
                let should_create = Select::with_theme(&theme::ColorfulTheme::default())
                    .with_prompt(format!(
                        "Folder {} does not exist. Do you want to create it?",
                        folder
                    ))
                    .items(&["Yes", "No"])
                    .default(0)
                    .interact()?;

                if should_create == 0 {
                    fs::create_dir(&folder).await?;
                }
            }

            &folder
        }
    };

    Ok(())
}

pub async fn read_folder() -> Result<String> {
    let folder: String = Input::with_theme(&theme::ColorfulTheme::default())
        .with_prompt("What is the folder you want to update?")
        .default("src".to_string())
        .show_default(true)
        .interact()?;

    match fs::read_dir(&folder).await {
        Ok(_) => &folder,
        Err(err) => {
            return Err(Error::new(err).context("Folder does not exist"));
        }
    };

    Ok(folder)
}

pub fn download_and_replace() -> Result<bool> {
    let download_and_replace = Select::with_theme(&theme::ColorfulTheme::default())
        .with_prompt("Do you want to download the images?")
        .items(&["Yes", "No"])
        .default(0)
        .interact()?;

    let res = match download_and_replace {
        0 => true,
        _ => false,
    };

    Ok(res)
}

pub async fn ask_for_target_folder() -> Result<String> {
    let folder: String = Input::with_theme(&theme::ColorfulTheme::default())
        .with_prompt("What is the target folder?")
        .default("public".to_string())
        .show_default(true)
        .interact()?;

    check_folder_exists(&folder, true).await?;

    Ok(folder)
}

pub fn replace() -> Result<bool> {
    let replace_occurrences = Select::with_theme(&theme::ColorfulTheme::default())
        .with_prompt("Do you want to replace the occurrences?")
        .items(&["Yes", "No"])
        .default(0)
        .interact()?;

    let res = match replace_occurrences {
        0 => true,
        _ => false,
    };

    Ok(res)
}
