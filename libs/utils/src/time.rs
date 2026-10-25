use std::time::{SystemTime, UNIX_EPOCH};

/// Current Unix timestamp in seconds; fails if the clock is before the epoch.
pub fn unix_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Clock before epoch")
        .as_secs()
}
