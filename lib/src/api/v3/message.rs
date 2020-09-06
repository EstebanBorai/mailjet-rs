use crate::v3::{Attachments, InlineAttachments};
use crate::api::common::{Payload, Recipient};
use serde::{Deserialize, Serialize};
use serde_json::to_string as to_json_string;

/// Mailjet's SendAPI V3 Message
///
/// Recipients listed in the `Recipients` `Vec` will
/// each receive a separate message without showing all other
/// recipients.
///
/// # Example
///
/// ```ignore
/// use mailjet_rs::common::Recipient;
/// use mailjet_rs::v3::Message;
/// use mailjet_rs::{Client, SendAPIVersion};
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
///     let client = Client::new(
///         SendAPIVersion::V3,
///         "public_key",
///         "private_key",
///     );
///
///     let recipients = vec![Recipient::new("receiver.email@mail.com")];
///
///     let message = Message::new(
///         "your.mailjet.email@yourcompany.com",
///         "Sender Name",
///         Some(String::from("Testing Mailjet Rust with Send API v3 Message")),
///         "This is a test on mailjet-rs with Send API v3 sending a basic email",
///         Some(String::from("<h1>Some HTML to give it a try</h1>")),
///         recipients,
///     );
///
///     client.send(message).await;
///     Ok(())
/// }
/// ```
///
/// ## Send to multiple recipients
///
/// To send the same email to multiple contacts, add multiple `Recipient` intances
/// to the `recipients` field.
///
/// Each recipient will receive a dedicated message.
///
/// ```ignore
/// use mailjet_rs::common::Recipient;
/// use mailjet_rs::v3::Message;
/// use mailjet_rs::{Client, SendAPIVersion};
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
///     let client = Client::new(
///         SendAPIVersion::V3,
///         "public_key",
///         "private_key",
///     );
///
///     let recipients = vec![
///         Recipient::new("receiver.email@mail.com"),
///         Recipient::new("foo@bar.com"),
///         Recipient::new("bar@baz.com")];
///
///     let message = Message::new(
///         "your.mailjet.email@yourcompany.com",
///         "Sender Name",
///         Some(String::from("Testing Mailjet Rust with Send API v3 Message")),
///         "This is a test on mailjet-rs with Send API v3 sending a basic email",
///         Some(String::from("<h1>Some HTML to give it a try</h1>")),
///         recipients,
///     );
///
///     let response = client.send(message).await;
///
///     println!("{:?}", response);
///
///     Ok(())
/// }
/// ```
///
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
    #[serde(rename = "Text-part")]
    pub text_part: String,
    /// The HTML content of the email
    #[serde(rename = "Html-part")]
    pub html_part: Option<String>,
    #[serde(rename = "Recipients")]
    pub recipients: Vec<Recipient>,
    #[serde(rename = "Attachments")]
    pub attachments: Option<Vec<Attachments>>,
    #[serde(rename = "Inline_attachments")]
    pub inline_attachments: Option<Vec<InlineAttachments>>,
}

impl Message {
    pub fn new(
        from_email: &str,
        from_name: &str,
        subject: Option<String>,
        text_part: &str,
        html_part: Option<String>,
        recipients: Vec<Recipient>,
    ) -> Self {
        Self {
            from_email: String::from(from_email),
            from_name: String::from(from_name),
            subject,
            text_part: String::from(text_part),
            html_part,
            recipients,
            attachments: None,
            inline_attachments: None,
        }
    }

    // pub fn attach<'a>(&mut self, file: &'a File) -> Result<(), ()> {
    //     let metadata = file.metadata().unwrap();
    //     let filename = metadata.len();


    //     Ok(())
    // }
}

impl Payload for Message {
    fn to_json(&self) -> String {
        to_json_string(self).unwrap()
    }
}
