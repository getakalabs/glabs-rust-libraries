use actix_web::{HttpRequest, HttpResponse, Responder};
use std::fmt::Display;
use actix_web::body::BoxBody;
use serde::Serialize;

/// Struct container for payloads options
#[derive(Debug, Clone, Serialize)]
pub struct Payload {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<u16>,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub challenge: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub message: String,
    #[serde(skip_serializing_if = "serde_json::Value::is_null")]
    pub data: serde_json::Value,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub error: String,
    #[serde(skip_serializing_if = "serde_json::Value::is_null")]
    pub errors: serde_json::Value
}

// Implement default for Payload
impl Default for Payload {
    fn default() -> Self {
        Self {
            code: None,
            challenge: String::default(),
            message: String::default(),
            data: serde_json::Value::Null,
            error: String::default(),
            errors: serde_json::Value::Null,
        }
    }
}

// Implement display for Payload
impl Display for Payload {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

// Implement responder for Payload
impl Responder for Payload {
    type Body = BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        // Set payload
        let payload = serde_json::to_string(&self.clone()).unwrap();

        // Set code
        let mut code = 400;
        match () {
            _ if self.code.as_ref().is_some() => code = *self.code.as_ref().unwrap() as i32,
            _ if self.code.as_ref().is_none() && !self.challenge.is_empty() => code = 200,
            _ => ()
        }

        // Set response builder
        match code {
            200 => HttpResponse::Ok(),
            401 => HttpResponse::Unauthorized(),
            404 => HttpResponse::NotFound(),
            500 => HttpResponse::InternalServerError(),
            _ => HttpResponse::BadRequest()
        }.content_type("application/json")
            .body(payload)
    }
}

// Create Payload implementation
impl Payload {
    /// Creates a new payload instance
    ///
    /// Example
    /// ```
    /// use library::Payload;
    ///
    /// fn main() {
    ///     // Initialize new payload with code 200
    ///     let payload = Payload::new(200);
    /// }
    ///
    /// ```
    pub fn new(code: u16) -> Self {
        let mut payload = Self::default();
        payload.code = Some(code);

        payload
    }

    /// Creates a new payload with success message
    ///
    /// Example
    /// ```
    /// use library::Payload;
    /// use serde_json::Value;
    /// use serde::{Serialize, Deserialize};
    ///
    /// #[derive(Clone, Debug, Serialize, Deserialize)]
    /// pub struct Code {
    ///     #[serde(skip_serializing_if = "Option::is_none")]
    ///     pub id: Option<String>,
    ///     #[serde(skip_serializing_if = "Option::is_none")]
    ///     pub ip: Option<String>,
    /// }
    ///
    /// // Implement default for Code
    /// impl Default for Code {
    ///     fn default() -> Self {
    ///         Self{id: None, ip: None}
    ///     }
    /// }
    ///
    /// fn main() {
    ///     // Initialize new payload with code 200
    ///     let payload = Payload::data(200, Code::default());
    /// }
    /// ```
    pub fn data<T>(code:u16, data:T) -> Self
        where T: Clone + Serialize
    {
        // Create bindings and convert to serde_json::Value
        let bindings = data.clone();
        let content = serde_json::to_value(bindings).unwrap();

        let mut payload = Self::default();
        payload.code = Some(code);
        payload.data = content;

        payload
    }

    /// Creates a new payload with database initialization failed message
    ///
    /// Example
    /// ```
    /// use library::Payload;
    ///
    /// fn main() {
    ///     // Initialize new payload with code 400 and database error message
    ///     let payload = Payload::database();
    /// }
    /// ```
    pub fn database() -> Self {
        let mut payload = Self::default();
        payload.code = Some(400);
        payload.error = String::from("Unable to initialize database connection. Please check your server configuration");

        payload
    }

    /// Creates a new payload with error message
    ///
    /// Example
    /// ```
    /// use library::Payload;
    ///
    /// fn main() {
    ///     // Initialize new payload with code 400 and error message
    ///     let payload = Payload::error("error!");
    /// }
    /// ```
    pub fn error<T: Into<String>>(error: T) -> Self {
        let mut payload = Self::default();
        payload.code = Some(400);
        payload.error = error.into();

        payload
    }

    /// Creates a new payload with success message
    ///
    /// Example
    /// ```
    /// use library::Payload;
    ///
    /// fn main() {
    ///     // Initialize new payload with code 200 and success message
    ///     let payload = Payload::success("success!");
    /// }
    /// ```
    pub fn success<T: Into<String>>(str: T) -> Self {
        let mut payload = Self::default();
        payload.code = Some(200);
        payload.message = str.into();

        payload
    }

