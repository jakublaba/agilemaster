use chrono::{DateTime, Duration, Utc};
use rand::{Rng, thread_rng};
use rand::rngs::ThreadRng;

pub(crate) struct DateGenerator {
    start_date: DateTime<Utc>,
    range: i64,
    rng: ThreadRng,
}

// todo add error handling for when dates are chronologically incorrect
impl DateGenerator {
    pub(crate) fn new(start_date: DateTime<Utc>, end_date: DateTime<Utc>) -> Self {
        let range = (end_date - start_date).num_days();
        let rng = thread_rng();
        Self { start_date, range, rng }
    }

    pub(crate) fn next(&mut self) -> DateTime<Utc> {
        let days = self.rng.gen_range(0..=self.range);
        self.start_date + Duration::days(days)
    }

    pub(crate) fn gen_after(&mut self, date: DateTime<Utc>) -> DateTime<Utc> {
        let range = (date - self.start_date).num_days();
        let days = self.rng.gen_range(0..=range);
        self.start_date + Duration::days(days)
    }
}
