use std::time::{SystemTime, UNIX_EPOCH};

// TODO: add random component
// TODO: make struct?
/// Generate a unique snowflake-like tag,
/// of the format: system time nanos (16 bytes) + random (16 bytes)
pub fn tag() -> Vec<u8> {
    let mut snowflake = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Couldn't find the duration since the epoch")
        .as_nanos()
        .to_be_bytes()
        .to_vec();
    return snowflake;
}
