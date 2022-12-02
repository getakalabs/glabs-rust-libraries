use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::json;
use crate::Payload;

/// Create Token struct which contains access, refresh and web tokens
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Token {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web: Option<String>
}

// Implement default for Token
impl Default for Token {
    fn default() -> Self {
        Self {
            access: None,
            refresh: None,
            web: None
        }
    }
}

// Create Token implementation
impl Token {
    /// Create new token instance
    ///
    /// Example
    /// ```
    /// // Import token
    /// use library::tokens::Token;
    ///
    /// fn main() {
    ///     // Set token
    ///     let token = Token::new();
    /// }
    /// ```
    pub fn new() -> Self {
        Self::default()
    }

    /// Convert to array
    ///
    /// Example
    /// ```
    /// // Import token
    /// use library::tokens::Token;
    ///
    /// fn main() {
    ///     // Set token
    ///     let token = Token::new();
    ///     let array = token.to_array();
    /// }
    /// ```
    pub fn to_array(&self) -> [Option<String>; 3] {
        [self.clone().access, self.clone().refresh, self.clone().web]
    }

    /// Check if tokens has no value
    ///
    /// Example
    /// ```
    /// // Import token
    /// use library::tokens::Token;
    ///
    /// fn main() {
    ///     // Set token
    ///     let token = Token::new();
    ///     let is_none = token.is_empty();
    /// }
    /// ```
    pub fn is_empty(&self) -> bool {
        for item in self.to_array() {
            if item.as_ref().is_some() && !item.as_ref().unwrap().is_empty() {
                return false
            }
        }

        true
    }

    /// Normalizes fields
    ///
    /// Example
    /// ```
    /// // Import token
    /// use library::tokens::Token;
    ///
    /// fn main() {
    ///     // Set token
    ///     let token = Token::new();
    ///     let form = token.normalize();
    /// }
    /// ```
    #[allow(dead_code)]
    pub fn normalize(self) -> Self {
        json::normalize(self.clone())
    }

    /// Checks if refresh token exists
    /// Example
    /// ```
    /// // Import token
    /// use library::tokens::Token;
    ///
    /// fn main() {
    ///     // Set token
    ///     let token = Token::new();
    ///     let form = token.normalize();
    ///     let errs = form.validate_refresh_token();
    /// }
    /// ```
    #[allow(dead_code)]
    pub fn validate_refresh_token(&self) -> Self {
        // Create new self instance
        let mut errs = Self::default();

        // Retrieve refresh token
        let refresh_token = self.refresh
            .clone()
            .unwrap_or(String::default());

        // Check if empty
        if refresh_token.is_empty() || refresh_token.len() < 5 {
            errs.refresh = Some(String::from("Invalid refresh token"));
        }

        errs
    }

    /// Checks if refresh token exists
    /// Example
    /// ```
    /// // Import token
    /// use library::tokens::Token;
    ///
    /// fn main() {
    ///     // Set token
    ///     let token = Token::new();
    ///     let form = token.normalize();
    ///     let errs = form.validate_web_token();
    /// }
    /// ```
    #[allow(dead_code)]
    pub fn validate_web_token(&self) -> Self {
        // Create new self instance
        let mut errs = Self::default();

        // Retrieve web token
        let web_token = self.web
            .clone()
            .unwrap_or(String::default());

        // Check if empty
        if web_token.is_empty() || web_token.len() < 5 {
            errs.web = Some(String::from("Invalid web token"));
        }

        errs
    }

    /// Converts current struct into serde_json::Value
    #[allow(dead_code)]
    pub fn to_json(&self) -> Value {
        serde_json::to_value(self.clone()).unwrap()
    }

    /// Converts to library::Payload with response code
    #[allow(dead_code)]
    pub fn to_payload(&self, code: u16) -> Payload {
        let mut payload = Payload::default();
        payload.code = Some(code);
        payload.errors = self.clone().to_json();

        payload
    }
}