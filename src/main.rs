use std::error::Error;

use clap::Parser;

use crate::cli::cli::Cli;
use crate::gen::generate_json;

mod model;
mod cli;
mod gen;

fn main() -> Result<(), impl Error> {
    let args = Cli::parse();
    dbg!(&args);

    generate_json(&args)
}
