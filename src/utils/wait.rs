use std::{time::Duration, thread};

pub fn for_secs(t: u64) {
    thread::sleep(Duration::from_secs(t));
}

pub fn for_ms(t: u64) {
    thread::sleep(Duration::from_millis(t));
}