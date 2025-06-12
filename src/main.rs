#![warn(clippy::all)]

use dotenv::dotenv;
use std::env;
use std::fs::{self, File};
use std::io::Write;

use comfy_table::presets::UTF8_FULL;
use comfy_table::{ContentArrangement, Table};

fn main() -> std::io::Result<()> {
    let cwd = env::current_dir().unwrap();

    let default_dotenv = cwd.join(".env");
    let custom_dotenv = cwd.join("custom/.env");

    if !default_dotenv.exists() {
        println!(
            "Creating default '{}' file",
            default_dotenv.file_stem().unwrap().display()
        );
        let mut dotenv_file = File::create(default_dotenv)?;
        writeln!(dotenv_file, "FIRST_VARIABLE=\"First Variable\"")?;
    }

    if !custom_dotenv.exists() {
        println!(
            "Creating custom '{}' file",
            custom_dotenv.strip_prefix(&cwd).unwrap().display()
        );

        if let Some(parent) = custom_dotenv.parent() {
            fs::create_dir_all(parent)?;
        }

        let mut dotenv_file = File::create(&custom_dotenv)?;
        writeln!(dotenv_file, "SECOND_VARIABLE=\"Second Variable\"")?;
    }

    // Loads .env on working directory
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

    let mut table = Table::new();

    table
        .load_preset(UTF8_FULL)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_header(vec!["DOTENV FILE", "KEY", "VALUE"])
        .add_row(vec![
            format!("{}", cwd.join(".env").display()),
            format!("{}", "FIRST_VARIABLE"),
            format!("{}", first_variable),
        ])
        .add_row(vec![
            format!("{}", custom_dotenv.display()),
            format!("{}", "SECOND VARIABLE"),
            format!("{}", second_variable),
        ]);

    println!("{table}");

    Ok(())
}
