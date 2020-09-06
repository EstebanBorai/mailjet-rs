use crate::api::common::{Payload, Recipient};
use serde::{Deserialize, Serialize};
use serde_json::{json, to_string as to_json_string};

/// Mailjet's SendAPI V3 Email
///
/// Recipients listed in `To` will receive a common
/// message showing every other recipients and carbon copies recipients.
///
/// When using To, Cc and Bcc instead of Recipients, the email addresses need
/// to be presented as SMTP headers in a string and not as an array.
///
/// _Note: If a recipient does not exist in any of your contact list it will
/// be created from scratch, keep that in mind if you are planning on sending
/// a welcome email and then you're trying to add the email to a list as the contact
/// effectively exists already._
///
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
    pub to: Vec<Recipient>,
    #[serde(rename = "Cc")]
    pub cc: Option<Vec<Recipient>>,
    #[serde(rename = "Bcc")]
    pub bcc: Option<Vec<Recipient>>,
}

impl Email {
    pub fn new(
        from_email: &str,
        from_name: &str,
        subject: Option<String>,
        text_part: &str,
        html_part: Option<String>,
        to: Vec<Recipient>,
        cc: Option<Vec<Recipient>>,
        bcc: Option<Vec<Recipient>>,
    ) -> Self {
        Self {
            from_email: String::from(from_email),
            from_name: String::from(from_name),
            subject,
            text_part: String::from(text_part),
            html_part,
            to,
            cc,
            bcc,
        }
    }
}

impl Payload for Email {
    fn to_json(&self) -> String {
        let subject = self.subject.clone().unwrap_or(String::default());
        let html_part = self.html_part.clone().unwrap_or(String::default());
        let to_recipients = self.to.iter().map(|r| {
            r.as_comma_separated()
        }).collect::<Vec<String>>().join(",");

        let mut cc_recipients = String::default();
        let mut bcc_recipients = String::default();

        if let Some(cc) = &self.cc {
            cc_recipients = cc.iter().map(|r| {
                r.as_comma_separated()
            }).collect::<Vec<String>>().join(",");
        }

        if let Some(bcc) = &self.bcc {
            bcc_recipients = bcc.iter().map(|r| {
                r.as_comma_separated()
            }).collect::<Vec<String>>().join(",");
        }

        let as_json = json!({
            "FromEmail": self.from_email,
            "FromName": self.from_name,
            "Subject": subject,
            "Text-part": self.text_part,
            "Html-part": html_part,
            "To": to_recipients,
            "Cc": cc_recipients,
            "Bcc": bcc_recipients,
        });

        as_json.to_string()
    }
}
