#![warn(clippy::all)]

use dotenv::dotenv;
use std::env;
use std::fs::File;
use std::io::Write;

fn main() -> std::io::Result<()> {
    let cwd = env::current_dir().unwrap();

    let default_dotenv = cwd.join(".env");
    let custom_dotenv = cwd.join("custom/.env");

    if !default_dotenv.exists() {
        println!("Creating default '{}' file", default_dotenv.file_stem().unwrap().display());
        let mut dotenv_file = File::create(default_dotenv)?;
        writeln!(dotenv_file, "FIRST_VARIABLE=\"First Variable\"")?;
    }

    if !custom_dotenv.exists() {
        println!("Creating custom '{}' file", custom_dotenv.strip_prefix(&cwd).unwrap().display());
        
        let mut dotenv_file = File::create(&custom_dotenv)?;
        writeln!(dotenv_file, "SECOND_VARIABLE=\"Second Variable\"")?;
    }

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
    println!("Custom dotenv file path is: {}", custom_dotenv.display());

    // Loads .env file
    dotenv().ok();

    // Loads .env file in specific path
    // Pass a reference to avoid moving the PathBuf
    let _ = dotenv::from_path(&custom_dotenv);

    // Panic if FIRST_VARIABLE doesn't exist
    if env::var("FIRST_VARIABLE").is_err() {
        panic!("FIRST_VARIABLE was not found!")
    }

    // Panic if SECOND_VARIABLE doesn't exist
    if env::var("SECOND_VARIABLE").is_err() {
        panic!("SECOND_VARIABLE was not found!")
    }

    // Unwrap here because it's already handled with panic above.
    let first_variable = env::var("FIRST_VARIABLE").unwrap();
    let second_variable = env::var("SECOND_VARIABLE").unwrap();

    println!(
        "File: {} Variable: {}",
        cwd.join(".env").display(),
        first_variable
    );
    println!(
        "File: {} Variable: {}",
        custom_dotenv.display(),
        second_variable
    );

    Ok(())
}
