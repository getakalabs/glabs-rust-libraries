use actix_cors::Cors;

/// Returns cors setup
///
/// Example
/// ```
/// use actix_web::App;
/// use library::cors;
///
/// pub static METHODS: &'static [&'static str] = &["GET", "POST", "PATCH", "DELETE", "OPTIONS"];
///
/// fn main() {
///     App::new()
///        .wrap(cors::stage(METHODS));
/// }
/// ```
pub fn stage(methods: &'static [&'static str]) -> Cors {
    let m = methods.clone().to_vec();

    Cors::default()
        .allow_any_origin()
        .allowed_methods(m)
        .allow_any_header()
        .max_age(3600)
}