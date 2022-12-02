// Bring serde crate to scope
use serde::Serialize;

// Insert user agent product
#[derive(Debug, Clone, Serialize)]
pub struct UserAgentProduct {
    pub name: Option<String>,
    pub major: Option<String>,
    pub minor: Option<String>,
    pub patch: Option<String>,
}

// Create implementation for user agent product
impl UserAgentProduct {
    pub fn new() -> Self {
        Self {
            name: None,
            major: None,
            minor: None,
            patch: None,
        }
    }
}

// Implement default for user agent product
impl Default for UserAgentProduct {
    fn default() -> Self {
        Self::new()
    }
}