// Bring serde crate to scope
use serde::Serialize;

// Insert user agent os
#[derive(Debug, Clone, Serialize)]
pub struct UserAgentOS {
    pub name: Option<String>,
    pub major: Option<String>,
    pub minor: Option<String>,
    pub patch: Option<String>,
    pub patch_minor: Option<String>
}

// Create implementation for user agent os
impl UserAgentOS {
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

// Implement default for user agent os
impl Default for UserAgentOS {
    fn default() -> Self {
        Self::new()
    }
}