// Bring serde crate to scope
use serde::Serialize;

// Insert user agent device
#[derive(Debug, Clone, Serialize)]
pub struct UserAgentDevice {
    pub name: Option<String>,
    pub brand: Option<String>,
    pub model: Option<String>,
}

// Create implementation for user agent device
impl UserAgentDevice {
    pub fn new() -> Self {
        Self {
            name: None,
            brand: None,
            model: None
        }
    }
}

// Implement default for user agent device
impl Default for UserAgentDevice {
    fn default() -> Self {
        Self::new()
    }
}