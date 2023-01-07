use sanitizer::prelude::*;
use serde::{Serialize, Deserialize};
use std::default::Default;

/// Struct container for token
#[derive(Debug, Clone, PartialEq, Sanitize, Serialize, Deserialize)]
pub struct Token {
    #[sanitize(trim)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access: Option<String>,
    #[sanitize(trim)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh: Option<String>,
    #[sanitize(trim)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web: Option<String>,
    #[serde(skip_serializing)]
    pub validation_errors: Option<Box<Token>>,
    #[serde(skip_serializing)]
    pub validation_required: Option<Box<Token>>,
    #[serde(skip_serializing)]
    pub validation_invalid: Option<Box<Token>>
}

/// Default implementation for Token
impl Default for Token {
    fn default() -> Self {
        Self {
            access: None,
            refresh: None,
            web: None,
            validation_errors: None,
            validation_required: None,
            validation_invalid: None,
        }
    }
}

/// Token implementation
impl Token {
    /// Create new Token instance
    ///
    /// Example
    /// ```
    /// use library::Token;
    ///
    /// fn main() {
    ///     let token = Token::new();
    /// }
    /// ```
    pub fn new() -> Self {
        Self::default()
    }

    /// Convert custom struct type to Token
    ///
    /// Example
    /// ```
    /// use library::Token;
    /// use serde::{Serialize, Deserialize};
    ///
    /// #[derive(Debug, Clone, Serialize, Deserialize)]
    /// pub struct Token2 {
    ///     #[serde(skip_serializing_if = "Option::is_none")]
    ///     pub access: Option<String>,
    /// 	#[serde(skip_serializing_if = "Option::is_none")]
    ///     pub refresh: Option<String>,
    /// }
    ///
    /// fn main() {
    ///     let token = Token::from(Token2{access: Some(String::from("token1.txt")), refresh: None});
    /// }
    /// ```
    #[allow(dead_code)]
    pub fn from<T>(input: T) -> Self
        where T: Serialize
    {
        let s = serde_json::to_string(&input).unwrap_or(String::default());
        serde_json::from_str(&s).unwrap_or(Token::default())
    }

    /// Convert custom struct type to Token
    ///
    /// Example
    /// ```
    /// use library::Token;
    ///
    /// fn main() {
    ///     let input = r#"{"access": "ABC1234", "refresh": "ABC1234"}"#;
    ///     let token = Token::from_string(input);
    /// }
    /// ```
    #[allow(dead_code)]
    pub fn from_string<T: Into<String>>(input: T) -> Self {
        let bindings = input.into();
        serde_json::from_str(&bindings).unwrap_or(Token::default())
    }

    /// Convert custom struct type from Token to T
    ///
    /// Example
    /// ```
    /// use library::Token;
    /// use serde::{Serialize, Deserialize};
    ///
    /// /// Struct container for Token2
    /// #[derive(Debug, Clone, Serialize, Deserialize)]
    /// pub struct Token2 {
    ///     #[serde(skip_serializing_if = "Option::is_none")]
    ///     pub access: Option<String>,
    ///     #[serde(skip_serializing_if = "Option::is_none")]
    ///     pub refresh: Option<String>
    /// }
    ///
    /// /// Note: this is always required. Default implementation for Token
    /// impl Default for Token2 {
    ///     fn default() -> Self {
    ///         Self {
    ///             access: None,
    ///             refresh: None
    ///         }
    ///     }
    /// }
    ///
    /// fn main() {
    ///     let input = r#"{"access": "ABC1234", "refresh": "ABC1234"}"#;
    ///     let token = Token::from_string(input);
    ///
    ///     let token2 = token.to::<Token2>();
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

