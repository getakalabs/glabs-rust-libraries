use chrono::{DateTime, Duration, Utc};
use paseto::tokens::{validate_local_token, PasetoBuilder, TimeBackend};
use serde::Serialize;

use crate::Cipher;
use crate::Errors;
use crate::Token;

/// Struct container for paseto
#[derive(Debug, Clone, PartialEq)]
pub struct Paseto {
    pub app_name: String,
    pub access_token_key_unit: i32,
    pub access_token_key_time: String,
    pub access_token_key_signing: Vec<u8>,
    pub refresh_token_key_unit: i32,
    pub refresh_token_key_time: String,
    pub refresh_token_key_signing: Vec<u8>,
}

/// Default implementation for Paseto
impl Default for Paseto {
    fn default() -> Self {
        Self {
            app_name: String::default(),
            access_token_key_unit: 0,
            access_token_key_time: String::default(),
            access_token_key_signing: vec![],
            refresh_token_key_unit: 0,
            refresh_token_key_time: String::default(),
            refresh_token_key_signing: vec![]
        }
    }
}

/// Paseto implementation
impl Paseto {
    /// Implement new instance
    ///
    /// Example
    /// ```
    /// use library::Paseto;
    ///
    /// fn main() {
    ///     // Create new paseto instance with default values
    ///     let paseto = Paseto::new();
    /// }
    /// ```
    pub fn new() -> Self {
        Self::default()
    }

    /// Clear current instance
    ///
    /// Example
    /// ```
    /// use library::Paseto;
    ///
    /// fn main() {
    ///     // Create new paseto instance with default values
    ///     let mut paseto = Paseto::new();
    ///     paseto.clear();
    /// }
    /// ```
    pub fn clear(&mut self) -> Self {
        Self::default()
    }

    /// Reconfigure instance
    ///
    /// Example
    /// ```
    /// use library::Paseto;
    ///
    /// fn main() {
    ///     // Create old paseto instance with default values
    ///     let mut old_paseto = Paseto::new();
    ///
    ///     // Create new paseto instance with new values
    ///     let mut new_paseto = Paseto::new();
    ///     new_paseto.app_name = String::from("Getaka Labs");
    ///
    ///     // Reconfigure
    ///     old_paseto.reconfigure(&new_paseto);
    /// }
    /// ```
    pub fn reconfigure(&mut self, item: &Paseto) {
        self.access_token_key_unit = item.clone().access_token_key_unit;
        self.access_token_key_time = item.clone().access_token_key_time;
        self.access_token_key_signing = item.clone().access_token_key_signing;
        self.refresh_token_key_unit = item.clone().refresh_token_key_unit;
        self.refresh_token_key_time = item.clone().refresh_token_key_time;
        self.refresh_token_key_signing = item.clone().refresh_token_key_signing;
    }

    /// Check if paseto has no value
    ///
    /// Example
    /// ```
    /// use library::Paseto;
    ///
    /// fn main() {
    ///     // Create new paseto instance with default values
    ///     let paseto = Paseto::new();
    ///     let is_empty = paseto.is_empty();
    /// }
    /// ```
    pub fn is_empty(&self) -> bool {
        self.clone() == Self::default()
    }

    /// Create new instance with app name
    ///
    /// Example
    /// ```
    /// use library::Paseto;
    ///
    /// fn main() {
    ///     let paseto = Paseto::with_app_name("Getaka Labs");
    /// }
    /// ```
    pub fn with_app_name<T: Into<String>>(app_name: T) -> Self {
        let mut paseto = Self::new();
        paseto.app_name = app_name.into();

        paseto
    }

