use serde::{Serialize, Deserialize};

/// Email recipient composed by an email address and
/// the name of the owner
#[derive(Debug, Serialize, Deserialize)]
pub struct Recipient {
  #[serde(rename = "Email")]
  pub email: String,
  #[serde(rename = "Name")]
  pub name: String,
}

impl Recipient {
  /// Creates a new `Recipient` instance. As the `name` field is optional
  /// if `None` is provided then the `name` property will be sent as
  /// an empty string value.
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
