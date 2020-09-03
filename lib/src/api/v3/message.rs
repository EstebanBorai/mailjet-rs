use crate::api::common::Recipient;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Addresses {
  #[serde(rename = "To")]
  pub to: Vec<String>,
  #[serde(rename = "Cc")]
  pub cc: Vec<String>,
  #[serde(rename = "Bcc")]
  pub bcc: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Receiver {
  Recipients(Vec<Recipient>),
  Specific()
}

Important: Recipients and To have a different behaviors. The recipients listed in To will receive a common message
showing every other recipients and carbon copies recipients.
The recipients listed in Recipients will each receive an separate message without showing all the other recipients.

/// Mailjet's SendAPI V3 Message
#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
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
  #[serde(rename = "TextPart")]
  pub text_part: String,
  /// The HTML content of the email
  #[serde(rename = "HTMLPart")]
  pub html_part: Option<String>,
  #[serde(rename = "Recipients")]
  pub recipients: Receiver,
}
