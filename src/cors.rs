use actix_cors::Cors;

/// Returns cors setup
///
/// Example
/// ```
/// use actix_web::App;
/// use library::cors;
///
/// fn main() {
///     App::new()
///        .wrap(cors::get());
/// }
/// ```
pub fn get() -> Cors {
    Cors::default()
        .allow_any_origin()
        .allowed_methods(vec!["GET", "POST", "PATCH", "DELETE", "OPTIONS"])
        .allow_any_header()
        .max_age(3600)
}