use serde::Serialize;
use std::default::Default;
use serde_json::Value;

/// File struct contains commonly used information referring to files
#[derive(Debug, Clone, Serialize)]
pub struct File {
    pub file_name: Option<String>,
    pub file_size: Option<String>,
    pub thumbnail: Option<String>,
    pub height: Option<i32>,
    pub width: Option<i32>,
    pub mime_type: Option<String>,
    pub label: Option<String>,
    pub module: Option<String>
}

/// Default implementation for File
impl Default for File {
    fn default() -> Self {
        Self {
            file_name: None,
            file_size: None,
            thumbnail: None,
            height: None,
            width: None,
            mime_type: None,
            label: None,
            module: None
        }
    }
}

/// File implementation
impl File {
    /// Create new File instance
    ///
    /// Example
    /// ```
    /// use library::files::File;
    ///
    /// fn main() {
    ///     let file = File::new();
    /// }
    /// ```
    pub fn new() -> Self {
        Self::default()
    }

    /// Convert file to custom struct
    ///
    /// Example
    /// ```
    /// use library::File;
    /// use serde::{Serialize, Deserialize};
    ///
    /// #[derive(Debug, Clone, Serialize, Deserialize)]
    /// pub struct MyFile {
    ///     pub file_name: Option<String>,
    ///     pub name: Option<String>
    /// }
    ///
    /// impl Default for MyFile {
    ///    fn default() -> Self {
    ///        Self {
    ///            file_name: None,
    ///            name: None
    ///        }
    ///    }
    /// }
    ///
    /// fn main() {
    ///     // Create file
    ///     let mut file = File::new();
    ///     file.file_name = Some(String::from("My file name"));
    ///
    ///     // Convert to MyFile
    ///     let my_file = file.to_struct(MyFile::default());
    /// }
    /// ```
    pub fn to_struct<T: Clone + Serialize + for<'de> serde::Deserialize<'de>>(&self, item: T) -> T {
        // Create bindings and convert to serde_json::Value
        let bindings = item.clone();
        let content = serde_json::to_value(bindings).unwrap_or(Value::Null);

        // Return value to custom struct
        return match serde_json::from_value(content.clone()) {
            Ok(value) => value,
            Err(_) => item
        };
    }
}
