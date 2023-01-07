use sanitizer::prelude::*;
use serde::{Serialize, Deserialize};
use std::default::Default;

use crate::enums::EnumI32;

/// Struct container for file
#[derive(Debug, Clone, PartialEq, Sanitize, Serialize, Deserialize)]
pub struct File {
    #[sanitize(trim)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
    #[sanitize(trim)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<String>,
    #[sanitize(trim)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<EnumI32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<EnumI32>,
    #[sanitize(trim)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,
    #[sanitize(trim)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[sanitize(trim)]
    #[serde(skip_serializing_if = "Option::is_none")]
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
            module: None,
        }
    }
}

/// File implementation
impl File {
    /// Create new File instance
    ///
    /// Example
    /// ```
    /// use library::File;
    ///
    /// fn main() {
    ///     let file = File::new();
    /// }
    /// ```
    pub fn new() -> Self {
        Self::default()
    }

    /// Convert custom struct type to File
    ///
    /// Example
    /// ```
    /// use library::File;
    /// use serde::{Serialize, Deserialize};
    ///
    /// #[derive(Debug, Clone, Serialize, Deserialize)]
    /// pub struct File2 {
    ///     #[serde(skip_serializing_if = "Option::is_none")]
    ///     pub file_name: Option<String>,
    /// 	#[serde(skip_serializing_if = "Option::is_none")]
    ///     pub file_size: Option<String>,
    /// }
    ///
    /// fn main() {
    ///     let file = File::from(File2{file_name: Some(String::from("file1.txt")), file_size: None});
    /// }
    /// ```
    #[allow(dead_code)]
    pub fn from<T>(input: T) -> Self
        where T: Serialize
    {
        let s = serde_json::to_string(&input).unwrap_or(String::default());
        serde_json::from_str(&s).unwrap_or(File::default())
    }

    /// Convert custom struct type to File
    ///
    /// Example
    /// ```
    /// use library::File;
    ///
    /// fn main() {
    ///     let input = r#"{"file_name": "ABC1234", "file_size": "ABC1234"}"#;
    ///     let file = File::from_string(input);
    /// }
    /// ```
    #[allow(dead_code)]
    pub fn from_string<T: Into<String>>(input: T) -> Self {
        let bindings = input.into();
        serde_json::from_str(&bindings).unwrap_or(File::default())
    }

    /// Convert custom struct type from File to T
    ///
    /// Example
    /// ```
    /// use library::File;
    /// use serde::{Serialize, Deserialize};
    ///
    /// /// Struct container for File2
    /// #[derive(Debug, Clone, Serialize, Deserialize)]
    /// pub struct File2 {
    ///     #[serde(skip_serializing_if = "Option::is_none")]
    ///     pub file_name: Option<String>,
    ///     #[serde(skip_serializing_if = "Option::is_none")]
    ///     pub file_size: Option<String>
    /// }
    ///
    /// /// Note: this is always required. Default implementation for File
    /// impl Default for File2 {
    ///     fn default() -> Self {
    ///         Self {
    ///             file_name: None,
    ///             file_size: None
    ///         }
    ///     }
    /// }
    ///
    /// fn main() {
    ///     let input = r#"{"file_name": "ABC1234", "file_size": "ABC1234"}"#;
    ///     let file = File::from_string(input);
    ///
    ///     let file2 = file.to::<File2>();
    /// }
    /// ```
    #[allow(dead_code)]
    pub fn to<T>(&self) -> T
        where T: serde::de::DeserializeOwned + Default
    {
        let value = self.clone();
        let s = serde_json::to_string(&value).unwrap_or(String::default());
        let response:T = serde_json::from_str(&s).unwrap_or(T::default());

        response
    }

    /// Check if files has no value
    ///
    /// Example
    /// ```
    /// // Import file
    /// use library::File;
    ///
    /// fn main() {
    ///     // Set file
    ///     let file = File::new();
    ///     let is_empty = file.is_empty();
    /// }
    /// ```
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.clone() == Self::default()
    }

    /// Normalize file by performing sanitation and other important stuff
    ///
    /// Example
    /// ```
    /// // Import file
    /// use library::File;
    ///
    /// fn main() {
    ///     // Set file
    ///     let mut file = File::new();
    ///     let form = file.normalize();
    /// }
    /// ```
    #[allow(dead_code)]
    pub fn normalize(&mut self) -> &mut Self {
        self.sanitize();
        self
    }

    /// Retrieve the file size of the given file
    #[allow(dead_code)]
    pub fn get_file_size(bytes: Vec<u8>) -> String {
        let mut size = bytes.len() as f64;
        let mut unit = "bytes";

        if size > 1024.0 {
            size /= 1024.0;
            unit = "KB";
        }

        if size > 1024.0 {
            size /= 1024.0;
            unit = "MB";
        }

        if size > 1024.0 {
            size /= 1024.0;
            unit = "GB";
        }

        if size > 1024.0 {
            size /= 1024.0;
            unit = "TB";
        }

        format!("{:.2} {}", size, unit)
    }

    /// Checks if mime type is image
    // Original list
    // let mimes = vec![
    //     "image/bmp",
    //     "image/gif",
    //     "image/jpeg",
    //     "image/png",
    //     "image/tiff",
    //     "image/vnd.adobe.photoshop",
    //     "image/vnd.dwg",
    //     "image/vnd.dxf",
    //     "image/vnd.fastbidsheet",
    //     "image/vnd.fpx",
    //     "image/vnd.net-fpx",
    //     "image/vnd.wap.wbmp",
    //     "image/webp"
    // ];
    pub fn is_image<T: Into<String>>(mime: T) -> bool {
        // Create bindings
        let bindings = mime.into().to_lowercase();

        // Set processable images
        let mimes = vec![
            String::from("image/bmp"),
            String::from("image/gif"),
            String::from("image/jpg"),
            String::from("image/jpeg"),
            String::from("image/png"),
            String::from("image/tiff"),
            String::from("image/webp"),
            String::from("image/x-icon"),
            String::from("image/x-tga"),
        ];

        mimes.contains(&bindings)
    }
}
