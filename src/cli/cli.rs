use chrono::{DateTime, NaiveDateTime, ParseResult, Utc};
use clap::Parser;

#[derive(Debug, Parser)]
#[command(version, author, about, long_about = None)]
pub(crate) struct Cli {
    /// Name of the generated project
    #[arg(short, long, value_name = "NAME")]
    pub(crate) name: String,
    /// Start date of the project
    #[arg(short, long, value_name = "DATE", value_parser = parse_date)]
    pub(crate) start: DateTime<Utc>,
    /// End date of the project
    #[arg(short, long, value_name = "DATE", value_parser = parse_date)]
    pub(crate) end: DateTime<Utc>,
    /// Amount of issues to generate
    #[arg(short, long, value_name = "AMOUNT")]
    pub(crate) issue_amount: i32,
}

fn parse_date(arg: &str) -> ParseResult<DateTime<Utc>> {
    let s = &format!("{arg} 00:00:00");
    let naive = NaiveDateTime::parse_from_str(s, "%d-%m-%Y %H:%M:%S")?;
    let utc = DateTime::<Utc>::from_naive_utc_and_offset(naive, Utc);

    Ok(utc)
}
