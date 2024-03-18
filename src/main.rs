use std::error::Error;
use std::fmt::Display;
use std::io::Write;

use clap::Parser;

use crate::cli::cli::Cli;
use crate::gen::generate_json;

mod model;
mod cli;
mod gen;

// todo add loggers
fn main() -> Result<(), impl Error> {
    let args = Cli::parse();
    dbg!(&args);

    generate_json(String::from("export.json"), &args)
}
