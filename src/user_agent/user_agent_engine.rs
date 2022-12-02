// Bring serde crate to scope
use serde::Serialize;

// Insert user agent engine
#[derive(Debug, Clone, Serialize)]
pub struct UserAgentEngine {
    pub name: Option<String>,
    pub major: Option<String>,
    pub minor: Option<String>,
    pub patch: Option<String>,
    pub patch_minor: Option<String>
}

// Create implementation for user agent engine
impl UserAgentEngine {
    pub fn new() -> Self {
        Self {
            name: None,
            major: None,
            minor: None,
            patch: None,
            patch_minor: None,
        }
    }
}

// Implement default for user agent engine
impl Default for UserAgentEngine {
    fn default() -> Self {
        Self::new()
    }
}