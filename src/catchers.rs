use actix_web::{HttpResponse, Result, web};
use actix_web::http::{header::{CacheControl, CacheDirective}, StatusCode};
use handlebars::Handlebars;
use std::collections::HashMap;

use crate::Payload;

/// Struct container for catchers options
pub struct Options {
    pub cache_directives: u32,
    pub mime_html: String,
    pub template_404_path: String,
}

/// Default implementation for options
impl Default for Options {
    fn default() -> Self {
        Self {
            cache_directives: 86400u32,
            mime_html: String::from("text/html; charset=utf-8"),
            template_404_path: String::from("error/404.html")
        }
    }
}

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
    // Initialize options
    let options = Options::default();

    // Return response result
    options.http_response_page(hbs, &options.template_404_path, StatusCode::NOT_FOUND)
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

/// Creates a not found page. For non async middleware
pub fn not_found_middleware(hbs: web::Data<Handlebars<'_>>) -> HttpResponse {
    // Initialize options
    let options = Options::default();

    // Set empty hashmap context
    let context:HashMap<String, String> = HashMap::new();

    // Set body
    let body = hbs.render(&options.template_404_path, &context).unwrap();

    // Return http response
    HttpResponse::NotFound()
        .content_type(options.mime_html.clone())
        .insert_header(CacheControl(vec![
            CacheDirective::Public,
            CacheDirective::MaxAge(options.cache_directives.clone()),
        ]))
        .body(serde_json::to_string(&body).unwrap())
}

/// Create catcher implementations
impl Options {
    /// Creates new instance
    pub fn new() -> Self {
        Self::default()
    }

    /// Create new with options
    #[allow(dead_code)]
    pub fn new_options<M, T>(cache_directives: u32, mime_html: M, template_404_path: T) -> Self
        where M: Into<String>,
              T: Into<String>
    {
        // Create bindings
        let mime_html_bindings = mime_html.into();
        let template_404_path_bindings = template_404_path.into();

        // Return with new values
        Self {
            cache_directives,
            mime_html: mime_html_bindings,
            template_404_path: template_404_path_bindings,
        }
    }

    /// Returns a `Result<HttpResponse>` type and displays a page in actix web
    fn http_response_page<T>(&self, hbs: web::Data<Handlebars<'_>>, template: T, status_code: StatusCode) -> Result<HttpResponse>
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
                CacheDirective::MaxAge(self.cache_directives),
            ]))
            .content_type(self.mime_html.clone())
            .body(body);

        // Set response html
        Ok(builder)
    }
}