    /// Generate access, refresh & web token pair
    ///
    /// Example
    /// ```
    /// use library::Paseto;
    /// use serde::{Serialize, Deserialize};
    ///
    /// #[derive(Clone, Debug, Serialize, Deserialize)]
    /// pub struct Actor {
    ///     #[serde(skip_serializing_if = "Option::is_none")]
    ///     pub id: Option<String>,
    ///     #[serde(skip_serializing_if = "Option::is_none")]
    ///     pub first_name: Option<String>,
    ///     #[serde(skip_serializing_if = "Option::is_none")]
    ///     pub last_name: Option<String>,
    /// }
    ///
    /// impl Default for Actor {
    ///     fn default() -> Self {
    ///         Self {
    ///             id: None,
    ///             first_name: None,
    ///             last_name: None
    ///         }
    ///     }
    /// }
    ///
    /// fn main() {
    ///     // Set claims
    ///     let claims = Actor {
    ///         id: Some(String::from("id-12345")),
    ///         first_name: Some(String::from("John")),
    ///         last_name: Some(String::from("Doe"))
    ///     };
    ///
    ///     // Set id
    ///     let id = claims.id.clone().unwrap();
    ///
    ///     // Set paseto config
    ///     let mut paseto = Paseto::with_app_name("Getaka Labs");
    ///     paseto.access_token_key_unit = 15;
    ///     paseto.access_token_key_time = String::from("Days");
    ///     paseto.access_token_key_signing = String::from("BX8hllVNjp5IbB2NiUlt7OUctq71PKSq").into_bytes();
    ///     paseto.refresh_token_key_unit = 30;
    ///     paseto.refresh_token_key_time = String::from("Days");
    ///     paseto.refresh_token_key_signing = String::from("-Xs6DCM7vQ9yKJX2uCQBgpqnWSyqDCGZ").into_bytes();
    ///
    ///     // Generate token
    ///     let result = paseto.generate_tokens(&id, &claims);
    /// }
    /// ```
    pub fn generate_tokens<I, C>(&self, id:I, claims: &C) -> Result<Token, Errors>
        where I: Into<String>,
              C: Serialize
    {
        let c = serde_json::to_value(claims.clone()).unwrap();

        // Set access token duration
        let access_token_duration = match self.access_token_key_time.as_ref() {
            "Minutes" => Duration::minutes(i64::from(self.access_token_key_unit)),
            "Hours" => Duration::hours(i64::from(self.access_token_key_unit)),
            "Days" => Duration::days(i64::from(self.access_token_key_unit)),
            _ =>  Duration::seconds(i64::from(self.access_token_key_unit))
        };

        // Set access token expiry
        let access_token_expiry = Utc::now().checked_add_signed(access_token_duration).unwrap();

        // Set aid
        let aid = id.into();

        // Set access token
        let access_token = PasetoBuilder::new()
            .set_encryption_key(&self.access_token_key_signing.clone()[..])
            .set_expiration(&access_token_expiry)
            .set_subject(&aid)
            .set_footer(format!("key-id:{}", &self.app_name).as_str())
            .set_claim("data", c.clone())
            .build();

        if access_token.is_err() {
            return Err(Errors::new("Unable to generate access token"));
        }

        // Set refresh token duration
        let refresh_token_duration = match self.refresh_token_key_time.as_ref() {
            "Minutes" => Duration::minutes(i64::from(self.refresh_token_key_unit)),
            "Hours" => Duration::hours(i64::from(self.refresh_token_key_unit)),
            "Days" => Duration::days(i64::from(self.refresh_token_key_unit)),
            _ =>  Duration::seconds(i64::from(self.refresh_token_key_unit))
        };

        // Set refresh token expiry
        let refresh_token_expiry = Utc::now().checked_add_signed(refresh_token_duration).unwrap();

        // Set refresh token
        let refresh_token = PasetoBuilder::new()
            .set_encryption_key(&self.refresh_token_key_signing.clone()[..])
            .set_expiration(&refresh_token_expiry)
            .set_subject(&aid)
            .set_footer(format!("key-id:{}", &self.app_name).as_str())
            .set_claim("data", c.clone())
            .build();

        if refresh_token.is_err() {
            return Err(Errors::new("Unable to generate refresh token"));
        }

        // Set cipher
        let cipher = Cipher::new();
        if cipher.is_err() {
            return Err(Errors::new("Cipher library failed to initialize"));
        }

        // Shadow cipher
        let cipher = cipher.unwrap();

        // Create encrypted web token
        let encrypted = cipher.encrypt_web(c.to_string().trim());
        if encrypted.is_err() {
            return Err(Errors::new("Encryption failed"));
        }

        // Create mutable token
        let mut tokens = Token::new();
        tokens.access = Some(access_token.unwrap());
        tokens.refresh = Some(refresh_token.unwrap());
        tokens.web = Some(encrypted.unwrap());

        // Return tokens
        Ok(tokens)
    }

