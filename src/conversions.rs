use super::Errors;

/// Convert Vec<u8> to i32 [Defaults to 0]
///
/// Example
/// ```
/// use library::conversions;
///
/// fn main() {
///     // Set item
///     let item = "12".to_string().into_bytes();
///     let converted = conversions::vec_to_i32(item);
/// }
/// ```
pub fn vec_to_i32(item: Vec<u8>) -> i32 {
    // Set content to 0
    let mut content = 0;

    // Convert content to bytes
    let content_bytes = &item[..];
    let content_deref = to_type4(content_bytes);

    // Check if content deref is ok
    if content_deref.is_ok() {
        content = i32::from_be_bytes(*content_deref.unwrap());
    }

    // Return int32
    content
}

/// Convert Vec<u8> to String [Defaults to ""]
///
/// Example
/// ```
/// use library::conversions;
///
/// fn main() {
///     // Set item
///     let item = "Foo".to_string().into_bytes();
///     let converted = conversions::vec_to_string(item);
/// }
/// ```
pub fn vec_to_string(item: Vec<u8>) -> String {
    String::from_utf8_lossy(&item).to_string()
}

/// Convert type into &[T; 4]
///
/// Example
/// ```
/// use library::conversions;
///
/// fn main() {
///     // Set item
///     let item:Vec<u8> = "12".to_string().into_bytes();
///     let converted = conversions::to_type4(&item);
/// }
/// ```
pub fn to_type4<T>(item: &[T]) -> Result<&[T; 4], Errors> {
    if item.len() == 4 {
        let ptr = item.as_ptr() as *const [T; 4];
        unsafe {Ok(&*ptr)}
    } else {
        Err(Errors::new("Unable to convert to &[T; 4]"))
    }
}

/// Convert type into &[T; 32]
///
/// Example
/// ```
/// use library::conversions;
///
/// fn main() {
///     // Set item
///     let item:Vec<u8> = "12".to_string().into_bytes();
///     let converted = conversions::to_type32(&item);
/// }
/// ```
pub fn to_type32<T>(item: &[T]) -> Result<&[T; 32], Errors> {
    if item.len() == 32 {
        let ptr = item.as_ptr() as *const [T; 32];
        unsafe {Ok(&*ptr)}
    } else {
        Err(Errors::new("Unable to convert to &[T; 32]"))
    }
}