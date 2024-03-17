use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

use chrono::{DateTime, Duration, Utc};
use rand::{Rng, thread_rng};
use rand::rngs::ThreadRng;

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

pub(crate) struct DateGenerator {
    start_date: DateTime<Utc>,
    end_date: DateTime<Utc>,
    range: i64,
    rng: ThreadRng,
}

impl DateGenerator {
    pub fn new(start_date: DateTime<Utc>, end_date: DateTime<Utc>) -> Result<Self, DateGeneratorError> {
        if start_date >= end_date {
            return Err(DateGeneratorError::new("end_date must be after start_date"));
        }
        let range = (end_date - start_date).num_days();
        let rng = thread_rng();
        Ok(Self { start_date, end_date, range, rng })
    }

    pub fn next(&mut self) -> DateTime<Utc> {
        let days = self.rng.gen_range(0..=self.range);
        self.start_date + Duration::days(days)
    }

    pub fn gen_after(&mut self, date: DateTime<Utc>) -> Result<DateTime<Utc>, DateGeneratorError> {
        if date <= self.start_date {
            return Err(DateGeneratorError::new("date must be after start_date"));
        }
        let range = (self.end_date - date).num_days();
        let days = self.rng.gen_range(0..=range);
        Ok(self.start_date + Duration::days(days))
    }
}
