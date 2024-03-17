use std::fs::File;
use std::io;
use std::io::Write;

use crate::cli::cli::Cli;

pub(crate) mod issue_gen;
pub(crate) mod date_gen;
pub(crate) mod export_gen;
pub(crate) mod history_entry_gen;
pub(crate) mod history_item_gen;
pub(crate) mod project_gen;
pub(crate) mod user_gen;

// todo fix lifetimes and maybe wrap return type in Option or Result
pub(crate) trait Generator<T> {
    fn next(&self) -> T;
}

pub fn generate_json(args: &Cli) -> Result<(), io::Error> {
    // todo Extracts args from CLI and generates result json file
    Ok(())
}

fn save_to_file(name: &'static str, contents: &'static str) -> Result<(), io::Error> {
    let mut file = File::create(name)?;
    file.write_all(contents.as_bytes())?;

    Ok(())
}
