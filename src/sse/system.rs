use sanitizer::prelude::*;
use serde::{Serialize, Deserialize};
use std::default::Default;

/// Struct container for action
#[derive(Debug, Clone, PartialEq, Sanitize, Serialize, Deserialize)]
pub struct System {
    #[sanitize(trim)]
    #[serde(skip_serializing_if = "String::is_empty")]
    pub ping: String,
    #[sanitize(trim)]
    #[serde(skip_serializing_if = "String::is_empty")]
    pub action: String,
    #[sanitize(trim)]
    #[serde(skip_serializing_if = "String::is_empty")]
    pub module: String,
    #[sanitize(trim)]
    #[serde(skip_serializing_if = "String::is_empty")]
    pub content: String,
    #[sanitize(trim)]
    #[serde(skip_serializing_if = "String::is_empty")]
    pub event: String,
}

/// Default implementation for System
impl Default for System {
    fn default() -> Self {
        Self {
            ping: String::default(),
            action: String::default(),
            module: String::default(),
            content: String::default(),
            event: String::default(),
        }
    }
}

/// System implementation
impl System {
    /// Create new System instance
    ///
    /// Example
    /// ```
    /// use library::sse::System;
    ///
    /// fn main() {
    ///     let action = System::new();
    /// }
    /// ```
    pub fn new() -> Self {
        Self {
            ping: String::from("ping"),
            action: String::from("connection"),
            module: String::from("SSE"),
            content: String::from("Successfully Connected"),
            event: String::from("message"),
        }
    }

    /// Convert custom struct type to System
    ///
    /// Example
    /// ```
    /// use library::sse::System;
    /// use serde::{Serialize, Deserialize};
    ///
    /// #[derive(Debug, Clone, Serialize, Deserialize)]
    /// pub struct System2 {
    ///     #[serde(skip_serializing_if = "String::is_empty")]
    ///     pub ping: String,
    /// 	#[serde(skip_serializing_if = "String::is_empty")]
    ///     pub module: String,
    /// }
    ///
    /// fn main() {
    ///     let action = System::from(System2{ping: String::from("ping"), module: String::default()});
    /// }
    /// ```
    #[allow(dead_code)]
    pub fn from<T>(input: T) -> Self
        where T: Serialize
    {
        let s = serde_json::to_string(&input).unwrap_or(String::default());
        serde_json::from_str(&s).unwrap_or(System::default())
    }

    /// Convert custom struct type to System
    ///
    /// Example
    /// ```
    /// use library::sse::System;
    ///
    /// fn main() {
    ///     let input = r#"{"ping": "ping", "module": "sse"}"#;
    ///     let action = System::from_string(input);
    /// }
    /// ```
    #[allow(dead_code)]
    pub fn from_string<T: Into<String>>(input: T) -> Self {
        let bindings = input.into();
        serde_json::from_str(&bindings).unwrap_or(System::default())
    }

    /// Convert custom struct type from System to T
    ///
    /// Example
    /// ```
    /// use library::sse::System;
    /// use serde::{Serialize, Deserialize};
    ///
    /// /// Struct container for System2
    /// #[derive(Debug, Clone, Serialize, Deserialize)]
    /// pub struct System2 {
    ///     #[serde(skip_serializing_if = "String::is_empty")]
    ///     pub ping: String,
    ///     #[serde(skip_serializing_if = "String::is_empty")]
    ///     pub module: String,
    /// }
    ///
    /// /// Note: this is always required. Default implementation for System
    /// impl Default for System2 {
    ///     fn default() -> Self {
    ///         Self {
    ///             ping: String::default(),
    ///             module: String::default()
    ///         }
    ///     }
    /// }
    ///
    /// fn main() {
    ///     let input = r#"{"ping": "ping", "module": "sse"}"#;
    ///     let action = System::from_string(input);
    ///
    ///     let action2 = action.to::<System2>();
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

    /// Check if actions has no value
    ///
    /// Example
    /// ```
    /// // Import action
    /// use library::sse::System;
    ///
    /// fn main() {
    ///     // Set action
    ///     let action = System::new();
    ///     let is_empty = action.is_empty();
    /// }
    /// ```
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.clone() == Self::default()
    }

    /// Normalize action by performing sanitation and other important stuff
    ///
    /// Example
    /// ```
    /// // Import action
    /// use library::sse::System;
    ///
    /// fn main() {
    ///     // Set action
    ///     let mut action = System::new();
    ///     let form = action.normalize();
    /// }
    /// ```
    #[allow(dead_code)]
    pub fn normalize(&mut self) -> &mut Self {
        self.sanitize();
        self
    }
}
