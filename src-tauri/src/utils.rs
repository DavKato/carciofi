use std::time::{SystemTime, UNIX_EPOCH};

#[allow(dead_code)]
pub fn get_current_time() -> String {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis()
        .to_string()
}
