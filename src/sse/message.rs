use sanitizer::prelude::*;
use serde::{Serialize, Deserialize};
use std::default::Default;

/// Struct container for message
#[derive(Debug, Clone, PartialEq, Sanitize, Serialize, Deserialize)]
pub struct Message {
    #[sanitize(trim)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<super::Data>,
    #[sanitize(trim)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event: Option<String>,
}

/// Default implementation for Message
impl Default for Message {
    fn default() -> Self {
        Self {
            channel: None,
            data: None,
            event: None,
        }
    }
}

/// Message implementation
impl Message {
    /// Create new Message instance
    ///
    /// Example
    /// ```
    /// use library::sse::Message;
    ///
    /// fn main() {
    ///     let message = Message::new();
    /// }
    /// ```
    pub fn new() -> Self {
        Self::default()
    }

    /// Convert custom struct type to Message
    ///
    /// Example
    /// ```
    /// use library::sse::Message;
    /// use serde::{Serialize, Deserialize};
    ///
    /// #[derive(Debug, Clone, Serialize, Deserialize)]
    /// pub struct Message2 {
    ///     #[serde(skip_serializing_if = "Option::is_none")]
    ///     pub channel: Option<String>,
    /// 	#[serde(skip_serializing_if = "Option::is_none")]
    ///     pub event: Option<String>,
    /// }
    ///
    /// fn main() {
    ///     let message = Message::from(Message2{channel: Some(String::from("ch1")), event: None});
    /// }
    /// ```
    #[allow(dead_code)]
    pub fn from<T>(input: T) -> Self
        where T: Serialize
    {
        let s = serde_json::to_string(&input).unwrap_or(String::default());
        serde_json::from_str(&s).unwrap_or(Message::default())
    }

    /// Convert custom struct type to Message
    ///
    /// Example
    /// ```
    /// use library::sse::Message;
    ///
    /// fn main() {
    ///     let input = r#"{"channel": "ch1", "event": "message sent"}"#;
    ///     let message = Message::from_string(input);
    /// }
    /// ```
    #[allow(dead_code)]
    pub fn from_string<T: Into<String>>(input: T) -> Self {
        let bindings = input.into();
        serde_json::from_str(&bindings).unwrap_or(Message::default())
    }

    /// Convert custom struct type from Message to T
    ///
    /// Example
    /// ```
    /// use library::sse::Message;
    /// use serde::{Serialize, Deserialize};
    ///
    /// /// Struct container for Message2
    /// #[derive(Debug, Clone, Serialize, Deserialize)]
    /// pub struct Message2 {
    ///     #[serde(skip_serializing_if = "Option::is_none")]
    ///     pub channel: Option<String>,
    ///     #[serde(skip_serializing_if = "Option::is_none")]
    ///     pub event: Option<String>,
    /// }
    ///
    /// /// Note: this is always required. Default implementation for Message
    /// impl Default for Message2 {
    ///     fn default() -> Self {
    ///         Self {
    ///             channel: None,
    ///             event: None
    ///         }
    ///     }
    /// }
    ///
    /// fn main() {
    ///     let input = r#"{"channel": "ch1", "event": "message sent"}"#;
    ///     let message = Message::from_string(input);
    ///
    ///     let message2 = message.to::<Message2>();
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

    /// Check if messages has no value
    ///
    /// Example
    /// ```
    /// // Import message
    /// use library::sse::Message;
    ///
    /// fn main() {
    ///     // Set message
    ///     let message = Message::new();
    ///     let is_empty = message.is_empty();
    /// }
    /// ```
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.clone() == Self::default()
    }

    /// Normalize message by performing sanitation and other important stuff
    ///
    /// Example
    /// ```
    /// // Import message
    /// use library::sse::Message;
    ///
    /// fn main() {
    ///     // Set message
    ///     let mut message = Message::new();
    ///     let form = message.normalize();
    /// }
    /// ```
    #[allow(dead_code)]
    pub fn normalize(&mut self) -> &mut Self {
        self.sanitize();

        if self.data.is_some() {
            let mut data = self.data.clone()
                .unwrap_or(super::Data::default());

            data.sanitize();

            self.data = Some(data);
        }

        self
    }
}
