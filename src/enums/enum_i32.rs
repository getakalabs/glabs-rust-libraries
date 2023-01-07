use serde::{Deserialize, Serialize};

/// EnumI32 consists of dynamic objects
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum EnumI32 {
    String(String),
    I32(i32),
    NULL
}

/// Default implementation for EnumI32
impl Default for EnumI32 {
    fn default() -> Self {
        Self::NULL
    }
}

/// EnumI32 implementation
impl EnumI32 {
    /// Create new instance
    pub fn new() -> Self {
        Self::default()
    }

    /// Retrieve none string value
    pub fn get(&self) -> Option<i32> {
        match self {
            EnumI32::I32(value) => Some(value.clone()),
            _ => None,
        }
    }

    /// Retrieve String value from self
    pub fn to_string(&self) -> Option<String> {
        match self {
            EnumI32::String(value) => Some(value.clone()),
            _ => None,
        }
    }

    /// Check if self is empty
    pub fn is_empty(&self) -> bool {
        self.clone() == Self::default()
    }
}