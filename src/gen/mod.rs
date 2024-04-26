use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io;
use std::io::Write;

use crate::cli::cli::Cli;
use crate::gen::export_gen::ExportGenerator;
use crate::gen::history_entry_gen::HistoryEntryGenerator;
use crate::gen::issue_gen::IssueGenerator;
use crate::gen::project_gen::ProjectGenerator;

pub(crate) mod date_gen;
pub(crate) mod export_gen;
pub(crate) mod history_entry_gen;
pub(crate) mod issue_gen;
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

pub async fn generate_json(args: &Cli) -> Result<(), AgileMasterError> {
    let mut hist_entry_gen = HistoryEntryGenerator::new(args, &args.statuses)?;
    let mut issue_gen = IssueGenerator::new(args, &mut hist_entry_gen).await?;
    let mut proj_gen = ProjectGenerator::new(args, &mut issue_gen);
    let mut export_gen = ExportGenerator::new(args.author.clone(), &mut proj_gen);

    let export = export_gen.generate();
    let json = serde_json::to_string(&export).map_err(|_| AgileMasterError)?;

    let project_name = &args.name;
    save_to_file(format!("{project_name}.json"), json).map_err(|_| AgileMasterError)
}

fn save_to_file(file_name: String, contents: String) -> Result<(), io::Error> {
    let mut file = File::create(file_name)?;
    file.write_all(contents.as_bytes())?;

    Ok(())
}
