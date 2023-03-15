use dialoguer::Input;

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

fn main() {
    let folder = read_folder();
    let files = files::find_files(folder);
    println!("{:?}", files);
}
