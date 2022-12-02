use bstr::ByteSlice;
use rand::Rng;
use xsalsa20poly1305::aead::{Aead, KeyInit};
use xsalsa20poly1305::aead::generic_array::{GenericArray, typenum};
use xsalsa20poly1305::XSalsa20Poly1305;

use crate::Errors;

/// Generate cipher key
///
/// Example
/// ```
/// use library::ciphers;
///
/// fn main() {
///     // Set cipher key and print
///     let key = ciphers::generate();
/// }
/// ```
pub fn generate() -> String {
    base64_url::encode(&rand::thread_rng().gen::<[u8; 32]>())
}

/// <p>Cipher struct contains cipher specific information</p>
/// <p><b>Note:</b> This requires 2 environment variables.</p>
/// <b>MASTER_KEY</b> - This will encrypt everything on a master level.<br/>
/// <b>WEB_KEY</b> - Every frontend related encryption will use web key.
#[derive(Clone)]
pub struct Cipher {
    pub master: Option<XSalsa20Poly1305>,
    pub web: Option<XSalsa20Poly1305>,
}

/// Default implementation for Cipher
impl Default for Cipher {
    fn default() -> Self {
        Self { master: None, web: None }
    }
}

/// Cipher implementations
impl Cipher {
    /// Create new cipher
    ///
    /// Example
    /// ```
    /// use library::ciphers::Cipher;
    ///
    /// fn main() {
    ///     // Initialize cipher keys by retrieving env variables for master and web keys
    ///     let cipher = Cipher::new();
    /// }
    /// ```
    pub fn new() -> Result<Self, Errors> {
        // Retrieve master key
        let result = std::env::var("MASTER_KEY");
        if result.is_err() {
            return Err(Errors::new("Master key is missing"));
        }

        // Decode master key
        let result = base64_url::decode(&result.unwrap());
        if result.is_err() {
            return Err(Errors::new("Invalid master key"))
        }

        // Set master key
        let master_key = result.unwrap();

        // Retrieve web key
        let result = std::env::var("WEB_KEY");
        if result.is_err() {
            return Err(Errors::new("Web key is missing"));
        }

        // Decode master key
        let result = base64_url::decode(&result.unwrap());
        if result.is_err() {
            return Err(Errors::new("Invalid web key"))
        }

        // Set web key
        let web_key = result.unwrap();

        // Set cipher
        let mut cipher = Self::default();

        // Set cipher's master key
        let key = GenericArray::from_slice(&master_key);
        cipher.master = Some(XSalsa20Poly1305::new(&key));

        // Set cipher's web key
        let key = GenericArray::from_slice(&web_key);
        cipher.web = Some(XSalsa20Poly1305::new(&key));

        // Return cipher
        Ok(cipher)
    }

    /// Encrypt string through hash
    ///
    /// Example
    /// ```
    /// use library::Cipher;
    ///
    /// fn main() {
    ///     // Initialize cipher keys by retrieving env variables for master and web keys
    ///     let result = Cipher::new();
    ///
    ///     // Check if cipher result is ok
    ///     if result.is_ok() {
    ///         // Set cipher
    ///         let cipher = result.unwrap();
    ///
    ///         // Create hash and content
    ///         let hash = cipher.generate();
    ///         let content = "Some string here";
    ///         let result = cipher.encrypt_hash(content, &hash);
    ///     }
    /// }
    /// ```
    pub fn encrypt_hash<C, H>(&self, content: C, hash: H) -> Result<String, Errors>
        where C: Into<Vec<u8>>,
              H: Into<String>
    {
        return match base64_url::decode(&hash.into()) {
            Ok(b64_decoded_hash) => {
                let nonce = XSalsa20Poly1305::generate_nonce(&mut rand::rngs::OsRng);
                let cipher = XSalsa20Poly1305::new(GenericArray::from_slice(&b64_decoded_hash));

                let text = cipher.encrypt(&nonce, content.into().as_bytes());

                self.complete_encryption(&mut nonce.clone().to_vec(), text)
            },
            Err(_) => Err(Errors::new("Unable to decode base64 encoding"))
        }
    }

