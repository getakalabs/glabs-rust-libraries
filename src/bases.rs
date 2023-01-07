/// Struct container for base options
#[derive(Clone, Debug)]
pub struct Base {
    pub api_url: String,
    pub web_url: String,
}

// Implement default for Base
impl Default for Base {
    fn default() -> Self {
        Self {
            api_url: String::default(),
            web_url: String::default()
        }
    }
}

// Create Base implementation
impl Base {
    /// Creates new base instance
    ///
    /// Example
    /// ```
    /// // Import base
    /// use library::Base;
    ///
    /// // Create new base instance
    /// let base = Base::new();
    /// ```
    pub fn new() -> Self {
        Self::default()
    }

    /// Clear current base instance values
    ///
    /// Example
    /// ```
    /// // Import base
    /// use library::Base;
    ///
    /// // Create new base instance
    /// let mut base = Base::new();
    /// base.api_url = String::from("http://localhost:8081/");
    /// base.web_url = String::from("http://localhost:3000/");
    ///
    /// // Clear base values
    /// // Output: Base{api_url: "", web_url: ""}
    /// base.clear();
    /// ```
    pub fn clear(&mut self) {
        self.api_url = String::default();
        self.web_url = String::default();
    }

    /// Reconfigures base variables (Useful under base configuration of API used by controller role to dynamically configure base urls)
    ///
    /// Example
    /// ```
    /// // Import base
    /// use library::Base;
    ///
    /// // Create new base instance
    /// let mut base = Base::new();
    /// base.api_url = String::from("http://localhost:8081/");
    /// base.web_url = String::from("http://localhost:3000/");
    ///
    /// // Create another base instance
    /// let mut another_base = Base::new();
    ///
    /// // Reconfigure new base instance
    /// another_base.reconfigure(base);
    /// ```
    pub fn reconfigure(&mut self, base: Base) {
        self.api_url = base.api_url.clone();
        self.web_url = base.web_url.clone();
    }
}