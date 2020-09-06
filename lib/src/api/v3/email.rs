use crate::api::common::{Payload, Recipient};
use serde::{Deserialize, Serialize};
use serde_json::to_string as to_json_string;

/// Mailjet's SendAPI V3 Email
///
/// Recipients listed in `To` will receive a common
/// message showing every other recipients and carbon copies recipients.
#[derive(Debug, Serialize, Deserialize)]
pub struct Email {
    /// The verified sender email address
    #[serde(rename = "FromEmail")]
    pub from_email: String,
    /// The name of the sender
    #[serde(rename = "FromName")]
    pub from_name: String,
    /// The subject of the email
    #[serde(rename = "Subject")]
    pub subject: Option<String>,
    /// The raw text content of the email
    #[serde(rename = "Text-part")]
    pub text_part: String,
    /// The HTML content of the email
    #[serde(rename = "Html-part")]
    pub html_part: Option<String>,
    #[serde(rename = "To")]
    pub to: Vec<String>,
    #[serde(rename = "Cc")]
    pub cc: Vec<String>,
    #[serde(rename = "Bcc")]
    pub bcc: Vec<String>,
}
