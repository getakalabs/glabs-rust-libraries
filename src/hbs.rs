use handlebars::Handlebars;

/// Set handlebars extension `.hbs`
pub static BACKEND_HBS_EXT: &'static str = ".hbs";
/// Set handlebars path to `./assets/templates`
pub static BACKEND_HBS_PATH: &'static str = "./assets/templates";
/// Set backend static path (from url)
pub static BACKEND_HBS_STATIC_PATH: &'static str = "static/";
/// Set subscription sse html
pub static BACKEND_HBS_SUBSCRIPTION_SSE: &'static str = "sse.html";
/// Set subscription websocket html
pub static BACKEND_HBS_SUBSCRIPTION_WS: &'static str = "websocket.html";

/// Stage handlebars - Create handlebars instance
///
/// Example
/// ```
/// // Import actix_web related crates and catchers
/// use actix_web::{App, web};
/// use library::hbs;
///
/// fn main() {
///     // Set handlebars
///     let handlebars = hbs::stage();
///
///     // Start actix web app
///     App::new()
///         .app_data(web::Data::new(handlebars.clone()));
/// }
/// ```
pub fn stage() -> Handlebars<'static> {
    // Initialize handlebars
    let mut handlebars = Handlebars::new();

    // Set template variables
    let tpl_extension = BACKEND_HBS_EXT;
    let dir_path = BACKEND_HBS_PATH;

    // Register directories
    handlebars
        .register_templates_directory(tpl_extension, dir_path)
        .expect("Invalid template directory path");

    // Return handlebars
    handlebars
}