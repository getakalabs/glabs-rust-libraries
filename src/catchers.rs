use actix_web::{HttpResponse, Result, web};
use actix_web::http::{header::{CacheControl, CacheDirective}, StatusCode};
use handlebars::Handlebars;
use std::collections::HashMap;

use super::Payload;

/// Set cache lifetime directives 60s x 60m * 24h
static BACKEND_CACHE_DIRECTIVES:u32 = 86400u32;
/// Set handlebars 404 html path
static BACKEND_HBS_ERROR_404: &'static str = "error/404.html";
/// Set html mime type
static BACKEND_MIME_HTML: &'static str = "text/html; charset=utf-8";

/// Creates a not found page. To be used under actix's `default_service`
///
/// Example
/// ```
/// // Import actix_web related crates and catchers
/// use actix_web::{App, web};
/// use library::catchers;
///
/// fn main() {
///     // Start actix web app
///     App::new()
///         .default_service(
///             web::route().to(catchers::not_found_page)
///         );
/// }
/// ```
pub async fn not_found_page(hbs: web::Data<Handlebars<'_>>) -> Result<HttpResponse> {
    http_response_page(hbs, BACKEND_HBS_ERROR_404, StatusCode::NOT_FOUND)
}

/// Creates a synced not found page
pub fn sync_not_found_page(hbs: web::Data<Handlebars<'_>>) -> HttpResponse {
    // Set empty hashmap context
    let context:HashMap<String, String> = HashMap::new();

    // Set body
    let body = hbs.render(BACKEND_HBS_ERROR_404, &context).unwrap();

    // Set builder
    let builder = HttpResponse::build(StatusCode::NOT_FOUND)
        .insert_header(CacheControl(vec![
            CacheDirective::Public,
            CacheDirective::MaxAge(BACKEND_CACHE_DIRECTIVES),
        ]))
        .content_type(BACKEND_MIME_HTML)
        .body(body);

    // Set response html
    builder
}

/// Creates a not found page. For non async middleware
pub fn not_found_middleware(hbs: web::Data<Handlebars<'_>>) -> HttpResponse {
    // Set empty hashmap context
    let context:HashMap<String, String> = HashMap::new();

    // Set body
    let body = hbs.render(&BACKEND_HBS_ERROR_404, &context).unwrap();

    HttpResponse::NotFound()
        .content_type(BACKEND_MIME_HTML)
        .insert_header(CacheControl(vec![
            CacheDirective::Public,
            CacheDirective::MaxAge(BACKEND_CACHE_DIRECTIVES),
        ]))
        .body(serde_json::to_string(&body).unwrap())
}

/// Creates a not found json response. To be used under actix's `default_service`
///
/// Example
/// ```
/// // Import actix_web related crates and catchers
/// use actix_web::{App, web};
/// use library::catchers;
///
/// fn main() {
///     // Start actix web app
///     App::new()
///         .default_service(
///             web::route().to(catchers::not_found_json)
///         );
/// }
/// ```
pub async fn not_found_json() -> Payload {
    let mut payload = Payload::new(404);
    payload.error = String::from("Page Not Found");

    payload
}

/// Returns a `Result<HttpResponse>` type and displays a page in actix web
fn http_response_page<T>(hbs: web::Data<Handlebars<'_>>, template: T, status_code: StatusCode) -> Result<HttpResponse>
    where T: Into<String>
{
    // Set empty hashmap context
    let context:HashMap<String, String> = HashMap::new();

    // Set body
    let body = hbs.render(&template.into(), &context).unwrap();

    // Set builder
    let builder = HttpResponse::build(status_code)
        .insert_header(CacheControl(vec![
            CacheDirective::Public,
            CacheDirective::MaxAge(BACKEND_CACHE_DIRECTIVES),
        ]))
        .content_type(BACKEND_MIME_HTML)
        .body(body);

    // Set response html
    Ok(builder)
}