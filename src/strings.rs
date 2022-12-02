use serde_json::Value;
use titlecase::titlecase;

/// Retrieve extension based on mime string
///
/// Example:
/// ```
/// use library::strings;
///
/// fn main() {
///     let mime = strings::get_extension_from_mime("image/jpeg");
/// }
/// ```
pub fn get_extension_from_mime<T: Into<String>>(value: T) -> String {
    return match value.into().to_lowercase().as_str() {
        "audio/aac" => String::from(".aac"),
        "application/x-abiword" => String::from(".abw"),
        "application/x-freearc" => String::from(".arc"),
        "video/x-msvideo" => String::from(".avi"),
        "application/vnd.amazon.ebook" => String::from(".azw"),
        "application/octet-stream" => String::from(".bin"),
        "image/bmp" => String::from(".bmp"),
        "application/x-bzip2" => String::from(".bz2"),
        "application/x-csh" => String::from(".csh"),
        "text/css" => String::from(".css"),
        "text/csv" => String::from(".csv"),
        "application/msword" => String::from(".doc"),
        "application/vnd.openxmlformats-officedocument.wordprocessingml.document" => String::from(".docx"),
        "application/vnd.ms-fontobject" => String::from(".eot"),
        "application/epub+zip" => String::from(".epub"),
        "application/gzip" => String::from(".gz"),
        "image/gif" => String::from(".gif"),
        "text/html" => String::from(".html"),
        "image/vnd.microsoft.icon" => String::from(".ico"),
        "text/calendar" => String::from(".ics"),
        "application/java-archive" => String::from(".jar"),
        "image/jpeg" => String::from(".jpg"),
        "text/javascript" => String::from(".js"),
        "application/json" => String::from(".json"),
        "application/ld+json" => String::from(".jsonld"),
        "audio/midi" => String::from(".mid"),
        "audio/x-midi" => String::from(".midi"),
        "audio/mpeg" => String::from(".mp3"),
        "video/mpeg" => String::from(".mpeg"),
        "application/vnd.apple.installer+xml" => String::from(".mpkg"),
        "application/vnd.oasis.opendocument.presentation" => String::from(".odp"),
        "application/vnd.oasis.opendocument.spreadsheet" => String::from(".ods"),
        "application/vnd.oasis.opendocument.text" => String::from(".odt"),
        "audio/ogg" => String::from(".oga"),
        "video/ogg" => String::from(".ogv"),
        "application/ogg" => String::from(".ogx"),
        "audio/opus" => String::from(".opus"),
        "font/otf" => String::from(".otf"),
        "image/png" => String::from(".png"),
        "application/pdf" => String::from(".pdf"),
        "application/x-httpd-php" => String::from(".php"),
        "application/vnd.ms-powerpoint" => String::from(".ppt"),
        "application/vnd.openxmlformats-officedocument.presentationml.presentation" => String::from(".pptx"),
        "application/vnd.rar" => String::from(".rar"),
        "application/rtf" => String::from(".rtf"),
        "application/x-sh" => String::from(".sh"),
        "image/svg+xml" => String::from(".svg"),
        "application/x-shockwave-flash" => String::from(".swf"),
        "application/x-tar" => String::from(".tar"),
        "image/tiff" => String::from(".tif"),
        "video/mp2t" => String::from(".ts"),
        "font/ttf" => String::from(".ttf"),
        "text/plain" => String::from(".txt"),
        "application/vnd.visio" => String::from(".vsd"),
        "audio/wav" => String::from(".wav"),
        "audio/webm" => String::from(".weba"),
        "video/webm" => String::from(".webm"),
        "image/webp" => String::from(".webp"),
        "font/woff" => String::from(".woff"),
        "font/woff2" => String::from(".woff2"),
        "application/xhtml+xml" => String::from(".xhtml"),
        "application/vnd.ms-excel" => String::from(".xls"),
        "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet" => String::from(".xlsx"),
        "application/xml" => String::from(".xml"),
        "application/vnd.mozilla.xul+xml" => String::from(".xul"),
        "application/zip" => String::from(".zip"),
        "video/3gpp" => String::from(".3gp"),
        "audio/3gpp" => String::from(".3gp"),
        "video/3g2" => String::from(".3g2"),
        "audio/3g2" => String::from(".3g2"),
        "application/x-7z-compressed" => String::from(".7z"),
        _ => String::new()
    }
}

/// Checks if string from json::Value is empty
///
/// Example:
/// ```
/// use serde_json::{Map, Value};
/// use library::strings;
///
/// fn main() {
///     let mut item = Map::new();
///     item.insert(String::from("key"), Value::from("Sample"));
///     let result = strings::get_string("key", Value::from(item));
/// }
/// ```
pub fn get_string(key: &str, params: Value) -> String {
    // Retrieve value
    let oval = params.get(key);
    if oval.is_none() {
        return String::new();
    }

    // Unwrap to string
    match serde_json::from_str(oval.unwrap().to_string().as_str()) {
        Ok(value) => value,
        Err(_)=> String::new()
    }
}

