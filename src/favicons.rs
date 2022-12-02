use actix_web::{Result, get};
use actix_files::NamedFile;

// Bring native std crate to scope
use std::path::PathBuf;

/// Create favicon endpoint
/// <p><b>Note:</b> it has trailing slash to normalize the path (see /src/routes.rs' middleware for more info)</p>
///
/// Example
/// ```
/// // Import actix_web related crates and catchers
/// use actix_web::{App, web};
/// use library::favicons;
///
/// fn main() {
///     // Start actix web app
///     App::new()
///         .service(favicons::serve);
/// }
/// ```
#[get("/favicon.ico/")]
pub async fn serve() -> Result<NamedFile> {
    let path: PathBuf = "./assets/static/media/favicon.ico".parse().unwrap();
    Ok(NamedFile::open(path)?)
}