    /// Encrypt string through master key
    ///
    /// Example
    /// ```
    /// use library::Cipher;
    ///
    /// fn main() {
    ///     // Initialize cipher keys by retrieving env variables for master and web keys
    ///     let result = Cipher::new();
    ///
    ///     // Check if cipher result is ok
    ///     if result.is_ok() {
    ///         // Set cipher
    ///         let cipher = result.unwrap();
    ///
    ///         // Create ontent
    ///         let content = "Some string here";
    ///         let result = cipher.encrypt_master(content);
    ///     }
    /// }
    /// ```
    pub fn encrypt_master<T: Into<String>>(&self, str: T) -> Result<String, Errors> {
        // Set none
        let nonce = XSalsa20Poly1305::generate_nonce(&mut rand::rngs::OsRng);

        // Check if master key is available
        if self.master.is_none() {
            return Err(Errors::new("Cipher failed to initialize"));
        }

        // Encrypt using master key
        let result = self.master
            .as_ref()
            .unwrap()
            .encrypt(&nonce, str.into().as_bytes());

        // Complete encryption
        self.complete_encryption(&mut nonce.clone().to_vec(), result)
    }

    /// Encrypt string through web key
    ///
    /// Example
    /// ```
    /// use library::Cipher;
    ///
    /// fn main() {
    ///     // Initialize cipher keys by retrieving env variables for master and web keys
    ///     let result = Cipher::new();
    ///
    ///     // Check if cipher result is ok
    ///     if result.is_ok() {
    ///         // Set cipher
    ///         let cipher = result.unwrap();
    ///
    ///         // Create ontent
    ///         let content = "Some string here";
    ///         let result = cipher.encrypt_web(content);
    ///     }
    /// }
    /// ```
    pub fn encrypt_web<T: Into<String>>(&self, str: T) -> Result<String, Errors> {
        // Set none
        let nonce = XSalsa20Poly1305::generate_nonce(&mut rand::rngs::OsRng);

        // Check if web key is available
        if self.web.is_none() {
            return Err(Errors::new("Cipher failed to initialize"));
        }

        // Encrypt using web key
        let result = self.web
            .as_ref()
            .unwrap()
            .encrypt(&nonce, str.into().as_bytes());

        // Complete encryption
        self.complete_encryption(&mut nonce.clone().to_vec(), result)
    }

    /// Decrypt string through hash
    ///
    /// Example
    /// ```
    /// use library::Cipher;
    ///
    /// fn main() {
    ///     // Initialize cipher keys by retrieving env variables for master and web keys
    ///     let result = Cipher::new();
    ///
    ///     // Check if cipher result is ok
    ///     if result.is_ok() {
    ///         // Set cipher
    ///         let cipher = result.unwrap();
    ///
    ///         // Create hash and content
    ///         let hash = cipher.generate();
    ///         let content = "JmShvSN40GyHIDMwIszpnoJNPeSkVhNPivvjfwXLgTo8zsqSpJGRZicqAAGt8dhptv4arfrqf0XN72HYk1BrX-evTxHsHUjp3Ge8m8zAhoUfZ2A";
    ///         let result = cipher.decrypt_hash(content, &hash);
    ///     }
    /// }
    /// ```
    pub fn decrypt_hash<C, H>(&self, content: C, hash: H) -> Result<Vec<u8>, Errors>
        where C: Into<String>,
              H: Into<String>
    {
        // Decrypt hash from master
        let result = base64_url::decode(&String::from_utf8_lossy(&hash.into().as_bytes()).to_string());
        if result.is_err() {
            return Err(Errors::new("Unable to decode base64 encoding"));
        }

        // Set decoded hash
        let hash = result.unwrap();

        // Set cipher
        let cipher = XSalsa20Poly1305::new(GenericArray::from_slice(&hash));

        // Start decrypting using hash
        let result = base64_url::decode(&String::from_utf8_lossy(content.into().as_bytes()).to_string());
        if result.is_err() {
            return Err(Errors::new("Unable to decrypt text"));
        }

        // Set decoded hash
        let decoded = result.unwrap();
        if decoded.len() <= 24 {
            return Err(Errors::new("Invalid hash length"));
        }

        // Set chunks
        let nonce = &decoded[0..24];
        let message = &decoded[24..];

        // Set nonce
        let nonce:&GenericArray<u8, typenum::U24> = GenericArray::from_slice(nonce);

        // Unseal hash
        let unsealed = cipher.decrypt(nonce, message);
        if unsealed.is_err() {
            return Err(Errors::new("Unable to decrypt text"));
        }

        // Return unsealed hash
        Ok(unsealed.unwrap())
    }

