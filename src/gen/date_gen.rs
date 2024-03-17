use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

use chrono::{DateTime, Duration, Utc};
use rand::{Rng, thread_rng};
use rand::rngs::ThreadRng;

pub(crate) struct DateGenerator {
    start_date: DateTime<Utc>,
    range: i64,
    rng: ThreadRng,
}

#[derive(Debug)]
pub(crate) struct DateGeneratorError;

impl Display for DateGeneratorError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error creating DateGenerator - end_date must be later than start_date")
    }
}

impl Error for DateGeneratorError {}

impl DateGenerator {
    pub fn new(start_date: DateTime<Utc>, end_date: DateTime<Utc>) -> Result<Self, DateGeneratorError> {
        if start_date >= end_date {
            return Err(DateGeneratorError);
        }
        let range = (end_date - start_date).num_days();
        let rng = thread_rng();

        Ok(Self { start_date, range, rng })
    }

    pub fn next(&mut self) -> DateTime<Utc> {
        let days = self.rng.gen_range(0..=self.range);
        self.start_date + Duration::days(days)
    }

    pub fn gen_after(&mut self, date: DateTime<Utc>) -> DateTime<Utc> {
        let range = (date - self.start_date).num_days();
        let days = self.rng.gen_range(0..=range);
        self.start_date + Duration::days(days)
    }
}
