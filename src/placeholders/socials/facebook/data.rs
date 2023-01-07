use serde::{Deserialize, Serialize};

/// Create Facebook Picture Data struct which contains  picture data related information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Data {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_silhouette: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

/// Implement default for Data
impl Default for Data {
    fn default() -> Self {
        Self {
            height: None,
            width: None,
            is_silhouette: None,
            url: None
        }
    }
}

/// Implement functions for Data
impl Data {
    pub fn new() -> Self {
        Self::default()
    }
}