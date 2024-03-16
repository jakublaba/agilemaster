use chrono::{DateTime, FixedOffset, NaiveDateTime, ParseResult, TimeZone};
use clap::Parser;

#[derive(Debug, Parser)]
#[command(version, author, about, long_about = None)]
pub(crate) struct Cli {
    /// Name of the generated project
    #[arg(short, long, value_name = "NAME")]
    pub(crate) name: String,
    /// Start date of the project
    #[arg(short, long, value_name = "DATE", value_parser = parse_date)]
    pub(crate) start: DateTime<FixedOffset>,
    /// End date of the project
    #[arg(short, long, value_name = "DATE", value_parser = parse_date)]
    pub(crate) end: DateTime<FixedOffset>,
    /// Amount of issues to generate
    #[arg(short, long, value_name = "AMOUNT")]
    pub(crate) issue_amount: i32,
}

fn parse_date(arg: &str) -> ParseResult<DateTime<FixedOffset>> {
    let s = &format!("{arg} 00:00:00");
    let naive = NaiveDateTime::parse_from_str(s, "%d-%m-%Y %H:%M:%S")?;
    let Some(offset) = FixedOffset::east_opt(0) else {
        panic!("Cannot parse date from cli")
    };

    Ok(offset.from_utc_datetime(&naive))
}
