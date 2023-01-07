use serde::{Deserialize, Serialize};

/// Create Google struct which contains google related information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Google {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    pub verified_email: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub given_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub family_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub picture: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>
}

/// Implement default for Google
impl Default for Google {
    fn default() -> Self {
        Self {
            id: None,
            email: None,
            verified_email: false,
            given_name: None,
            family_name: None,
            picture: None,
            locale: None
        }
    }
}

/// Implement functions for Google
impl Google {
    pub fn new() -> Self {
        Self::default()
    }
}