    /// Validate access token
    ///
    /// Example
    /// ```
    /// use library::Paseto;
    /// use serde::{Serialize, Deserialize};
    ///
    /// #[derive(Clone, Debug, Serialize, Deserialize)]
    /// pub struct Actor {
    ///     #[serde(skip_serializing_if = "Option::is_none")]
    ///     pub id: Option<String>,
    ///     #[serde(skip_serializing_if = "Option::is_none")]
    ///     pub first_name: Option<String>,
    ///     #[serde(skip_serializing_if = "Option::is_none")]
    ///     pub last_name: Option<String>,
    /// }
    ///
    /// impl Default for Actor {
    ///     fn default() -> Self {
    ///         Self {
    ///             id: None,
    ///             first_name: None,
    ///             last_name: None
    ///         }
    ///     }
    /// }
    ///
    /// fn main() {
    ///     // Set claims
    ///     let claims = Actor {
    ///         id: Some(String::from("id-12345")),
    ///         first_name: Some(String::from("John")),
    ///         last_name: Some(String::from("Doe"))
    ///     };
    ///
    ///     // Set id
    ///     let id = claims.id.clone().unwrap();
    ///
    ///     // Set paseto config
    ///     let mut paseto = Paseto::with_app_name("Getaka Labs");
    ///     paseto.access_token_key_unit = 15;
    ///     paseto.access_token_key_time = String::from("Days");
    ///     paseto.access_token_key_signing = String::from("BX8hllVNjp5IbB2NiUlt7OUctq71PKSq").into_bytes();
    ///     paseto.refresh_token_key_unit = 30;
    ///     paseto.refresh_token_key_time = String::from("Days");
    ///     paseto.refresh_token_key_signing = String::from("-Xs6DCM7vQ9yKJX2uCQBgpqnWSyqDCGZ").into_bytes();
    ///
    ///     // Generate token
    ///     let result = paseto.generate_tokens(&id, &claims);
    ///     if result.is_ok() {
    ///         let token = result.unwrap();
    ///         let access_token = token.clone().access.unwrap();
    ///         let access_token_result = paseto.validate_access_token(&access_token, Actor::default());
    ///     }
    /// }
    /// ```
    pub fn validate_access_token<T, C>(&self, token: T, _: C) -> Result<C, Errors>
        where T: Into<String>,
              C: serde::de::DeserializeOwned + Default
    {
        // Verify token
        let result = validate_local_token(
            &token.into(),
            Some(format!("key-id:{}", &self.app_name).as_str()),
            &self.access_token_key_signing.clone()[..],
            &TimeBackend::Chrono
        );

        // Check if result is error
        if result.is_err() {
            let is_expired = result.unwrap_err()
                .to_string()
                .to_lowercase()
                .as_str() == "this token is expired (exp claim).";

            return match is_expired {
                true => Err(Errors::new("Your authentication token has expired")),
                false => Err(Errors::new("Invalid authentication token"))
            }
        }

        // Retrieve values from paseto
        let result = result.unwrap();
        let result = result.get("data");
        if result.is_none() {
            return Err(Errors::new("Invalid authentication token"));
        }

        // Return value to custom struct
        let result:Result<C, _> = serde_json::from_value(result.unwrap().clone());
        if result.is_err() {
            return Err(Errors::new("Invalid authentication token"));
        }

        // Return claims
        Ok(result.unwrap())
    }

