use actix_web::Error;
use actix_web::body::{EitherBody, MessageBody};
use actix_web::dev::{Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::web::Data;
use actix_utils::future::{ok, Ready};
use std::sync::{Arc, Mutex};

use crate::guards::Options;
use crate::GuardMiddleware;
use crate::Paseto;
use crate::PgPooledConnection;

/// RoleGuard struct middleware
pub struct Guard<T: 'static> {
    pub roles: Option<Vec<String>>,
    pub callback: Option<fn(&mut PgPooledConnection, Options, Option<Data<Arc<Mutex<Paseto>>>>) -> Result<T, String>>,
    pub has_database: Option<bool>,
    pub json_response: bool,
    pub is_optional: bool,
    pub is_refresh_token: bool,
    pub is_web_token: bool,
}

/// Default implementation
///
/// Example
/// ```
/// use library::Guard;
///
/// // Create actor struct
/// pub struct Actor {
///     id: String,
///     first_name: String,
///     last_name: String,
/// }
///
/// fn main() {
///     // Infer actor as generic type for guard
///     let guard = Guard::<Actor>::default();
/// }
/// ```
impl<T> Default for Guard<T> {
    fn default() -> Self {
        Self {
            roles: None,
            callback: None,
            has_database: None,
            json_response: false,
            is_optional: false,
            is_refresh_token: false,
            is_web_token: false,
        }
    }
}

/// Guard implementation
impl<T> Guard<T> {
    /// Creates Guard instance that checks only for database instance
    ///
    /// Example
    /// ```
    /// use library::Guard;
    ///
    /// // Create actor struct
    /// pub struct Actor {
    ///     id: String,
    ///     first_name: String,
    ///     last_name: String,
    /// }
    ///
    /// fn main() {
    ///     // Infer actor as generic type for guard
    ///     let guard = Guard::<Actor>::database();
    /// }
    /// ```
    pub fn database() -> Self {
        Self {
            roles: None,
            callback: None,
            has_database: Some(true),
            json_response: true,
            is_optional: false,
            is_refresh_token: false,
            is_web_token: false,
        }
    }

    /// Creates Guard instance that checks for controller input
    pub fn controller(callback: Option<fn(&mut PgPooledConnection, Options, Option<Data<Arc<Mutex<Paseto>>>>) -> Result<T, String>>) -> Self {
        Self {
            roles: Some(vec![String::from("Controller")]),
            callback,
            has_database: Some(true),
            json_response: true,
            is_optional: false,
            is_refresh_token: false,
            is_web_token: false,
        }
    }

    /// Creates Guard instance that checks for roles input
    pub fn roles(roles:Vec<String>, callback: Option<fn(&mut PgPooledConnection, Options, Option<Data<Arc<Mutex<Paseto>>>>) -> Result<T, String>>) -> Self {
        Self {
            roles: Some(roles),
            callback,
            has_database: Some(true),
            json_response: true,
            is_optional: false,
            is_refresh_token: false,
            is_web_token: false,
        }
    }

    /// Creates Guard instance that checks for refresh token input
    pub fn refresh(roles:Vec<String>, callback: Option<fn(&mut PgPooledConnection, Options, Option<Data<Arc<Mutex<Paseto>>>>) -> Result<T, String>>) -> Self {
        Self {
            roles: Some(roles),
            callback,
            has_database: Some(true),
            json_response: true,
            is_optional: false,
            is_refresh_token: true,
            is_web_token: false,
        }
    }

    /// Creates Guard instance that checks for web token input
    pub fn web(roles:Vec<String>, callback: Option<fn(&mut PgPooledConnection, Options, Option<Data<Arc<Mutex<Paseto>>>>) -> Result<T, String>>) -> Self {
        Self {
            roles: Some(roles),
            callback,
            has_database: Some(true),
            json_response: true,
            is_optional: false,
            is_refresh_token: false,
            is_web_token: true,
        }
    }

    /// Creates Guard instance that checks for roles input but optional
    pub fn optional(roles:Vec<String>, callback: Option<fn(&mut PgPooledConnection, Options, Option<Data<Arc<Mutex<Paseto>>>>) -> Result<T, String>>) -> Self {
        Self {
            roles: Some(roles),
            callback,
            has_database: Some(true),
            json_response: true,
            is_optional: true,
            is_refresh_token: false,
            is_web_token: false,
        }
    }

    /// Set guard as json response
    ///
    /// Example
    /// ```
    /// use library::Guard;
    ///
    /// // Create actor struct
    /// pub struct Actor {
    ///     id: String,
    ///     first_name: String,
    ///     last_name: String,
    /// }
    ///
    /// fn main() {
    ///     // Infer actor as generic type for guard
    ///     let mut guard = Guard::<Actor>::database();
    ///     guard.set_json_response();
    /// }
    /// ```
    pub fn set_json_response(&mut self) -> &mut Self {
        self.json_response = true;
        self
    }
}

/// Middleware factory is `Transform` trait
impl<S, B, T> Transform<S, ServiceRequest> for Guard<T>
    where
        S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
        S::Future: 'static,
        B: MessageBody,
        T: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Transform = GuardMiddleware<S, T>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        let roles = self.roles.clone();
        let callback = self.callback.clone();
        let has_database = self.has_database.clone();
        let json_response = self.json_response.clone();
        let is_optional = self.is_optional.clone();
        let is_refresh_token = self.is_refresh_token.clone();
        let is_web_token = self.is_web_token.clone();

        ok(GuardMiddleware {
            service,
            roles,
            callback,
            has_database,
            json_response,
            is_optional,
            is_refresh_token,
            is_web_token,
        })
    }
}
