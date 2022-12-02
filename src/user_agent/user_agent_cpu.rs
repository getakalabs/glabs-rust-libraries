// Bring serde crate to scope
use serde::Serialize;

// Insert user agent cpu
#[derive(Debug, Clone, Serialize)]
pub struct UserAgentCPU {
    pub architecture: Option<String>,
}

// Create implementation for user agent cpu
impl UserAgentCPU {
    pub fn new() -> Self {
        Self {
            architecture: None,
        }
    }
}

// Implement default for user agent cpu
impl Default for UserAgentCPU {
    fn default() -> Self {
        Self::new()
    }
}