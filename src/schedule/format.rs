use std::time::Duration;

pub fn dur_to_xhxmxs(dur: Duration) -> String {
    let mut secs = dur.as_secs();

    let mut mins = secs / 60;
    secs %= 60;

    let hours = mins / 60;
    mins %= 60;

    let secs_str = if secs == 0 {String::new()} else {format!("{secs}s")};
    let mins_str = if mins == 0 {String::new()} else {format!("{mins}m")};
    let hours_str = if hours == 0 {String::new()} else {format!("{hours}h")};

    format!("{hours_str}{mins_str}{secs_str}")
}

pub fn dur_to_hhmmss(dur: Duration) -> String {
    let mut secs = dur.as_secs();

    let mut mins = secs / 60;
    secs %= 60;

    let hours = mins / 60;
    mins %= 60;

    let secs_str = {
        if secs < 10 {
            format!("0{secs}")
        } else {
            format!("{secs}")
        }
    };

    let mins_str = {
        if mins < 10 {
            format!("0{mins}:")
        } else {
            format!("{mins}:")
        }
    };

    let hours_str = {
        if hours == 0 {
            String::new()
        } else {
            format!("{hours}:")
        }
    };

    format!("{hours_str}{mins_str}{secs_str}")
}

/// Only use if you're 100% sure that your string is 100% able to be converted into a Duration.
pub fn hhmmss_to_dur(str: &str) -> Duration {
    let mut secs = 0u64;
    
    for (s, i) in str.split(':').rev().zip(0u32..) {
        let parsed: u64 = s.parse().unwrap();

        secs += parsed * 60u64.pow(i);
    }

    Duration::from_secs(secs)
}

pub fn try_hhmmss_to_dur(str: &str) -> Option<Duration> {
    let mut secs = 0u64;
    
    for (s, i) in str.split(':').rev().zip(0u32..) {
        if i > 2 {
            return None
        }

        let parsed: u64 = s.parse().ok()?;

        if parsed > 60 {
            if i == 0 || i == 1 {
                return None
            }
        }

        secs += parsed * 60u64.pow(i);
    }

    Some(Duration::from_secs(secs))
}