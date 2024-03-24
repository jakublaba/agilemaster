use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io;
use std::io::Write;

use serde::de::Error;

use crate::cli::cli::Cli;
use crate::gen::history_entry_gen::HistoryEntryGenerator;
use crate::gen::issue_gen::IssueGenerator;
use crate::gen::project_gen::ProjectGenerator;

pub(crate) mod issue_gen;
pub(crate) mod date_gen;
pub(crate) mod export_gen;
pub(crate) mod history_entry_gen;
pub(crate) mod project_gen;

pub(crate) trait Generator<T> {
    fn generate(&mut self) -> T;
}

#[derive(Debug)]
pub(crate) struct AgileMasterError;

impl Display for AgileMasterError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "AgileMaster has encountered an error")
    }
}

impl std::error::Error for AgileMasterError {}

pub fn generate_json(project_name: &String, args: &Cli) -> Result<(), AgileMasterError> {
    let statuses = vec![
        String::from("TO DO"),
        String::from("IN PROGRESS"),
        String::from("DONE"),
    ];
    let hist_entry_gen = &mut HistoryEntryGenerator::new(args, &statuses)?;
    let issue_gen = &mut IssueGenerator::new(args, hist_entry_gen, &statuses)?;
    let mut proj_gen = ProjectGenerator::new(args, issue_gen);

    let project = proj_gen.generate();
    let json = serde_json::to_string(&project).map_err(|_| AgileMasterError)?;

    save_to_file(format!("{project_name}.json"), json).map_err(|_| AgileMasterError)
}

fn save_to_file(file_name: String, contents: String) -> Result<(), io::Error> {
    let mut file = File::create(file_name)?;
    file.write_all(contents.as_bytes())?;

    Ok(())
}
