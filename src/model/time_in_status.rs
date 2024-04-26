use std::collections::HashMap;

#[derive(Debug)]
pub struct TimeInStatus {
    // key - status, value - how many times in status + total time in status (ms)
    times: HashMap<String, (i32, i32)>,
}