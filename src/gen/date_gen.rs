use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

use chrono::{DateTime, TimeDelta, Utc};
use rand::{Rng, thread_rng};
use rand::rngs::ThreadRng;

use crate::cli::cli::Cli;

#[derive(Debug)]
pub(crate) struct DateGeneratorError {
    message: String,
}

impl DateGeneratorError {
    pub fn new(msg: &str) -> Self {
        Self { message: String::from(msg) }
    }
}

impl Display for DateGeneratorError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for DateGeneratorError {}

#[derive(Debug, Clone)]
pub(crate) struct DateGenerator {
    start_date: DateTime<Utc>,
    end_date: DateTime<Utc>,
    range: i64,
    rng: ThreadRng,
}

impl DateGenerator {
    pub fn new(cli_args: &Cli) -> Result<Self, DateGeneratorError> {
        let start_date = cli_args.start;
        let end_date = cli_args.end;
        if start_date >= end_date {
            let msg = format!("end_date must be after start_date, was end_date={end_date}, start_date={start_date}");
            return Err(DateGeneratorError::new(&msg));
        }
        let range = (end_date - start_date).num_days();
        let rng = thread_rng();
        Ok(Self { start_date, end_date, range, rng })
    }

    pub fn next(&mut self) -> DateTime<Utc> {
        let days = self.rng.gen_range(1..self.range);
        self.start_date + TimeDelta::try_days(days).unwrap()
    }

    pub fn next_after(&mut self, date: DateTime<Utc>) -> Result<DateTime<Utc>, DateGeneratorError> {
        self.validate_date(&date)?;
        let range = (self.end_date - date).num_days();
        let days = self.rng.gen_range(0..range);
        Ok(date + TimeDelta::try_days(days).unwrap())
    }

    pub fn next_before(&mut self, date: DateTime<Utc>) -> Result<DateTime<Utc>, DateGeneratorError> {
        self.validate_date(&date)?;
        let range = (date - self.start_date).num_days();
        let days = self.rng.gen_range(0..range);
        Ok(date - TimeDelta::try_days(days).unwrap())
    }

    fn validate_date(&self, date: &DateTime<Utc>) -> Result<(), DateGeneratorError> {
        let range = (self.start_date + TimeDelta::try_days(1).unwrap())..self.end_date;
        if !range.contains(date) {
            let msg = format!("date must be in inclusive range ({}, {}), was {date}", range.start, range.end - TimeDelta::try_days(1).unwrap());
            return Err(DateGeneratorError::new(&msg));
        }
        Ok(())
    }
}
