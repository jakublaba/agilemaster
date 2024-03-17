use std::error::Error;
use std::io::Write;

use clap::Parser;

use crate::cli::cli::Cli;

mod model;
mod cli;
mod gen;

// todo add loggers
fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::parse();

    println!("{:#?}", args);

    Ok(())
}
