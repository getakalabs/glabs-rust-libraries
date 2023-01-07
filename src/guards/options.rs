use sanitizer::prelude::*;
use serde::{Serialize, Deserialize};
use std::default::Default;

/// Struct container for option
#[derive(Debug, Clone, PartialEq, Sanitize, Serialize, Deserialize)]
pub struct Options {
    #[sanitize(trim)]
    pub token: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roles: Option<Vec<String>>,
    pub json_response: bool,
    pub is_optional: bool,
    pub is_refresh_token: bool,
    pub is_web_token: bool,
}

/// Default implementation for Options
impl Default for Options {
    fn default() -> Self {
        Self {
            token: String::default(),
            roles: None,
            json_response: false,
            is_optional: false,
            is_refresh_token: false,
            is_web_token: false,
        }
    }
}

/// Options implementation
impl Options {
    /// Create new Options instance
    ///
    /// Example
    /// ```
    /// use library::guards::Options;
    ///
    /// fn main() {
    ///     let option = Options::new();
    /// }
    /// ```
    pub fn new() -> Self {
        Self::default()
    }

    /// Convert custom struct type to Options
    ///
    /// Example
    /// ```
    /// use library::guards::Options;
    /// use serde::{Serialize, Deserialize};
    ///
    /// #[derive(Debug, Clone, Serialize, Deserialize)]
    /// pub struct Options2 {
    ///     #[serde(skip_serializing_if = "String::is_empty")]
    ///     pub token: String,
    ///     pub is_optional: bool,
    /// }
    ///
    /// fn main() {
    ///     let option = Options::from(Options2{token: String::from("My token"), is_optional: true});
    /// }
    /// ```
    #[allow(dead_code)]
    pub fn from<T>(input: T) -> Self
        where T: Serialize
    {
        let s = serde_json::to_string(&input).unwrap_or(String::default());
        serde_json::from_str(&s).unwrap_or(Options::default())
    }

    /// Convert custom struct type to Options
    ///
    /// Example
    /// ```
    /// use library::guards::Options;
    ///
    /// fn main() {
    ///     let input = r#"{"token": "ABC1234", "is_optional":true}"#;
    ///     let option = Options::from_string(input);
    /// }
    /// ```
    #[allow(dead_code)]
    pub fn from_string<T: Into<String>>(input: T) -> Self {
        let bindings = input.into();
        serde_json::from_str(&bindings).unwrap_or(Options::default())
    }

    /// Convert custom struct type from Options to T
    ///
    /// Example
    /// ```
    /// use library::guards::Options;
    /// use serde::{Serialize, Deserialize};
    ///
    /// /// Struct container for Options2
    /// #[derive(Debug, Clone, Serialize, Deserialize)]
    /// pub struct Options2 {
    ///     #[serde(skip_serializing_if = "String::is_empty")]
    ///     pub token: String,
    ///     pub is_optional: bool
    /// }
    ///
    /// /// Note: this is always required. Default implementation for Options
    /// impl Default for Options2 {
    ///     fn default() -> Self {
    ///         Self {
    ///             token: String::default(),
    ///             is_optional: true
    ///         }
    ///     }
    /// }
    ///
    /// fn main() {
    ///     let input = r#"{"token": "ABC1234", "is_optional":true}"#;
    ///     let option = Options::from_string(input);
    ///
    ///     let option2 = option.to::<Options2>();
    /// }
    /// ```
    #[allow(dead_code)]
    pub fn to<T>(&self) -> T
        where T: serde::de::DeserializeOwned + Default
    {
        let value = self.clone();
        let s = serde_json::to_string(&value).unwrap_or(String::default());
        let response:T = serde_json::from_str(&s).unwrap_or(T::default());

        response
    }

    /// Check if options has no value
    ///
    /// Example
    /// ```
    /// // Import option
    /// use library::guards::Options;
    ///
    /// fn main() {
    ///     // Set option
    ///     let option = Options::new();
    ///     let is_empty = option.is_empty();
    /// }
    /// ```
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.clone() == Self::default()
    }

    /// Normalize option by performing sanitation and other important stuff
    ///
    /// Example
    /// ```
    /// // Import option
    /// use library::guards::Options;
    ///
    /// fn main() {
    ///     // Set option
    ///     let mut option = Options::new();
    ///     let form = option.normalize();
    /// }
    /// ```
    #[allow(dead_code)]
    pub fn normalize(&mut self) -> &mut Self {
        self.sanitize();
        self
    }
}