    /// Decrypt string through hash with double encryption
    ///
    /// Example
    /// ```
    /// use bstr::ByteSlice;
    /// use library::Cipher;
    ///
    /// fn main() {
    ///     // Initialize cipher keys by retrieving env variables for master and web keys
    ///     let result = Cipher::new();
    ///
    ///     // Check if cipher result is ok
    ///     if result.is_ok() {
    ///         // Set cipher
    ///         let cipher = result.unwrap();
    ///
    ///         // Create hash and content
    ///         let hash = cipher.generate();
    ///         let encrypted = cipher.encrypt_hash("test", &hash);
    ///         let content = encrypted.unwrap();
    ///
    ///         // Encrypt hash
    ///         let hash = cipher.encrypt_master(&hash);
    ///
    ///         let decrypted = cipher.decrypt_deep_hash(&content, hash.unwrap().clone());
    ///          let content = String::from_utf8_lossy(decrypted.unwrap().as_bytes()).to_string();
    ///     }
    /// }
    /// ```
    pub fn decrypt_deep_hash<C, H>(&self, content: C, hash: H) -> Result<Vec<u8>, Errors>
        where C: Into<String>,
              H: Into<String>
    {
        // Decrypt hash from master
        let result = base64_url::decode(&String::from_utf8_lossy(&hash.into().as_bytes()).to_string());
        if result.is_err() {
            return Err(Errors::new("Unable to decode base64 encoding"));
        }

        // Set decoded hash
        let hash = result.unwrap();

        // Check hash length
        if hash.len() <= 24 {
            return Err(Errors::new("Invalid hash length"));
        }

        // Set chunks
        let nonce = &hash[0..24];
        let message = &hash[24..];

        // Shadow nonce
        let nonce:&GenericArray<u8, typenum::U24> = GenericArray::from_slice(nonce);

        // Unseal hash
        let unsealed = self.master
            .as_ref()
            .unwrap()
            .decrypt(nonce, message);

        // If unsealed hash is not valid return
        if unsealed.is_err() {
            return Err(Errors::new("Unable to decrypt text"));
        }

        // Decode unsealed hash
        let unsealed = base64_url::decode(&unsealed.unwrap());
        if unsealed.is_err() {
            return Err(Errors::new("Unable to decrypt text"));
        }

        // Set cipher
        let hash = base64_url::encode(&unsealed.unwrap());

        // Decrypt hash
        self.decrypt_hash(content, &hash)
    }