    /// Validate refresh token
    ///
    /// Example
    /// ```
    /// use library::Paseto;
    /// use serde::{Serialize, Deserialize};
    ///
    /// #[derive(Clone, Debug, Serialize, Deserialize)]
    /// pub struct Actor {
    ///     #[serde(skip_serializing_if = "Option::is_none")]
    ///     pub id: Option<String>,
    ///     #[serde(skip_serializing_if = "Option::is_none")]
    ///     pub first_name: Option<String>,
    ///     #[serde(skip_serializing_if = "Option::is_none")]
    ///     pub last_name: Option<String>,
    /// }
    ///
    /// impl Default for Actor {
    ///     fn default() -> Self {
    ///         Self {
    ///             id: None,
    ///             first_name: None,
    ///             last_name: None
    ///         }
    ///     }
    /// }
    ///
    /// fn main() {
    ///     // Set claims
    ///     let claims = Actor {
    ///         id: Some(String::from("id-12345")),
    ///         first_name: Some(String::from("John")),
    ///         last_name: Some(String::from("Doe"))
    ///     };
    ///
    ///     // Set id
    ///     let id = claims.id.clone().unwrap();
    ///
    ///     // Set paseto config
    ///     let mut paseto = Paseto::with_app_name("Getaka Labs");
    ///     paseto.access_token_key_unit = 15;
    ///     paseto.access_token_key_time = String::from("Days");
    ///     paseto.access_token_key_signing = String::from("BX8hllVNjp5IbB2NiUlt7OUctq71PKSq").into_bytes();
    ///     paseto.refresh_token_key_unit = 30;
    ///     paseto.refresh_token_key_time = String::from("Days");
    ///     paseto.refresh_token_key_signing = String::from("-Xs6DCM7vQ9yKJX2uCQBgpqnWSyqDCGZ").into_bytes();
    ///
    ///     // Generate token
    ///     let result = paseto.generate_tokens(&id, &claims);
    ///     if result.is_ok() {
    ///         let token = result.unwrap();
    ///         let refresh_token = token.clone().refresh.unwrap();
    ///         let refresh_token_result = paseto.validate_refresh_token(&refresh_token, Actor::default());
    ///     }
    /// }
    /// ```
    pub fn validate_refresh_token<T, C>(&self, token: T, _: C) -> Result<C, Errors>
        where T: Into<String>,
              C: serde::de::DeserializeOwned + Default
    {
        // Verify token
        let result = validate_local_token(
            &token.into(),
            Some(format!("key-id:{}", &self.app_name).as_str()),
            &self.refresh_token_key_signing.clone()[..],
            &TimeBackend::Chrono
        );

        // Check if result is error
        if result.is_err() {
            let is_expired = result.unwrap_err()
                .to_string()
                .to_lowercase()
                .as_str() == "this token is expired (exp claim).";

            return match is_expired {
                true => Err(Errors::new("Your refresh token has expired")),
                false => Err(Errors::new("Invalid refresh token"))
            }
        }

        // Retrieve values from paseto
        let result = result.unwrap();
        let result = result.get("data");
        if result.is_none() {
            return Err(Errors::new("Invalid refresh token"));
        }

        // Return value to custom struct
        let result:Result<C, _> = serde_json::from_value(result.unwrap().clone());
        if result.is_err() {
            return Err(Errors::new("Invalid refresh token"));
        }

        // Return claims
        Ok(result.unwrap())
    }

    /// Validate web token
    ///
    /// Example
    /// ```
    /// use library::Paseto;
    /// use serde::{Serialize, Deserialize};
    ///
    /// #[derive(Clone, Debug, Serialize, Deserialize)]
    /// pub struct Actor {
    ///     #[serde(skip_serializing_if = "Option::is_none")]
    ///     pub id: Option<String>,
    ///     #[serde(skip_serializing_if = "Option::is_none")]
    ///     pub first_name: Option<String>,
    ///     #[serde(skip_serializing_if = "Option::is_none")]
    ///     pub last_name: Option<String>,
    /// }
    ///
    /// impl Default for Actor {
    ///     fn default() -> Self {
    ///         Self {
    ///             id: None,
    ///             first_name: None,
    ///             last_name: None
    ///         }
    ///     }
    /// }
    ///
    /// fn main() {
    ///     // Set claims
    ///     let claims = Actor {
    ///         id: Some(String::from("id-12345")),
    ///         first_name: Some(String::from("John")),
    ///         last_name: Some(String::from("Doe"))
    ///     };
    ///
    ///     // Set id
    ///     let id = claims.id.clone().unwrap();
    ///
    ///     // Set paseto config
    ///     let mut paseto = Paseto::with_app_name("Getaka Labs");
    ///     paseto.access_token_key_unit = 15;
    ///     paseto.access_token_key_time = String::from("Days");
    ///     paseto.access_token_key_signing = String::from("BX8hllVNjp5IbB2NiUlt7OUctq71PKSq").into_bytes();
    ///     paseto.refresh_token_key_unit = 30;
    ///     paseto.refresh_token_key_time = String::from("Days");
    ///     paseto.refresh_token_key_signing = String::from("-Xs6DCM7vQ9yKJX2uCQBgpqnWSyqDCGZ").into_bytes();
    ///
    ///     // Generate token
    ///     let result = paseto.generate_tokens(&id, &claims);
    ///     if result.is_ok() {
    ///         let token = result.unwrap();
    ///         let web_token = token.clone().web.unwrap();
    ///         let web_token_result = paseto.validate_refresh_token(&web_token, Actor::default());
    ///     }
    /// }
    /// ```
    pub fn validate_web_token<T, C>(&self, token: T, _: C) -> Result<C, Errors>
        where T: Into<String>,
              C: serde::de::DeserializeOwned + Default
    {
        // Set cipher
        let cipher = Cipher::new();
        if cipher.is_err() {
            return Err(Errors::new("Cipher library failed to initialize"));
        }

        // Shadow cipher
        let cipher = cipher.unwrap();

        // Create decrypt web token
        let result = cipher.decrypt_web(token.into());
        if result.is_err() {
            return Err(Errors::new("Decryption failed"));
        }

        // Return value to custom struct
        let result:Result<C, _> = serde_json::from_str(&result.unwrap().clone());
        if result.is_err() {
            return Err(Errors::new("Invalid authentication token"));
        }

        // Return claims
        Ok(result.unwrap())
    }