/// Retrieve file size in string
///
/// Example:
/// ```
/// use library::strings;
///
/// fn main() {
///     let result = strings::get_file_size(1234.00);
/// }
/// ```
pub fn get_file_size(value: f64) -> String {
    if value <= 1.0 {
        return String::from("0 KB");
    }

    // Check size (truncate it to 2 decimal places)
    let size  = value / 1024 as f64;
    let size = (size * 100f64).floor() / 100.0;

    return match () {
        _ if size < 1000.0 => format!("{} KB", size),
        _ if size < 1000000.0 =>  format!("{} MB", ((size / 1000.0)  * 100f64).floor() / 100.0),
        _ => format!("{} GB", ((size / 1000000.0) * 100f64).floor() / 100.0)
    }
}

/// Retrieve token from string
///
/// Example:
/// ```
/// use library::strings;
///
/// fn main() {
///     let result = strings::get_token("Bearer: My_Key_Here");
/// }
/// ```
pub fn get_token<T: Into<String> + Copy>(value: T) -> Option<String> {
    // Split string
    if !value.into().is_empty() {
        let binding = value.into().clone();
        let split = binding.split("Bearer").collect::<Vec<&str>>();

        if split.len() == 2 {
            return Some(split[1].trim().to_string())
        }
    }

    // Return none
    None
}

/// Checks if value has lowercase
///
/// Example:
/// ```
/// use library::strings;
///
/// fn main() {
///     let result = strings::has_lowercase("test");
/// }
/// ```
pub fn has_lowercase<T: Into<String>>(value: T) -> bool {
    if value.into().bytes().any(|b| matches!(b, b'a'..=b'z')) {
        return true;
    }

    false
}

/// Checks if value has number
///
/// Example:
/// ```
/// use library::strings;
///
/// fn main() {
///     let result = strings::has_number("test1");
/// }
/// ```
pub fn has_number<T: Into<String>>(value: T) -> bool {
    if value.into().bytes().any(|b| matches!(b, b'0'..=b'9')) {
        return true;
    }

    false
}

/// Checks if value has uppercase
///
/// Example:
/// ```
/// use library::strings;
///
/// fn main() {
///     let result = strings::has_uppercase("Test");
/// }
/// ```
pub fn has_uppercase<T: Into<String>>(value: T) -> bool {
    if value.into().bytes().any(|b| matches!(b, b'A'..=b'Z')) {
        return true;
    }

    false
}

/// Checks if value only contains alphanumeric
///
/// Example:
/// ```
/// use library::strings;
///
/// fn main() {
///     let result = strings::has_lowercase("test12345");
/// }
/// ```
pub fn is_alphanumeric<T: Into<String>>(value: T) -> bool {
    if value.into().chars().all(|x| x.is_alphanumeric()) {
        return true;
    }

    false
}

/// Mask a given string
///
/// Example:
/// ```
/// use library::strings;
///
/// fn main() {
///     let result = strings::mask_string("johndoe@gmail.com");
/// }
/// ```
pub fn mask_string<T: Into<String> + Copy>(value: T) -> String {
    // Create default variables
    let mut str = String::new();
    let binding = value.into();
    let len = binding.as_str().len();

    // Loop through characters
    for (i, c) in binding.chars().enumerate() {
        if i == 0 || i == (len - 1) {
            str = format!("{}{}", str, c);
        } else {
            str = format!("{}{}", str, "*");
        }
    }

    str
}

/// Normalize name
///
/// Example:
/// ```
/// use library::strings;
///
/// fn main() {
///     let result = strings::normalize_name("john doe");
/// }
/// ```
pub fn normalize_name<T: Into<String>>(value: T) -> String {
    let bindings = value.into();

    if bindings.is_empty() {
        return String::default();
    }

    // Create string vector
    let mut name_vector = Vec::new();

    // Split string
    let name_split = bindings.split(" ");

    // Loop through name split
    for row in name_split {
        // Set item
        let item = titlecase(row.clone().to_lowercase().as_str());

        match item.clone().as_str() {
            "." => name_vector.push(String::from("")),
            "Jr." => name_vector.push(String::from("Jr")),
            "Sr." => name_vector.push(String::from("Sr")),
            "Ii" => name_vector.push(String::from("II")),
            "Iii" => name_vector.push(String::from("III")),
            "Iv" => name_vector.push(String::from("IV")),
            "Vi" => name_vector.push(String::from("VI")),
            "Vii" => name_vector.push(String::from("VII")),
            "Viii" => name_vector.push(String::from("VIII")),
            "Ix" => name_vector.push(String::from("Ix")),
            "Xi" => name_vector.push(String::from("XI")),
            "Xii" => name_vector.push(String::from("XII")),
            "Xiii" => name_vector.push(String::from("XIII")),
            "Xiv" => name_vector.push(String::from("XIV")),
            "Xv" => name_vector.push(String::from("XV")),
            "Xvi" => name_vector.push(String::from("XVI")),
            "Xvii" => name_vector.push(String::from("XVII")),
            "Xviii" => name_vector.push(String::from("XVIII")),
            "Xix" => name_vector.push(String::from("XIX")),
            "Xx" => name_vector.push(String::from("XX")),
            s=> name_vector.push(s.clone().to_string())
        }

    }

    name_vector.clone().join(" ").to_string()
}