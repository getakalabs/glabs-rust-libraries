use handlebars::Handlebars;

/// Struct container for handlebars options
pub struct Options {
    pub asset_path: String,
    pub extension: String,
}

/// Default implementation for options
impl Default for Options {
    fn default() -> Self {
        Self {
            asset_path: String::from("./assets/templates"),
            extension: String::from(".hbs"),
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
    pub fn new_options<AP, E>(asset_path: AP, extension: E) -> Self
        where AP: Into<String>,
              E: Into<String>
    {
        // Create bindings
        let asset_path_bindings = asset_path.into();
        let extension_bindings = extension.into();

        // Return with new values
        Self {
            asset_path: asset_path_bindings,
            extension: extension_bindings,
        }
    }
}

/// Stage handlebar instance
pub fn stage(options: &Options) -> Handlebars<'static> {
    // Initialize handlebars
    let mut handlebars = Handlebars::new();

    // Register directories
    handlebars
        .register_templates_directory(&options.extension, &options.asset_path)
        .expect("Invalid template directory path");

    // Return handlebars
    handlebars
}