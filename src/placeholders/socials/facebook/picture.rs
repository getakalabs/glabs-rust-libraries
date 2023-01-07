use serde::{Deserialize, Serialize};

/// Create Facebook Picture struct which contains  picture related information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Picture {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<super::data::Data>,
}

/// Implement default for Picture
impl Default for Picture {
    fn default() -> Self {
        Self { data: None }
    }
}

/// Implement functions for Picture
impl Picture {
    pub fn new() -> Self {
        Self::default()
    }
}