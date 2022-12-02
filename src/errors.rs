use std::error::Error;
use std::fmt::Display;

/// Errors enum consists of Message(String) type
#[derive(Debug, Clone)]
pub enum Errors {
    Message(String)
}

/// Display implementation for Errors
impl Display for Errors {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt.write_str(self.to_string().as_str())
    }
}

/// Error implementation for Errors
impl Error for Errors {}

/// Errors implementation
impl Errors {
    /// Create new error instance
    ///
    /// Example
    /// ```
    /// use library::Errors;
    ///
    /// fn main() {
    ///     let error = Errors::new("My new error");
    /// }
    /// ```
    pub fn new<T: Into<String>>(str: T) -> Self {
        Self::Message(str.into())
    }

    /// Convert error to &str type
    ///
    /// Example
    /// ```
    /// use library::Errors;
    ///
    /// fn main() {
    ///     println!("{:?}", Errors::new("sample error").as_str());
    /// }
    /// ```
    pub fn as_str(&self) -> &str {
        return match self {
            Errors::Message(value) => value,
        }
    }

    /// Convert error to String type
    ///
    /// Example
    /// ```
    /// use library::Errors;
    ///
    /// fn main() {
    ///     println!("{:?}", Errors::new("sample error").to_string());
    /// }
    /// ```
    pub fn to_string(&self) -> String {
        return match self {
            Errors::Message(value) => String::from(value),
        }
    }
}

