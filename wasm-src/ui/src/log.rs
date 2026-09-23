//! Engine log lines under the `mycube-ui` section.

const SECTION: &str = "mycube-ui";
/// Spring's `LOG_LEVEL_INFO`.
const INFO: i32 = 30;
/// Spring's `LOG_LEVEL_WARNING`.
const WARNING: i32 = 40;

pub fn info(message: &str) {
    let _ = spring::log(SECTION, INFO, message);
}

pub fn warning(message: &str) {
    let _ = spring::log(SECTION, WARNING, message);
}
