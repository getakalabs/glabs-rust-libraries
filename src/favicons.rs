use actix_web::{Result, get};
use actix_files::NamedFile;

// Bring native std crate to scope
use std::path::PathBuf;

/// Struct container for handlebars options
struct Options {
    pub asset_path: String,
}

/// Default implementation for options
impl Default for Options {
    fn default() -> Self {
        Self {
            asset_path: String::from("./assets/templates")
        }
    }
}

/// Create handlebar implementations
impl Options {
    /// Creates new instance
    pub fn new() -> Self {
        Self::default()
    }

    /// Create new with options
    #[allow(dead_code)]
    pub fn new_options<AP: Into<String>>(asset_path: AP) -> Self {
        // Create bindings
        let bindings = asset_path.into();

        // Return with new values
        Self {
            asset_path: bindings,
        }
    }
}

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
    let options = Options::new();
    let path: PathBuf = options.asset_path.parse().unwrap();
    Ok(NamedFile::open(path)?)
}