    /// Creates a new payload for slack web hook challenge response
    ///
    /// Example
    /// ```
    /// use library::Payload;
    ///
    /// fn main() {
    ///     // Initialize new payload with code 200 and success message
    ///     let payload = Payload::challenge(String::from("some-random-crap-here!"));
    /// }
    /// ```
    pub fn challenge<T: Into<String>>(str: T) -> Self {
        let mut payload = Self::default();
        payload.challenge = str.into();

        payload
    }

    /// Creates a new payload instance for 404 page not found
    ///
    /// Example
    /// ```
    /// use library::Payload;
    ///
    /// fn main() {
    ///     // Initialize new payload with code 200 and success message
    ///     let payload = Payload::page_not_found();
    /// }
    /// ```
    pub fn page_not_found() -> Self {
        let mut payload = Self::default();
        payload.code = Some(404);
        payload.error = String::from("Page Not Found");

        payload
    }

    /// Creates a new payload instance for invalid permission
    ///
    /// Example
    /// ```
    /// use library::Payload;
    ///
    /// fn main() {
    ///     // Initialize new invalid permission error
    ///     let payload = Payload::permission();
    /// }
    /// ```
    pub fn permission() -> HttpResponse {
        let mut payload = Self::default();
        payload.code = Some(400);
        payload.error = String::from("Your account does not have enough permission to perform this task");

        HttpResponse::BadRequest()
            .content_type("application/json")
            .body(serde_json::to_string(&payload).unwrap())
    }

    /// Creates a new http response for database connection error
    ///
    /// Example
    /// ```
    /// use library::Payload;
    ///
    /// fn main() {
    ///     // Initialize new payload with HttpResponse type json output
    ///     let payload = Payload::database_connection();
    /// }
    /// ```
    pub fn database_connection() -> HttpResponse {
        let mut payload = Self::default();
        payload.code = Some(400);
        payload.error = String::from("Unable to initialize database connection. Please check your server configuration");

        HttpResponse::BadRequest()
            .content_type("application/json")
            .body(serde_json::to_string(&payload).unwrap())
    }

    /// Creates a new http response for expired token
    ///
    /// Example
    /// ```
    /// use library::Payload;
    ///
    /// fn main() {
    ///     // Initialize new payload with HttpResponse type json output
    ///     let payload = Payload::expired_token();
    /// }
    /// ```
    pub fn expired_token() -> HttpResponse {
        let mut payload = Self::default();
        payload.code = Some(401);
        payload.error = String::from("Your authentication token has expired");

        HttpResponse::BadRequest()
            .content_type("application/json")
            .body(serde_json::to_string(&payload).unwrap())
    }

    /// Creates a new http response for invalid server configuration
    ///
    /// Example
    /// ```
    /// use library::Payload;
    ///
    /// fn main() {
    ///     // Initialize new payload with HttpResponse type json output
    ///     let payload = Payload::invalid_server_config();
    /// }
    /// ```
    pub fn invalid_server_config() -> HttpResponse {
        let mut payload = Self::default();
        payload.code = Some(400);
        payload.error = String::from("Invalid server configuration. Please contact your server administrator for more info");

        HttpResponse::BadRequest()
            .content_type("application/json")
            .body(serde_json::to_string(&payload).unwrap())
    }

    /// Creates a new http response for invalid token
    ///
    /// Example
    /// ```
    /// use library::Payload;
    ///
    /// fn main() {
    ///     // Initialize new payload with HttpResponse type json output
    ///     let payload = Payload::invalid_authentication_token();
    /// }
    /// ```
    pub fn invalid_authentication_token() -> HttpResponse {
        let mut payload = Self::default();
        payload.code = Some(400);
        payload.error = String::from("Invalid authentication token");

        HttpResponse::BadRequest()
            .content_type("application/json")
            .body(serde_json::to_string(&payload).unwrap())
    }

    /// Creates a new http response for invalid refresh token
    ///
    /// Example
    /// ```
    /// use library::Payload;
    ///
    /// fn main() {
    ///     // Initialize new payload with HttpResponse type json output
    ///     let payload = Payload::invalid_refresh_token();
    /// }
    /// ```
    pub fn invalid_refresh_token() -> HttpResponse {
        let mut payload = Self::default();
        payload.code = Some(400);
        payload.error = String::from("Invalid refresh token");

        HttpResponse::BadRequest()
            .content_type("application/json")
            .body(serde_json::to_string(&payload).unwrap())
    }

    /// Creates a new http response for invalid web token
    ///
    /// Example
    /// ```
    /// use library::Payload;
    ///
    /// fn main() {
    ///     // Initialize new payload with HttpResponse type json output
    ///     let payload = Payload::invalid_web_token();
    /// }
    /// ```
    pub fn invalid_web_token() -> HttpResponse {
        let mut payload = Self::default();
        payload.code = Some(400);
        payload.error = String::from("Invalid web token");

        HttpResponse::BadRequest()
            .content_type("application/json")
            .body(serde_json::to_string(&payload).unwrap())
    }
}

