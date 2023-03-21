use anyhow::Result;
use dialoguer::{console::Term, theme, Input, Select};

mod files;
mod images;

fn read_folder() -> String {
    let folder: String = Input::with_theme(&theme::ColorfulTheme::default())
        .with_prompt("What is the folder you want to update?")
        .default("src".to_string())
        .show_default(true)
        .interact()
        .unwrap();

    folder
}

fn download_and_replace_prompt() -> bool {
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

fn group_occurrences_by_file(
    occurrences: &Vec<files::Occurrence>,
) -> std::collections::HashMap<String, Vec<files::Occurrence>> {
    let mut occurrences_by_file: std::collections::HashMap<String, Vec<files::Occurrence>> =
        std::collections::HashMap::new();
    for occurrence in occurrences {
        let file_occurrences = occurrences_by_file
            .entry(occurrence.file.clone())
            .or_default();
        file_occurrences.push(occurrence.clone());
    }

    occurrences_by_file
}

fn ask_for_target_folder() -> String {
    let target_folder: String = Input::with_theme(&theme::ColorfulTheme::default())
        .with_prompt("What is the target folder?")
        .default("public".to_string())
        .show_default(true)
        .interact()
        .unwrap();

    target_folder
}

fn main() -> Result<()> {
    let term = Term::stdout();

    let folder = read_folder();
    let target_folder = ask_for_target_folder();

    let files = files::find_files(folder)?;
    let image_occurrences = files::find_image_urls(&files)?;

    term.write_line(&format!(
        "\nFound {} occurrences \n",
        image_occurrences.len()
    ))?;

    let occurrences_by_file = group_occurrences_by_file(&image_occurrences);

    for (file, occurrences) in occurrences_by_file {
        term.write_line(&format!("{} images found in {}:", occurrences.len(), file))?;
        for occurrence in occurrences {
            term.write_line(&format!(
                "   line {}, column {} to {}",
                occurrence.line, occurrence.column, occurrence.column_end
            ))?;
        }
    }

    term.write_line("\n")?;

    let download_and_replace = download_and_replace_prompt();

    if download_and_replace {
        term.write_line("Downloading images...")?;

        let download_count = images::download_images(&image_occurrences, target_folder)?;

        term.move_cursor_up(1)?;
        term.clear_line()?;
        term.write_line(&format!(
            "{}/{} images downloaded successfully",
            download_count,
            image_occurrences.len()
        ))?;
    }

    Ok(())
}
