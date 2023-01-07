use serde::{Deserialize, Serialize};

/// FB struct contains facebook payload struct
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Facebook {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub picture: Option<super::picture::Picture>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<super::error::Error>,
}