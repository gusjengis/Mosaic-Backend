use chrono::{Local, Timelike};

#[cfg(feature = "debug")]
pub fn debug(message: String) {
    let now = Local::now();
    let hour = now.hour();
    let am_pm = if hour < 12 { "a" } else { "p" };
    let formatted_time = now.format("%Y-%m-%d %I:%M");
    println!("{}{}: {}", formatted_time, am_pm, message);
}

#[cfg(not(feature = "debug"))]
pub fn debug(_message: String) {
    // Do nothing when debug feature is not enabled
}