    /// Check if tokens has no value
    ///
    /// Example
    /// ```
    /// // Import token
    /// use library::Token;
    ///
    /// fn main() {
    ///     // Set token
    ///     let token = Token::new();
    ///     let is_empty = token.is_empty();
    /// }
    /// ```
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.clone() == Self::default()
    }

    /// Normalize token by performing sanitation and other important stuff
    ///
    /// Example
    /// ```
    /// // Import token
    /// use library::Token;
    ///
    /// fn main() {
    ///     // Set token
    ///     let mut token = Token::new();
    ///     let form = token.normalize();
    /// }
    /// ```
    #[allow(dead_code)]
    pub fn normalize(&mut self) -> &mut Self {
        self.sanitize();
        self
    }

    /// Set error messages
    ///
    /// Example
    /// ```
    /// // Import token
    /// use library::set_error_messages;
    /// use library::Token;
    ///
    /// fn main() {
    ///     // Set token
    ///     let mut token = Token::new();
    ///
    ///     // Create error message for invalid
    ///     let invalid = set_error_messages!(
    ///         Token::new(),
    ///         access = "Invalid access token",
    ///         refresh = "Invalid refresh token",
    ///         web = "Invalid web token"
    ///     );
    ///
    ///     // Create error message for required
    ///     let required = set_error_messages!(
    ///         Token::new(),
    ///         access = "Empty access token",
    ///         refresh = "Empty refresh token",
    ///         web = "Empty web token"
    ///     );
    ///
    ///     // Set error messages
    ///     token
    ///         .normalize()
    ///         .set_error_messages(Some(invalid), Some(required));
    /// }
    /// ```
    #[allow(dead_code)]
    pub fn set_error_messages(&mut self, invalid: Option<Token>, required: Option<Token>) -> &mut Self {
        // Check if invalid has value
        if invalid.is_some() && !invalid.clone().unwrap().is_empty() {
            self.validation_invalid = Some(Box::new(invalid.unwrap().clone()));
        }

        // Check if required has value
        if required.is_some() && !required.clone().unwrap().is_empty() {
            self.validation_required = Some(Box::new(required.unwrap().clone()));
        }

        // Return immutable self
        self
    }


    /// Validate token
    ///
    /// Example
    /// ```
    /// // Import token
    /// use library::set_error_messages;
    /// use library::Token;
    ///
    /// fn main() {
    ///     // Set token
    ///     let mut token = Token::new();
    ///
    ///     // Create error message for invalid
    ///     let invalid = set_error_messages!(
    ///         Token::new(),
    ///         access = "Invalid access token",
    ///         refresh = "Invalid refresh token",
    ///         web = "Invalid web token"
    ///     );
    ///
    ///     // Create error message for required
    ///     let required = set_error_messages!(
    ///         Token::new(),
    ///         access = "Empty access token",
    ///         refresh = "Empty refresh token",
    ///         web = "Empty web token"
    ///     );
    ///
    ///     // Set error messages
    ///     let error = token
    ///         .normalize()
    ///         .set_error_messages(Some(invalid), Some(required))
    ///         .validate();
    /// }
    /// ```
    #[allow(dead_code)]
    pub fn validate(&self) -> Result<(), Self> {
        // Retrieve required messages
        let required = *self
            .validation_required
            .clone()
            .unwrap_or(Box::new(Self::default()));

        // Initialize errors
        let mut errors = Self::new();

        // Check access token
        errors.access = self.verify_access_token_required(
            required.access.unwrap_or(String::default())
        );

        // Check refresh token
        errors.refresh = self.verify_refresh_token_required(
            required.refresh.unwrap_or(String::default())
        );

        errors.web = self.verify_web_token_required(
            required.web.unwrap_or(String::default())
        );

        // Check if validation errors exists
        if errors.validation_errors.is_some() &&
            errors.validation_errors.clone().is_none() {
            return Err(errors);
        }

        Ok(())
    }

    /// Checks if refresh token exists
    /// Example
    /// ```
    /// // Import token
    /// use library::Token;
    ///
    /// fn main() {
    ///     // Set token
    ///     let mut token = Token::new();
    ///     let form = token.normalize();
    ///     let errs = form.verify_access_token_required("Invalid access token");
    /// }
    /// ```
    #[allow(dead_code)]
    pub fn verify_access_token_required<T: Into<String>>(&self, error: T) -> Option<String> {
        // Create error bindings
        let bindings = error.into();

        // Validate refresh token field
        return match validator::validate_required(&self.access) {
            true => Some(bindings),
            false => None
        };
    }

    /// Checks if refresh token exists
    /// Example
    /// ```
    /// // Import token
    /// use library::Token;
    ///
    /// fn main() {
    ///     // Set token
    ///     let mut token = Token::new();
    ///     let form = token.normalize();
    ///     let errs = form.verify_refresh_token_required("Invalid refresh token");
    /// }
    /// ```
    #[allow(dead_code)]
    pub fn verify_refresh_token_required<T: Into<String>>(&self, error: T) -> Option<String> {
        // Create error bindings
        let bindings = error.into();

        // Validate refresh token field
        return match validator::validate_required(&self.refresh) {
            true => Some(bindings),
            false => None
        };
    }

    /// Checks if refresh token exists
    /// Example
    /// ```
    /// // Import token
    /// use library::Token;
    ///
    /// fn main() {
    ///     // Set token
    ///     let mut token = Token::new();
    ///     let error = token
    ///         .normalize()
    ///         .verify_web_token_required("Invalid web token");
    /// }
    /// ```
    #[allow(dead_code)]
    pub fn verify_web_token_required<T: Into<String>>(&self, error: T) -> Option<String> {
        // Create error bindings
        let bindings = error.into();

        // Validate refresh token field
        return match validator::validate_required(&self.web) {
            true => Some(bindings),
            false => None
        };
    }
}
