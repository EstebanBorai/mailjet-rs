use serde::{Serialize, Deserialize};
use serde_json::to_string as to_json_string;

#[derive(Debug, Serialize, Deserialize)]
pub struct Recipient {
  #[serde(rename = "Email")]
  pub email: String,
  #[serde(rename = "Name")]
  pub name: String,
}

impl Recipient {
  pub fn new(email: String, name: Option<String>) -> Self {
    if name.is_none() {
      return Self {
        email,
        name: String::default(),
      };
    }

    Self {
      email,
      name: name.unwrap()
    }
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
  #[serde(rename = "From")]
  pub from: Recipient,
  #[serde(rename = "To")]
  pub to: Vec<Recipient>,
  #[serde(rename = "Subject")]
  pub subject: String,
  #[serde(rename = "TextPart")]
  pub text_part: String,
  #[serde(rename = "HTMLPart")]
  pub html_part: String,
}

impl Message {
  pub fn new(from: Recipient, to: Vec<Recipient>, subject: String, text_part: String, html_part: String) -> Self {
    Self {
      from,
      to,
      subject,
      text_part,
      html_part,
    }
  }

  pub fn to_json(&self) -> String {
    to_json_string(self).unwrap()
  }
}

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

  pub fn to_json(&self) -> String {
    to_json_string(self).unwrap()
  }
}
