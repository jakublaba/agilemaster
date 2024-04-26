use std::error::Error;

use clap::Parser;

use crate::cli::cli::Cli;
use crate::gen::generate_json;

mod model;
mod cli;
mod gen;

#[tokio::main]
async fn main() -> Result<(), impl Error> {
    let args = Cli::parse();
    dbg!(&args);

    generate_json(&args).await
}