    /// Decrypt using master key
    ///
    /// Example
    /// ```
    /// use library::Cipher;
    ///
    /// fn main() {
    ///     // Initialize cipher keys by retrieving env variables for master and web keys
    ///     let result = Cipher::new();
    ///
    ///     // Check if cipher result is ok
    ///     if result.is_ok() {
    ///        let cipher = result.unwrap();
    ///        let content = "JmShvSN40GyHIDMwIszpnoJNPeSkVhNPivvjfwXLgTo8zsqSpJGRZicqAAGt8dhptv4arfrqf0XN72HYk1BrX-evTxHsHUjp3Ge8m8zAhoUfZ2A";
    ///        let content = cipher.decrypt_master(content);
    ///    }
    /// }
    /// ```
    pub fn decrypt_master<T: Into<String>>(&self, str: T) -> Result<String, Errors> {
        return match base64_url::decode(&str.into()) {
            Ok(b64_decoded) => {
                // Check b64_decoded's length
                if b64_decoded.clone().len() < 25 {
                    return Err(Errors::new("Invalid hash length"));
                }

                // Set nonce & message
                let nonce = &b64_decoded[0..24];
                let message = &b64_decoded[24..];

                // Shadow nonce
                let nonce:&GenericArray<u8, typenum::U24> = GenericArray::from_slice(nonce);

                // Unseal text
                let unsealed = self.master
                    .as_ref()
                    .unwrap()
                    .decrypt(nonce, message);

                // Check if unsealed text has problems
                if unsealed.is_err() {
                    return Err(Errors::new("Unable to decrypt text"));
                }

                Ok(String::from_utf8_lossy(&unsealed.unwrap()).to_string())
            },
            _ => Err(Errors::new("Unable to decode base64 encoding"))
        }
    }

    /// Decrypt string using web key
    ///
    /// Example
    /// ```
    /// use library::Cipher;
    ///
    /// fn main() {
    ///     // Initialize cipher keys by retrieving env variables for master and web keys
    ///     let result = Cipher::new();
    ///
    ///     // Check if cipher result is ok
    ///     if result.is_ok() {
    ///        let cipher = result.unwrap();
    ///        let content = "JmShvSN40GyHIDMwIszpnoJNPeSkVhNPivvjfwXLgTo8zsqSpJGRZicqAAGt8dhptv4arfrqf0XN72HYk1BrX-evTxHsHUjp3Ge8m8zAhoUfZ2A";
    ///        let content = cipher.decrypt_web(content);
    ///    }
    /// }
    /// ```
    pub fn decrypt_web<T: Into<String>>(&self, str: T) -> Result<String, Errors> {
        return match base64_url::decode(&str.into()) {
            Ok(b64_decoded) => {
                // Check b64_decoded's length
                if b64_decoded.clone().len() < 25 {
                    return Err(Errors::new("Invalid hash length"));
                }

                // Set nonce & message
                let nonce = &b64_decoded[0..24];
                let message = &b64_decoded[24..];

                // Shadow nonce
                let nonce:&GenericArray<u8, typenum::U24> = GenericArray::from_slice(nonce);

                // Unseal text
                let unsealed = self.web
                    .as_ref()
                    .unwrap()
                    .decrypt(nonce, message);

                // Check if unsealed text has problems
                if unsealed.is_err() {
                    return Err(Errors::new("Unable to decrypt text"));
                }

                Ok(String::from_utf8_lossy(&unsealed.unwrap()).to_string())
            },
            _ => Err(Errors::new("Unable to decode base64 encoding"))
        }
    }

    /// Imports generate cipher to self
    ///
    /// Example
    /// ```
    /// use library::Cipher;
    ///
    /// fn main() {
    ///     // Initialize cipher keys by retrieving env variables for master and web keys
    ///     let result = Cipher::new();
    ///
    ///     // Check if cipher result is ok
    ///     if result.is_ok() {
    ///        let cipher = result.unwrap();
    ///        let key = cipher.generate();
    ///     }
    /// }
    /// ```
    pub fn generate(&self) -> String {
        generate()
    }

    /// Completes the encryption of text
    fn complete_encryption<E>(&self, nonce: &mut Vec<u8>, result: Result<Vec<u8>, E>) -> Result<String, Errors> {
        return match result {
            Ok(mut value) => {
                nonce.append(&mut value);

                Ok(base64_url::encode(&nonce))
            },
            Err(_) => return Err(Errors::new("Encryption failed"))
        }
    }
}