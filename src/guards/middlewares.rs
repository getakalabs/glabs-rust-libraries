use actix_web::{Error, HttpMessage, HttpResponse};
use actix_web::body::{EitherBody, MessageBody};
use actix_web::dev::{Service, ServiceRequest, ServiceResponse};
use actix_web::http::Method;
use actix_web::web::Data;
use actix_utils::future::{Either, ok, Ready};
use handlebars::Handlebars;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll};
use std::marker::PhantomData;

use crate::{DBPool, PgPooledConnection};
use crate::catchers;
use crate::guards::{AuthenticationFuture, Options};
use crate::Paseto;
use crate::Payload;
use crate::strings;

/// GuardMiddleware service struct
pub struct GuardMiddleware<S, T: 'static> {
    pub service: S,
    pub roles: Option<Vec<String>>,
    pub callback: Option<fn(&mut PgPooledConnection, Options, Option<Data<Arc<Mutex<Paseto>>>>) -> Result<T, String>>,
    pub has_database: Option<bool>,
    pub json_response: bool,
    pub is_optional: bool,
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
        let token = strings::get_token(authorization)
            .unwrap_or(String::new());

        // Retrieve database connection pool
        let mut conn = pool.unwrap();

        // TODO: Create further filter for paseto (case: no paseto in app data)
        // Retrieve paseto
        let paseto = req.app_data::<Data<Arc<Mutex<Paseto>>>>().clone();

        // Create Options
        let guard_options = Options {
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
