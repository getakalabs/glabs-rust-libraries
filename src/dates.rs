use chrono::{Local, Datelike, Timelike};

/// Helper function to generate the current date in the format "YYYY-MM-DD"
///
/// Example:
/// ```
/// use library::dates;
///
/// fn main() {
///     println!("{:?}", dates::get_current_date_string());
/// }
/// ```
pub fn get_current_date_string() -> String {
    let now = Local::now();
    return format!("{}-{:02}-{:02}", now.year(), now.month(), now.day());
}

/// Helper function to generate the current time in the format "HH:ii:ss AM/PM"
///
/// Example:
/// ```
/// use library::dates;
///
/// fn main() {
///     println!("{:?}", dates::get_current_time_string());
/// }
/// ```
pub fn get_current_time_string() -> String {
    let now = Local::now();
    let hour = now.hour() % 12;
    let am_pm = if now.hour() >= 12 { "PM" } else { "AM" };
    return format!("{:02}:{:02}:{:02} {}", hour, now.minute(), now.second(), am_pm);
}