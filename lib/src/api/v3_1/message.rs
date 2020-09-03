use crate::api::common::{Payload, Recipient};
use serde::{Serialize, Deserialize};
use serde_json::to_string as to_json_string;

/// Mailjet's SendAPI V3.1 Message
/// 
/// From the Mailjet SendAPI V3.1 documentation:
/// 
/// The recipients listed in `To` will receive a common message, showing every other recipient and carbon copy (CC) recipients.
/// If you do not wish the recipients to see each other, you have to create multiple messages in the Messages array. 
/// 
/// Source: https://dev.mailjet.com/email/guides/send-api-v31/
/// 
#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
  /// The sender of the email in question
  #[serde(rename = "From")]
  pub from: Recipient,
  /// The collection of recipients who will receive the email.
  #[serde(rename = "To")]
  pub to: Vec<Recipient>,
  /// The subject of the email. This field is optional and if
  /// is not provided then an empty string will be sent
  #[serde(rename = "Subject")]
  pub subject: String,
  /// The text content of the email
  #[serde(rename = "TextPart")]
  pub text_part: String,
  /// The HTML content of the email
  #[serde(rename = "HTMLPart")]
  pub html_part: Option<String>,
}

impl Message {
  pub fn new(from: Recipient, to: Vec<Recipient>, subject: Option<String>, text_part: String, html_part: Option<String>) -> Self {
    let mut final_subject = String::default();

    if subject.is_some() {
      final_subject = subject.unwrap();
    }

    Self {
      from,
      to,
      subject: final_subject,
      text_part,
      html_part,
    }
  }

  pub fn to_json(&self) -> String {
    to_json_string(self).unwrap()
  }
}

/// Collection of `Message` `structs` used by the SendAPI.
/// 
/// This `struct` represents the _root_ JSON object sent as the
/// payload for the HTTP request made to Mailjet
#[derive(Debug, Serialize, Deserialize)]
pub struct Messages {
  #[serde(rename = "Messages")]
  messages: Vec<Message>,
}

impl Messages {
  pub fn new(message: Message) -> Self {
    let mut messages = Vec::new();

    messages.push(message);

    Self {
      messages
    }
  }
}

impl Payload for Messages {
  fn to_json(&self) -> String {
    to_json_string(self).unwrap()
  }
}
