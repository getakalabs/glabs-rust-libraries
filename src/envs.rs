use std::fs;
use std::io::{self, BufRead};

/// Silently load an environment variable file. This won't panic if the file wasn't found. Used in development mode.
///
/// Example:
/// ```
/// use library::envs;
///
/// fn main() {
///     envs::load(".env");
/// }
/// ```
pub fn load<T: Into<String>>(path: T) {
    // Create bindings
    let bindings = path.into();

    // Open env file
    let file = fs::File::open(&bindings.to_lowercase());
    if file.is_ok() {
        // Read the file line by line
        let reader = io::BufReader::new(file.unwrap());
        for line in reader.lines() {
            // Unwrap each line
            let line = line.unwrap_or(String::default());

            // Check if line is empty
            if !line.trim().is_empty() {
                // Split the line on the '=' character
                let mut parts = line.trim().split('=');
                let key = parts.next().unwrap();
                let value = parts.next().unwrap();

                // Set the environment variable
                std::env::set_var(key, value);
            }
        }
    }
}


/// Retrieves an environment variable based on key
///
/// Example:
/// ```
/// use library::envs;
///
/// fn main() {
///     envs::load(".env");
///
///     println!("{:?}", envs::get("HOME"));
/// }
/// ```
pub fn get<T: Into<String>>(key: T) -> String {
    // Create bindings
    let bindings = key.into();

    // Return a `String` value
    std::env::var(&bindings).unwrap_or(String::default())
}