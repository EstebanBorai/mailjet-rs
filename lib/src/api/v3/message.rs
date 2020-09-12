use crate::v3::Attachment;
use crate::api::common::{Payload, Recipient, Recipients};
use serde::{Deserialize, Serialize, Serializer};
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
    /// The recipients to send the `Message`
    #[serde(rename = "To")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(serialize_with = "serialize_email_field")]
    pub to: Option<Recipients>,
    /// The carbon copy recipients
    #[serde(rename = "Cc")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(serialize_with = "serialize_email_field")]
    pub cc: Option<Recipients>,
    /// The blind carbon copy recipients
    #[serde(rename = "Bcc")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(serialize_with = "serialize_email_field")]
    pub bcc: Option<Recipients>,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_part: Option<String>,
    /// The HTML content of the email
    #[serde(rename = "Html-part")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html_part: Option<String>,
    #[serde(rename = "Recipients")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipients: Option<Recipients>,
    #[serde(rename = "Attachments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<Attachment>>,
    #[serde(rename = "Inline_attachments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_attachments: Option<Vec<Attachment>>,
}

impl Message {
    pub fn new(
        from_email: &str,
        from_name: &str,
        subject: Option<String>,
        text_part: Option<String>
    ) -> Self {
        Self {
            from_email: String::from(from_email),
            from_name: String::from(from_name),
            text_part,
            to: None,
            cc: None,
            bcc: None,
            subject,
            html_part: None,
            recipients: None,
            attachments: None,
            inline_attachments: None,
        }
    }

    /// Pushes a `Recipient` to the `Recipients` field of the `Message`
    pub fn push_recipient(&mut self, recipient: Recipient) {
        if self.have_email_fields_filled() {
            panic!(r#"Attempt to define `Recipients` fields with any of `To`, `Cc` and `Bcc` already defined. You must either define one or the other"#)
        }

        self.recipients.get_or_insert_with(Vec::new)
            .push(recipient);
    }

    /// Pushes every `Recipient` object into the `Recipients` field
    /// of the `Message`
    pub fn push_many_recipients(&mut self, recipients: Recipients) {
        if self.have_email_fields_filled() {
            panic!(r#"Attempt to define `Recipients` fields with any of `To`, `Cc` and `Bcc` already defined. You must either define one or the other"#)
        }

        recipients.into_iter().for_each(|recipient| {
            self.recipients.get_or_insert_with(Vec::new).push(recipient);
        });
    }

    /// Set the `To`, `Cc` and `Bcc` fields for the `Message`.
    /// 
    /// When calling this method any of the fields will be replaced
    /// with the values provided on the call.
    /// 
    /// ## Panic
    /// 
    /// When calling this method with `Recipients` defined (with a value different to `None`) this
    /// method will panic.
    /// 
    /// This is because Mailjet's Send API v3 documentation expects one of two ways to define recipients
    /// but never both:
    /// 
    /// > Optionally, in place of Recipients, you can use To, Cc and Bcc properties. `To`, `Cc` and `Bcc` can't be used in conjunction with `Recipients`
    /// 
    /// [Mailjet SendAPI V3 Documentation](https://dev.mailjet.com/email/guides/send-api-V3/#send-a-basic-email)
    pub fn set_receivers(&mut self, to: Recipients, cc: Option<Recipients>, bcc: Option<Recipients>) {
        if self.recipients.is_some() {
            panic!(r#"Attempt to define `To`, `Cc` and `Bcc` fields with `Recipients` already defined. You must either define one or the other"#)
        }

        self.to = Some(to);
        self.cc = cc;
        self.bcc = bcc;
    }

    /// Attach an `Attachment` to the `Message`
    /// The recipient of a email with attachment will
    /// have to click to see it. The inline attachment can be
    /// visible directly in the body of the message depending
    /// of the email client support.
    /// 
    /// The content will need to be Base64 encoded. You will need to specify the
    /// MIME type and a file name.
    ///
    /// Remember to keep the size of your attachements low and not to exceed 15 MB.
    pub fn attach(&mut self, attachment: Attachment) {
        self.attachments.get_or_insert_with(Vec::new)
            .push(attachment)
    }

    /// Attach an `Attachment` to the `Message`
    /// When using an inline Attachment, it's possible to insert
    /// the file inside the HTML code of the email by using cid:FILENAME.EXT
    /// where FILENAME.EXT is the Filename specified in the declaration of the Attachment.
    /// 
    /// The content will need to be Base64 encoded. You will need to specify the
    /// MIME type and a file name.
    ///
    /// Remember to keep the size of your attachements low and not to exceed 15 MB.
    pub fn attach_inline(&mut self, attachment: Attachment) {
        self.inline_attachments.get_or_insert_with(Vec::new)
            .push(attachment)
    }

    /// Checks for any of `To`, `Cc` or `Bcc` to be `Some`.
    /// 
    /// Used to validate if the `Recipients` could be filled or not
    fn have_email_fields_filled(&self) -> bool {
        self.to.is_some() || self.cc.is_some() || self.bcc.is_some()
    }
}

fn serialize_email_field<'a, S>(recipients: &'a std::option::Option<Recipients>, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    if recipients.is_some() {
        let repc = recipients.as_deref().unwrap();

        let as_comma_separated = repc.into_iter()
            .map(|v| v.as_comma_separated())
            .collect::<Vec<String>>()
            .join(",");

        return s.serialize_str(as_comma_separated.as_str());
    }

    s.serialize_none()
}

impl Payload for Message {
    fn to_json(&self) -> String {
        to_json_string(self).unwrap()
    }
}
