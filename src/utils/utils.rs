use std::time::{SystemTime, UNIX_EPOCH, Duration};
use chrono::prelude::*;
use std::convert::TryInto;


pub fn print_timestamp(stamp: u64) {
    println!("stamp {}", stamp);
    // Creates a new SystemTime from the specified number of whole seconds
    let d = UNIX_EPOCH + Duration::from_nanos(stamp);
    // Create DateTime from SystemTime
    let datetime = DateTime::<Utc>::from(d);
    // Formats the combined date and time with the specified format string.
    let timestamp_str = datetime.format("%Y-%m-%d %H:%M:%S.%f").to_string();
    println! {"{}", timestamp_str};
}

pub fn get_timestamp() -> u64 {
    let start = SystemTime::now();
    let since_the_epoch = start.duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    let in_ms = since_the_epoch.as_secs() * 1_000_000_000 +
        since_the_epoch.subsec_nanos() as u64;
    in_ms
}

pub fn to_static8(barry: &[u8]) -> [u8; 8] {
    println!("{:?}", barry);
    barry.try_into().expect("slice with incorrect length")
}
