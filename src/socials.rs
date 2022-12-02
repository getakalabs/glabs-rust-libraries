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
    pub picture: Option<FBPicture>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<FBError>,
}

/// Default implementation for Facebook
impl Default for Facebook {
    fn default() -> Self {
        Self {
            id: None,
            name: None,
            first_name: None,
            last_name: None,
            email: None,
            picture: None,
            error: None
        }
    }
}

/// Facebook implementations
impl Facebook {
    /// Create new instance
    ///
    /// Example
    /// ```
    /// use library::socials::Facebook;
    ///
    /// let fb = Facebook::new();
    /// ```
    pub fn new() -> Self {
        Self::default()
    }
}

/// Create Facebook FBError struct which contains FB error related information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FBError {
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

/// Implement default for FBError
impl Default for FBError {
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

/// Implement functions for FBError
impl FBError {
    /// Create new instance
    ///
    /// Example
    /// ```
    /// use library::socials::FBError;
    ///
    /// let error = FBError::new();
    /// ```
    pub fn new() -> Self {
        Self::default()
    }
}

/// Create Facebook FBPicture struct which contains FB picture related information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FBPicture {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<FBData>,
}

/// Implement default for FBPicture
impl Default for FBPicture {
    fn default() -> Self {
        Self { data: None }
    }
}

/// Implement functions for FBPicture
impl FBPicture {
    /// Create new instance
    ///
    /// Example
    /// ```
    /// use library::socials::FBPicture;
    ///
    /// let picture = FBPicture::new();
    /// ```
    pub fn new() -> Self {
        Self::default()
    }
}

/// Create Facebook FBPicture FBData struct which contains FB picture data related information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FBData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_silhouette: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

/// Implement default for FBData
impl Default for FBData {
    fn default() -> Self {
        Self {
            height: None,
            width: None,
            is_silhouette: None,
            url: None
        }
    }
}

/// Implement functions for FBData
impl FBData {
    /// Create new instance
    ///
    /// Example
    /// ```
    /// use library::socials::FBData;
    ///
    /// let data = FBData::new();
    /// ```
    pub fn new() -> Self {
        Self::default()
    }

    /// Check if Facebook's FBPicture FBData has value
    ///
    /// Example
    /// ```
    /// use library::socials::FBData;
    ///
    /// let data = FBData::new();
    /// let is_none = data.is_none();
    /// ```
    pub fn is_none(&self) -> bool {
        let items = [
            self.clone().height,
            self.clone().width
        ];

        for item in items {
            if item.is_some() && !item.as_ref().unwrap().lt(&i32::from(1)) {
                return false;
            }
        }

        if (self.is_silhouette.is_some() &&!self.url.as_ref().unwrap().is_empty()) ||
            (self.is_silhouette.is_some() && self.is_silhouette.as_ref().unwrap() == &true) {
            return false;
        }

        true
    }
}

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
    /// Create new instance
    ///
    /// Example
    /// ```
    /// use library::socials::Google;
    ///
    /// let google = Google::new();
    /// ```
    pub fn new() -> Self {
        Self::default()
    }
}