use std::time::{Duration, Instant};

pub struct TimedResult<T> {
    pub result: T,
    pub duration: Duration,
}

pub fn time_execution<F, T>(func: F) -> TimedResult<T>
where
    F: FnOnce() -> T,
{
    let start = Instant::now();
    let result = func();
    let duration = start.elapsed();
    TimedResult { result, duration }
}