    /// Retrieve access token expiry
    ///
    /// Example
    /// ```
    /// use library::Paseto;
    ///
    /// fn main() {
    ///     // Set paseto config
    ///     let mut paseto = Paseto::with_app_name("Getaka Labs");
    ///     paseto.access_token_key_unit = 15;
    ///     paseto.access_token_key_time = String::from("Days");
    ///     paseto.access_token_key_signing = String::from("BX8hllVNjp5IbB2NiUlt7OUctq71PKSq").into_bytes();
    ///     paseto.refresh_token_key_unit = 30;
    ///     paseto.refresh_token_key_time = String::from("Days");
    ///     paseto.refresh_token_key_signing = String::from("-Xs6DCM7vQ9yKJX2uCQBgpqnWSyqDCGZ").into_bytes();
    ///
    ///     // Generate token
    ///     let result = paseto.get_access_token_expiry();
    /// }
    /// ```
    pub fn get_access_token_expiry(&self) -> DateTime<Utc> {
        // Create default expiry
        let expiry = Utc::now()
            .checked_add_signed(Duration::minutes(
                i64::from(self.refresh_token_key_unit)
            ))
            .unwrap();

        self
            .access_token_key_time
            .is_empty()
            .then(|| String::from("Minutes"))
            .map_or(expiry, |item| {
                let duration = match item.as_ref() {
                    "Minutes" => Duration::minutes(i64::from(self.access_token_key_unit)),
                    "Hours" => Duration::hours(i64::from(self.access_token_key_unit)),
                    "Days" => Duration::days(i64::from(self.access_token_key_unit)),
                    _ =>  Duration::seconds(i64::from(self.access_token_key_unit))
                };

                Utc::now()
                    .checked_add_signed(duration)
                    .unwrap()
            })
    }

    /// Retrieve refresh token expiry
    ///
    /// Example
    /// ```
    /// use library::Paseto;
    ///
    /// fn main() {
    ///     // Set paseto config
    ///     let mut paseto = Paseto::with_app_name("Getaka Labs");
    ///     paseto.access_token_key_unit = 15;
    ///     paseto.access_token_key_time = String::from("Days");
    ///     paseto.access_token_key_signing = String::from("BX8hllVNjp5IbB2NiUlt7OUctq71PKSq").into_bytes();
    ///     paseto.refresh_token_key_unit = 30;
    ///     paseto.refresh_token_key_time = String::from("Days");
    ///     paseto.refresh_token_key_signing = String::from("-Xs6DCM7vQ9yKJX2uCQBgpqnWSyqDCGZ").into_bytes();
    ///
    ///     // Generate token
    ///     let result = paseto.get_refresh_token_expiry();
    /// }
    /// ```
    pub fn get_refresh_token_expiry(&self) -> DateTime<Utc> {
        // Create default expiry
        let expiry = Utc::now()
            .checked_add_signed(Duration::minutes(
                i64::from(self.refresh_token_key_unit)
            ))
            .unwrap();

        self
            .refresh_token_key_time
            .is_empty()
            .then(|| String::from("Minutes"))
            .map_or(expiry, |item| {
                let duration = match item.as_ref() {
                    "Minutes" => Duration::minutes(i64::from(self.refresh_token_key_unit)),
                    "Hours" => Duration::hours(i64::from(self.refresh_token_key_unit)),
                    "Days" => Duration::days(i64::from(self.refresh_token_key_unit)),
                    _ =>  Duration::seconds(i64::from(self.refresh_token_key_unit))
                };

                Utc::now()
                    .checked_add_signed(duration)
                    .unwrap()
            })
    }
}
