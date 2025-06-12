#![warn(clippy::all)]

use dotenv::dotenv;
use std::env;

fn main() {
    let cwd = env::current_dir().unwrap();
    let custom_dotenv_file = cwd.join("custom/.env");

    println!("The current directory is: {}", cwd.display());
    println!(
        "The directory name is: {}",
        cwd.file_stem().unwrap().display()
    );
    println!(
        "The parent directory is: {}",
        cwd.parent().unwrap().display()
    );
    println!(
        "Default dotenv file path is: {}",
        cwd.join(".env").display()
    );
    println!(
        "Custom dotenv file path is: {}",
        custom_dotenv_file.display()
    );

    // Loads .env file
    dotenv().ok();

    // Loads .env file in specific path
    // Pass a reference to avoid moving the PathBuf
    let _ = dotenv::from_path(&custom_dotenv_file);

    // Panic if FIRST_VARIABLE doesn't exist
    if env::var("FIRST_VARIABLE").is_err() {
        panic!("FIRST_VARIABLE was not found!")
    }

    // Panic if SECOND_VARIABLE doesn't exist
    if env::var("SECOND_VARIABLE").is_err() {
        panic!("SECOND_VARIABLE was not found!")
    }

    let first_variable = env::var("FIRST_VARIABLE").unwrap();
    let second_variable = env::var("SECOND_VARIABLE").unwrap();

    println!(
        "File: {} Variable: {}",
        cwd.join(".env").display(),
        first_variable
    );
    println!(
        "File: {} Variable: {}",
        custom_dotenv_file.display(),
        second_variable
    );
}
