use std::{
    error::Error,
    fmt::{Display, Formatter},
    fs::File,
    io::BufReader,
};

use chrono::{DateTime, NaiveDateTime, Utc};
use clap::{ArgAction, Parser};
use serde::de::DeserializeOwned;

use crate::model::jira_rest::AuthParams;
use crate::model::user::User;

#[derive(Debug)]
struct CliError {
    message: String,
}

impl CliError {
    pub fn new(msg: &str) -> Self {
        Self {
            message: String::from(msg),
        }
    }
}

impl Display for CliError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for CliError {}

#[derive(Debug, Parser)]
#[command(version, author, about, long_about = None)]
pub(crate) struct Cli {
    /// Name of the generated project
    #[arg(short, long)]
    pub(crate) name: String,
    /// Fully qualified name (with path) of json file with user data
    #[arg(short, long, value_name = "PATH", value_parser = parse_struct::< User >)]
    pub(crate) author: User,
    /// Fully qualified name (with path) of json file with authentication data
    #[arg(short, long, value_name = "PATH", value_parser = parse_struct::< AuthParams >)]
    pub(crate) auth_params: AuthParams,
    /// Start date of the project (dd-mm-YYYY)
    #[arg(short, long, value_name = "DATE", value_parser = parse_date)]
    pub(crate) start: DateTime<Utc>,
    /// End date of the project (dd-mm-YYYY)
    #[arg(short, long, value_name = "DATE", value_parser = parse_date)]
    pub(crate) end: DateTime<Utc>,
    /// Amount of issues to generate
    #[arg(short, long, value_name = "AMOUNT")]
    pub(crate) issue_amount: i32,
    /// Space-separated list of statuses available in project
    #[arg(short, long, value_name = "STATUSES", action = ArgAction::Append, num_args = 1..)]
    pub(crate) statuses: Vec<String>,
}

fn parse_struct<T: DeserializeOwned>(arg: &str) -> Result<T, CliError> {
    let file = File::open(arg).map_err(|e| CliError::new(&e.to_string()))?;
    let reader = BufReader::new(file);
    let parsed_struct: T = serde_json::from_reader(reader).map_err(|e| CliError::new(&e.to_string()))?;

    Ok(parsed_struct)
}

fn parse_date(arg: &str) -> Result<DateTime<Utc>, CliError> {
    let s = &format!("{arg} 00:00:00");
    let naive = NaiveDateTime::parse_from_str(s, "%d-%m-%Y %H:%M:%S")
        .map_err(|e| CliError::new(&e.to_string()))?;
    let utc = DateTime::<Utc>::from_naive_utc_and_offset(naive, Utc);

    Ok(utc)
}
