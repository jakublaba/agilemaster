use chrono::{NaiveDate, ParseResult};
use clap::Parser;

#[derive(Debug, Parser)]
#[command(version, author, about, long_about = None)]
pub(crate) struct Cli {
    /// Name of the generated project
    #[arg(short, long, value_name = "NAME")]
    name: String,
    /// Start date of the project
    #[arg(short, long, value_name = "START DATE", value_parser = parse_date)]
    start: NaiveDate,
    /// End date of the project
    #[arg(short, long, value_name = "END DATE", value_parser = parse_date)]
    end: NaiveDate,
    /// Amount of issues to generate
    #[arg(short, long, value_name = "ISSUE AMOUNT")]
    issue_amount: i32,
}

fn parse_date(arg: &str) -> ParseResult<NaiveDate> {
    let date = NaiveDate::parse_from_str(arg, "%d-%m-%Y")?;
    Ok(date)
}
