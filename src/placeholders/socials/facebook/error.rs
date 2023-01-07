use serde::{Deserialize, Serialize};

/// Create Facebook FBError struct which contains FB error related information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Error {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(alias = "type")]
    pub error_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_subcode: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fbtrace_id: Option<String>,
}

/// Implement default for Error
impl Default for Error {
    fn default() -> Self {
        Self {
            message: None,
            error_type: None,
            code: None,
            error_subcode: None,
            fbtrace_id: None
        }
    }
}

/// Implement functions for Error
impl Error {
    pub fn new() -> Self {
        Self::default()
    }
}