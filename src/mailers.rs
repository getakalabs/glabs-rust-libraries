use lettre::{Message, SmtpTransport, Transport};
use lettre::message::{header, MultiPart, SinglePart};
use lettre::transport::smtp::authentication::Credentials;

use crate::Errors;
use crate::traits::Configurable;

/// Mailer struct contains commonly used smtp email credentials
#[derive(Clone, Debug)]
pub struct Mailer {
    pub sender: String,
    pub username: String,
    pub password: String,
    pub smtp_host: String,
    pub service: String
}

// Implement default for Mailer
impl Default for Mailer {
    fn default() -> Self {
        Self {
            sender: String::default(),
            username: String::default(),
            password: String::default(),
            smtp_host: String::default(),
            service: String::default()
        }
    }
}

// Set Configurable trait
impl Configurable<Mailer> for Mailer {
    /// Implement new instance
    ///
    /// Example
    /// ```
    /// use library::Mailer;
    /// use library::traits::Configurable;
    ///
    /// fn main() {
    ///     // Create new mailer instance with default values
    ///     let mailer = Mailer::new();
    /// }
    /// ```
    fn new() -> Self {
        Self::default()
    }

    /// Clear current instance
    ///
    /// Example
    /// ```
    /// use library::Mailer;
    /// use library::traits::Configurable;
    ///
    /// fn main() {
    ///     // Create new mailer instance with default values
    ///     let mut mailer = Mailer::new();
    ///     mailer.clear();
    /// }
    /// ```
    fn clear(&mut self) -> Self {
        Self::default()
    }

    /// Reconfigure instance
    ///
    /// Example
    /// ```
    /// use library::Mailer;
    /// use library::traits::Configurable;
    ///
    /// fn main() {
    ///     // Create old mailer instance with default values
    ///     let mut old_mailer = Mailer::new();
    ///
    ///     // Create new mailer instance with new values
    ///     let mut new_mailer = Mailer::new();
    ///     new_mailer.smtp_host = String::from("smtp.some-host.com");
    ///
    ///     // Reconfigure
    ///     old_mailer.reconfigure(&new_mailer);
    /// }
    /// ```
    fn reconfigure(&mut self, item: &Mailer) {
        self.sender = item.clone().sender;
        self.username = item.clone().username;
        self.password = item.clone().password;
        self.service = item.clone().service;
        self.smtp_host = item.clone().smtp_host;
    }

    /// Check if current instance has no value
    ///
    /// Example
    /// ```
    /// use library::Mailer;
    /// use library::traits::Configurable;
    ///
    /// fn main() {
    ///     // Create new mailer instance with default values
    ///     let mailer = Mailer::new();
    ///     let is_valid = mailer.is_none();
    /// }
    /// ```
    fn is_none(&self) -> bool {
        let items = [
            self.clone().sender,
            self.clone().username,
            self.clone().password,
            self.clone().service,
            self.clone().smtp_host
        ];

        for item in items {
            if !item.is_empty() {
                return false
            }
        }

        true
    }
}

// Mailer implementation
impl Mailer {
    /// Sends email
    ///
    /// Example
    /// ```
    /// // Import mailer error
    /// use library::mailers::Mailer;
    /// use library::traits::Configurable;
    ///
    /// // Set mailer
    /// let mailer = Mailer::new();
    /// let result = mailer.send_mail("johndoe@gmail.com", "My Subject", "My message");
    /// ```
    pub fn send_mail(&self, to: &str, subject: &str, body: &str) -> Result<String, Errors> {
        // Check if self has data
        if self.is_none() {
            return Err(Errors::new("Your platform's email configuration is invalid. Please contact your administrator"));
        }

        // Create multipart body
        let multipart = MultiPart::alternative()
            .singlepart(
            SinglePart::builder()
                    .header(header::ContentType::TEXT_HTML)
                    .body(body.to_string())
            );

        // Create email builder
        let builder = Message::builder()
            .from(self.sender.parse().unwrap())
            .to(to.parse().unwrap())
            .subject(subject)
            .multipart(multipart);

        // If builder encounters an error
        if builder.is_err() {
            return Err(Errors::new(&builder.unwrap_err().to_string()));
        }

        // Set credentials
        let credentials = Credentials::new(self.username.clone(), self.password.clone());

        // Set smtp transport relay
        let relay = SmtpTransport::relay(self.smtp_host.as_str());
        if relay.is_err() {
            return Err(Errors::new(&relay.unwrap_err().to_string()));
        }

        // Open a remote connection
        let mailer = relay.unwrap().credentials(credentials).build();

        // Send the email
        match mailer.send(&builder.unwrap()) {
            Ok(_) => Ok(format!("Email send successfully to {}", to)),
            Err(e) => Err(Errors::new(&e.to_string())),
        }
    }
}
