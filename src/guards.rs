use actix_web::{Error, HttpMessage, HttpResponse};
use actix_web::body::{EitherBody, MessageBody};
use actix_web::dev::{Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::http::Method;
use actix_web::web::Data;
use actix_utils::future::{Either, ok, Ready};
use futures::{ready, Future};
use handlebars::Handlebars;
use pin_project::pin_project;
use std::marker::PhantomData;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll};

use crate::catchers;
use crate::{DBPool, Paseto, Payload, PgPooledConnection};
use crate::strings::get_token;

/// AuthenticationFuture struct
#[pin_project]
pub struct AuthenticationFuture<S, B> where S: Service<ServiceRequest>, {
    #[pin]
    fut: S::Future,
    _phantom: PhantomData<B>,
}

/// Implement Future for AuthenticationFuture
impl<S, B> Future for AuthenticationFuture<S, B>
    where
        B: MessageBody,
        S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
{
    type Output = Result<ServiceResponse<EitherBody<B>>, Error>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let res = match ready!(self.project().fut.poll(cx)) {
            Ok(res) => res,
            Err(err) => return Poll::Ready(Err(err.into())),
        };

        Poll::Ready(Ok(res.map_into_left_body()))
    }
}

/// RoleGuard struct middleware
pub struct Guard<T: 'static> {
    pub roles: Option<Vec<String>>,
    pub callback: Option<fn(&mut PgPooledConnection, GuardOptions, Option<Data<Arc<Mutex<Paseto>>>>) -> Result<T, String>>,
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
    pub fn controller(callback: Option<fn(&mut PgPooledConnection, GuardOptions, Option<Data<Arc<Mutex<Paseto>>>>) -> Result<T, String>>) -> Self {
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
    pub fn roles(roles:Vec<String>, callback: Option<fn(&mut PgPooledConnection, GuardOptions, Option<Data<Arc<Mutex<Paseto>>>>) -> Result<T, String>>) -> Self {
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
    pub fn refresh(roles:Vec<String>, callback: Option<fn(&mut PgPooledConnection, GuardOptions, Option<Data<Arc<Mutex<Paseto>>>>) -> Result<T, String>>) -> Self {
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
    pub fn web(roles:Vec<String>, callback: Option<fn(&mut PgPooledConnection, GuardOptions, Option<Data<Arc<Mutex<Paseto>>>>) -> Result<T, String>>) -> Self {
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
    pub fn optional(roles:Vec<String>, callback: Option<fn(&mut PgPooledConnection, GuardOptions, Option<Data<Arc<Mutex<Paseto>>>>) -> Result<T, String>>) -> Self {
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

/// GuardMiddleware service struct
pub struct GuardMiddleware<S, T: 'static> {
    service: S,
    roles: Option<Vec<String>>,
    callback: Option<fn(&mut PgPooledConnection, GuardOptions, Option<Data<Arc<Mutex<Paseto>>>>) -> Result<T, String>>,
    has_database: Option<bool>,
    json_response: bool,
    is_optional: bool,
    pub is_refresh_token: bool,
    pub is_web_token: bool,
}

/// Service implementation for GuardMiddleware
impl<S, B, T> Service<ServiceRequest> for GuardMiddleware<S, T>
    where
        S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
        S::Future: 'static,
        B: MessageBody,
        T: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = Either<AuthenticationFuture<S, B>, Ready<Result<Self::Response, Self::Error>>>;

    fn poll_ready(&self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        // Set flags and payload
        let mut payload = Payload::invalid_authentication_token();

        // Check if method is options, allow request
        if Method::OPTIONS == *req.method() {
            return Either::left(AuthenticationFuture {
                fut: self.service.call(req),
                _phantom: PhantomData,
            });
        }

        // Check for handlebars
        let hbs = req.app_data::<Data<Handlebars<'_>>>();

        // TODO: Create further filter for database (case: no database in app data)
        // Retrieve pg pool to validate token in database
        let has_database = self.clone().has_database;
        let json_response = self.clone().json_response;
        let pool = req.app_data::<Data<DBPool>>().unwrap().get();
        if has_database.is_some() && has_database.clone().unwrap() == true && pool.is_err() {
            // Check response type
            match json_response.clone() || (json_response.clone() == false && hbs.is_none()) {
                true => payload = Payload::database_connection(),
                false => payload = catchers::not_found_middleware(hbs.clone().unwrap().clone()),
            }

            // Return response
            return Either::right(ok(req
                .into_response(payload)
                .map_into_boxed_body()
                .map_into_right_body()));
        }

        // Check if other options does not exist
        if has_database.is_some() && has_database.clone().unwrap() == true &&
            !pool.is_err() && self.callback.clone().is_none() && self.roles.clone().is_none() &&
            self.is_refresh_token.clone() == false && self.is_web_token.clone() == false {

            // Allow access
            return Either::left(AuthenticationFuture {
                fut: self.service.call(req),
                _phantom: PhantomData,
            });
        }

        // Retrieve allowed roles
        let roles = self.roles.clone();
        let has_allowed_roles = match roles.is_some() {
            true => roles.clone().unwrap().len() > 0,
            false => false
        };

        // Check if roles exists and database does not exists
        if has_allowed_roles && (has_database.is_none() || !has_database.clone().unwrap()) {
            match json_response.clone() || ( json_response.clone() == false && hbs.is_none() ) {
                true => payload = Payload::database_connection(),
                false => payload = catchers::not_found_middleware(hbs.clone().unwrap().clone()),
            }

            // Return response
            return Either::right(ok(req
                .into_response( payload)
                .map_into_boxed_body()
                .map_into_right_body()));
        }

        // Retrieve authorization
        let authorization = req
            .headers()
            .get("Authorization")
            .map(|h| h.to_str().unwrap_or(""))
            .unwrap_or("")
            .trim();

        // Retrieve token
        let token = get_token(authorization).unwrap_or(String::new());

        // Retrieve database connection pool
        let mut conn = pool.unwrap();

        // TODO: Create further filter for paseto (case: no paseto in app data)
        // Retrieve paseto
        let paseto = req.app_data::<Data<Arc<Mutex<Paseto>>>>().clone();

        // Create GuardOptions
        let guard_options = GuardOptions {
            token,
            roles,
            json_response: self.json_response.clone(),
            is_optional: self.is_optional.clone(),
            is_refresh_token: self.is_refresh_token.clone(),
            is_web_token: self.is_web_token.clone()
        };

        // Retrieve callback
        let callback = self.callback.clone();
        if callback.is_some() {
            let result = (callback.unwrap())(&mut conn, guard_options, paseto.cloned());
            return match result {
                Ok(claims) => {
                    req.extensions_mut().insert(claims);

                    Either::left(AuthenticationFuture {
                        fut: self.service.call(req),
                        _phantom: PhantomData,
                    })
                },
                Err(error) => {
                    let payload = match error.contains("expired") {
                        true => {
                            let mut payload = Payload::default();
                            payload.code = Some(401);
                            payload.error = error.clone();

                            HttpResponse::Unauthorized()
                                .content_type("application/json")
                                .body(serde_json::to_string(&payload).unwrap())
                        }
                        false => {
                            let mut payload = Payload::default();
                            payload.code = Some(400);
                            payload.error = error.clone();

                            HttpResponse::BadRequest()
                                .content_type("application/json")
                                .body(serde_json::to_string(&payload).unwrap())
                        }
                    };

                    // Disable access
                    Either::right(ok(req
                        .into_response(payload)
                        .map_into_boxed_body()
                        .map_into_right_body()))
                }
            }
        }

        // Disable access
        return Either::right(ok(req
            .into_response(payload)
            .map_into_boxed_body()
            .map_into_right_body()));
    }
}

/// GuardOptions struct
pub struct GuardOptions {
    pub token: String,
    pub roles: Option<Vec<String>>,
    pub json_response: bool,
    pub is_optional: bool,
    pub is_refresh_token: bool,
    pub is_web_token: bool,
}