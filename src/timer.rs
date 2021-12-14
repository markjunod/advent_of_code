use std::time::{SystemTime, UNIX_EPOCH};

pub fn time_millis<F>(f: F) -> u64 where F: FnOnce() {
    let nanos = time_nanos(f);

    nanos / 1_000_000
}

pub fn time_nanos<F>(f: F) -> u64 where F: FnOnce() {
    let start = current_nanos();

    f();

    current_nanos() - start
}

fn current_nanos() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().subsec_nanos() as u64
}
