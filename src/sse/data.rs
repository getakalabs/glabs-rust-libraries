use sanitizer::prelude::*;
use serde::{Serialize, Deserialize};
use std::default::Default;

/// Struct container for content
#[derive(Debug, Clone, PartialEq, Sanitize, Serialize, Deserialize)]
pub struct Data {
    #[sanitize(trim)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    #[sanitize(trim)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[sanitize(trim)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub module: Option<String>,
}

/// Default implementation for Data
impl Default for Data {
    fn default() -> Self {
        Self {
            action: None,
            content: None,
            module: None,
        }
    }
}

/// Data implementation
impl Data {
    /// Create new Data instance
    ///
    /// Example
    /// ```
    /// use library::sse::Data;
    ///
    /// fn main() {
    ///     let content = Data::new();
    /// }
    /// ```
    pub fn new() -> Self {
        Self::default()
    }

    /// Convert custom struct type to Data
    ///
    /// Example
    /// ```
    /// use library::sse::Data;
    /// use serde::{Serialize, Deserialize};
    ///
    /// #[derive(Debug, Clone, Serialize, Deserialize)]
    /// pub struct Data2 {
    ///     #[serde(skip_serializing_if = "Option::is_none")]
    ///     pub action: Option<String>,
    /// 	#[serde(skip_serializing_if = "Option::is_none")]
    ///     pub module: Option<String>,
    /// }
    ///
    /// fn main() {
    ///     let content = Data::from(Data2{action: Some(String::from("update")), module: None});
    /// }
    /// ```
    #[allow(dead_code)]
    pub fn from<T>(input: T) -> Self
        where T: Serialize
    {
        let s = serde_json::to_string(&input).unwrap_or(String::default());
        serde_json::from_str(&s).unwrap_or(Data::default())
    }

    /// Convert custom struct type to Data
    ///
    /// Example
    /// ```
    /// use library::sse::Data;
    ///
    /// fn main() {
    ///     let input = r#"{"action": "update", "module": "images"}"#;
    ///     let content = Data::from_string(input);
    /// }
    /// ```
    #[allow(dead_code)]
    pub fn from_string<T: Into<String>>(input: T) -> Self {
        let bindings = input.into();
        serde_json::from_str(&bindings).unwrap_or(Data::default())
    }

    /// Convert custom struct type from Data to T
    ///
    /// Example
    /// ```
    /// use library::sse::Data;
    /// use serde::{Serialize, Deserialize};
    ///
    /// /// Struct container for Data2
    /// #[derive(Debug, Clone, Serialize, Deserialize)]
    /// pub struct Data2 {
    ///     #[serde(skip_serializing_if = "Option::is_none")]
    ///     pub action: Option<String>,
    ///     #[serde(skip_serializing_if = "Option::is_none")]
    ///     pub module: Option<String>,
    /// }
    ///
    /// /// Note: this is always required. Default implementation for Data
    /// impl Default for Data2 {
    ///     fn default() -> Self {
    ///         Self {
    ///             action: None,
    ///             module: None
    ///         }
    ///     }
    /// }
    ///
    /// fn main() {
    ///     let input = r#"{"action": "update", "module": "images"}"#;
    ///     let content = Data::from_string(input);
    ///
    ///     let content2 = content.to::<Data2>();
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

    /// Check if contents has no value
    ///
    /// Example
    /// ```
    /// // Import content
    /// use library::sse::Data;
    ///
    /// fn main() {
    ///     // Set content
    ///     let content = Data::new();
    ///     let is_empty = content.is_empty();
    /// }
    /// ```
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.clone() == Self::default()
    }

    /// Normalize content by performing sanitation and other important stuff
    ///
    /// Example
    /// ```
    /// // Import content
    /// use library::sse::Data;
    ///
    /// fn main() {
    ///     // Set content
    ///     let mut content = Data::new();
    ///     let form = content.normalize();
    /// }
    /// ```
    #[allow(dead_code)]
    pub fn normalize(&mut self) -> &mut Self {
        self.sanitize();
        self
    }
}
