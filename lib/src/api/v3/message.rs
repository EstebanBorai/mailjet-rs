use crate::api::common::{Payload, Recipient, Recipients};
use crate::v3::Attachment;
use serde::{Deserialize, Serialize, Serializer};
use serde_json::{to_string as to_json_string, Map, Value};

/// Error message to panic with when pushing to the `Recipients` vector
/// when receivers (`To`, `Cc`, `Bcc`) has been defined
pub const PUSHING_RECIPIENTS_WITH_RECEIVERS_ERROR_MESSAGE: &str = "Attempt to define `Recipients` fields with any of `To`, `Cc` and `Bcc` already defined. You must either define one or the other";

/// Error message to panic with when setting receivers (`To`, `Cc`, `Bcc`) with
/// recipients already defined
pub const SETTING_RECEIVERS_WITH_RECIPIENTS_ERROR_MESSAGE: &str = "Attempt to define `To`, `Cc` and `Bcc` fields with `Recipients` already defined. You must either define one or the other";

/// # Mailjet Send API v3 Message
///
/// ### Basic Message
///
/// A `Message` is created and sent to the `Recipient` defined.
/// This message neither contains HTML or is consuming a template, instead this `Message` contains
/// raw text.
///
/// ```ignore
/// use mailjet_rs::common::Recipient;
/// use mailjet_rs::v3::Message;
/// use mailjet_rs::{Client, SendAPIVersion};
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
///     // Create an instance of the Mailjet API client
///     // used to send the `Message` and also define your API
///     // credentials
///     let client = Client::new(
///         SendAPIVersion::V3,
///         "public_key",
///         "private_key",
///     );
///
///     // Create your a `Message` instance with the minimum required values
///     let mut message = Message::new(
///         "mailjet_sender@company.com",
///         "Mailjet Rust",
///         Some("Your email flight plan!".to_string()),
///         Some("Dear passenger, welcome to Mailjet! May the delivery force be with you!".to_string())
///     );
///
///     message.push_recipient(Recipient::new("receiver@company.com"));
///
///     // Finally send the message using the `Client`
///     let response = client.send(message).await;
///
///     // Do something with the response from Mailjet
///     // Ok(Response { sent: [Sent { email: "your_receiver@company.com", message_id: 000, message_uuid: "message-uuid" }] })
///     println!("{:?}", response);
///
///     Ok(())
/// }
/// ```
///
/// ### Send to multiple recipients
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
///     let mut message = Message::new(
///         "mailjet_sender@company.com",
///         "Mailjet Rust",
///         Some("Your email flight plan!".to_string()),
///         Some("Dear passenger, welcome to Mailjet! May the delivery force be with you!".to_string())
///     );
///
///     let recipients = vec![
///         Recipient::new("receiver1@company.com"),
///         Recipient::new("receiver2@company.com"),
///         Recipient::new("receiver3@company.com"),
///     ];
///
///     message.push_many_recipients(recipients);
///
///     let response = client.send(message).await;
///
///     println!("{:?}", response);
///
///     Ok(())
/// }
/// ```
///
/// ### Using `To`, `Cc` and `Bcc` instead of `Recipients`
///
/// > Note: If a recipient does not exist in any of your contact list it will be created from scratch, keep that in mind if you are planning on sending a welcome email and then you're trying to add the email to a list as the contact effectively exists already. [Mailjet's API Documentation](https://dev.mailjet.com/email/guides/send-api-V3/#send-a-basic-email)
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
///     let mut message = Message::new(
///         "mailjet_sender@company.com",
///         "Mailjet Rust",
///         Some("Your email flight plan!".to_string()),
///         Some("Dear passenger, welcome to Mailjet! May the delivery force be with you!".to_string())
///     );
///
///     message.set_receivers(
///         vec![
///             Recipient::new("bar@foo.com"),
///         ],
///         Some(vec![
///             Recipient::new("bee@foo.com"),
///         ]),
///         None
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
/// ### Send Inline Attachments
///
/// ```ignore
/// use mailjet_rs::common::Recipient;
/// use mailjet_rs::v3::{Message, Attachment};
/// use mailjet_rs::{Client, SendAPIVersion};
///
/// /// Base64 representation of the Mailjet logo found in the Mailjet SendAPI V3 docs
/// const MAILJET_LOGO_BASE64: &str = "iVBORw0KGgoAAAANSUhEUgAAABQAAAALCAYAAAB/Ca1DAAAACXBIWXMAAA7EAAAOxAGVKw4bAAAAB3RJTUUH4wIIChcxurq5eQAAAAd0RVh0QXV0aG9yAKmuzEgAAAAMdEVYdERlc2NyaXB0aW9uABMJISMAAAAKdEVYdENvcHlyaWdodACsD8w6AAAADnRFWHRDcmVhdGlvbiB0aW1lADX3DwkAAAAJdEVYdFNvZnR3YXJlAF1w/zoAAAALdEVYdERpc2NsYWltZXIAt8C0jwAAAAh0RVh0V2FybmluZwDAG+aHAAAAB3RFWHRTb3VyY2UA9f+D6wAAAAh0RVh0Q29tbWVudAD2zJa/AAAABnRFWHRUaXRsZQCo7tInAAABV0lEQVQokaXSPWtTYRTA8d9N7k1zm6a+RG2x+FItgpu66uDQxbFurrr5OQQHR9FZnARB3PwSFqooddAStCBoqmLtS9omx+ESUXuDon94tnP+5+1JYm057GyQjZFP+l+S6G2FzlNe3WHtHc2TNI8zOlUUGLxsD1kDyR+EEQE2P/L8Jm/uk6RUc6oZaYM0JxtnpEX9AGPTtM6w7yzVEb61EaSNn4QD3j5m4QabH6hkVFLSUeqHyCeot0ib6BdNVGscPM/hWWr7S4Tw9TUvbpFUitHTnF6XrS+sL7O6VBSausT0FZonSkb+nZUFFm+z8Z5up5Btr1Lby7E5Zq4yPrMrLR263ZV52g+LvfW3iy6PXubUNVrnhqYNF3bmiZ1i1MmLnL7OxIWh4T+IMpYeRNyrRzyZjWg/ioh+aVgZu4WfXxaixbsRve5fiwb8epTo8+kZjSPFf/sHvgNC0/mbjJbxPAAAAABJRU5ErkJggg==";
///
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
///     let client = Client::new(
///         SendAPIVersion::V3,
///         "public_key",
///         "private_key",
///     );
///
///     let mut message = Message::new(
///         "mailjet_sender@company.com",
///         "Mailjet Rust",
///         Some("Your email flight plan!".to_string()),
///         Some("Dear passenger, welcome to Mailjet! May the delivery force be with you!".to_string())
///     );
///
///     message.set_receivers(
///         vec![
///             Recipient::new("bar@foo.com"),
///         ],
///         Some(vec![
///             Recipient::new("bee@foo.com"),
///         ]),
///         None
///     );
///
///     let mailjet_logo = Attachment::new(
///         "image/png",
///         "logo.png",
///         MAILJET_LOGO_BASE64);
///
///     message.attach_inline(mailjet_logo);
///
///     message.html_part = Some("<h3>Dear [[var:name]] [[var:last]], welcome to <img src=\"cid:logo.png\"> <a href=\"https://www.mailjet.com/\">Mailjet</a>!<br />May the delivery force be with you!".to_string());
///
///
///     let response = client.send(message).await;
///
///     println!("{:?}", response);
///
///     Ok(())
/// }
/// ```
///
/// ### Full Featured Example on v3
///
/// The following is an example using the Mailjet's Send API v3 where the following
/// features are covered:
///
/// - Attach inline images
/// - Attach files
/// - Use template variables
///
/// ```ignore
/// use mailjet_rs::common::Recipient;
/// use mailjet_rs::v3::{Message, Attachment};
/// use mailjet_rs::{Client, SendAPIVersion};
/// use mailjet_rs::{Map, Value};
///
/// /// Base64 representation of the Mailjet logo found in the Mailjet SendAPI V3 docs
/// const MAILJET_LOGO_BASE64: &str = "iVBORw0KGgoAAAANSUhEUgAAABQAAAALCAYAAAB/Ca1DAAAACXBIWXMAAA7EAAAOxAGVKw4bAAAAB3RJTUUH4wIIChcxurq5eQAAAAd0RVh0QXV0aG9yAKmuzEgAAAAMdEVYdERlc2NyaXB0aW9uABMJISMAAAAKdEVYdENvcHlyaWdodACsD8w6AAAADnRFWHRDcmVhdGlvbiB0aW1lADX3DwkAAAAJdEVYdFNvZnR3YXJlAF1w/zoAAAALdEVYdERpc2NsYWltZXIAt8C0jwAAAAh0RVh0V2FybmluZwDAG+aHAAAAB3RFWHRTb3VyY2UA9f+D6wAAAAh0RVh0Q29tbWVudAD2zJa/AAAABnRFWHRUaXRsZQCo7tInAAABV0lEQVQokaXSPWtTYRTA8d9N7k1zm6a+RG2x+FItgpu66uDQxbFurrr5OQQHR9FZnARB3PwSFqooddAStCBoqmLtS9omx+ESUXuDon94tnP+5+1JYm057GyQjZFP+l+S6G2FzlNe3WHtHc2TNI8zOlUUGLxsD1kDyR+EEQE2P/L8Jm/uk6RUc6oZaYM0JxtnpEX9AGPTtM6w7yzVEb61EaSNn4QD3j5m4QabH6hkVFLSUeqHyCeot0ib6BdNVGscPM/hWWr7S4Tw9TUvbpFUitHTnF6XrS+sL7O6VBSausT0FZonSkb+nZUFFm+z8Z5up5Btr1Lby7E5Zq4yPrMrLR263ZV52g+LvfW3iy6PXubUNVrnhqYNF3bmiZ1i1MmLnL7OxIWh4T+IMpYeRNyrRzyZjWg/ioh+aVgZu4WfXxaixbsRve5fiwb8epTo8+kZjSPFf/sHvgNC0/mbjJbxPAAAAABJRU5ErkJggg==";
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
///
///     // Create an instance of the Mailjet API client
///     // used to send the `Message` and also define your API
///     // credentials
///     let client = Client::new(
///         SendAPIVersion::V3,
///         "public_key",
///         "private_key",
///     );
///
///     // Create your a `Message` instance with the minimum required values
///     let mut message = Message::new(
///         "mailjet_sender@company.com",
///         "Mailjet Rust",
///         Some("Your email flight plan!".to_string()),
///         Some("Dear passenger, welcome to Mailjet! May the delivery force be with you!".to_string())
///     );
///
///     message.push_recipient(Recipient::new("receiver@company.com"));
///
///     // Set some HTML for your email
///     //
///     // Note that here we are using `cid:logo.png` as the src value for our image
///     // this is using the `inline_attachment` with `filename` "logo.png" as the
///     // image source
///     message.html_part = Some("<h3>Dear [[var:name]] [[var:last]], welcome to <img src=\"cid:logo.png\"> <a href=\"https://www.mailjet.com/\">Mailjet</a>!<br />May the delivery force be with you!".to_string());
///
///     // Attach inline files providing its base64 representation
///     // content-type and a name.
///     // The name of the file can be used to reference this file in your HTML content
///     let mailjet_logo_inline = Attachment::new(
///       "image/png",
///       "logo.png",
///       MAILJET_LOGO_BASE64);
///
///     // Attach the `Attachment` as an Inline Attachment
///     // this function can also be used to attach common Attachments
///     message.attach_inline(mailjet_logo_inline);
///
///     // Creates a txt file Attachment
///     let txt_file_attachment = Attachment::new(
///       "text/plain",
///       "test.txt",
///       "VGhpcyBpcyB5b3VyIGF0dGFjaGVkIGZpbGUhISEK");
///
///     // Attaches the TXT file as an email Attachment
///     message.attach(txt_file_attachment);
///
///     // Provide variables for your template
///     // `Map` and `Value` are reexported from
///     // `serde_json`
///     let mut vars = Map::new();
///
///     vars.insert(String::from("name"), Value::from("Foo"));
///     vars.insert(String::from("last"), Value::from("Bar"));
///
///     message.vars = Some(vars);
///
///     // Finally send the message using the `Client`
///     let response = client.send(message).await;
///
///     // Do something with the response from Mailjet
///     // Ok(Response { sent: [Sent { email: "your_receiver@company.com", message_id: 000, message_uuid: "message-uuid" }] })
///     println!("{:?}", response);
///
///     Ok(())
/// }
/// ```
///
/// ## Reference
///
/// [Send API V3](https://dev.mailjet.com/email/guides/send-api-V3/)
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
    /// Variables for email templating
    #[serde(rename = "Vars")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vars: Option<Map<String, Value>>,
    /// ID provided by Passport at the end of your designing process or
    /// the ID returned by the /template resource.
    #[serde(rename = "Mj-TemplateID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mj_template_id: Option<usize>,
    /// Flag for Mailjet's `Message` to interpret the template language
    #[serde(rename = "Mj-TemplateLanguage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_mj_template_language: Option<bool>,
    /// Custom ID for the email
    #[serde(rename = "Mj-CustomID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mj_custom_id: Option<String>,
    #[serde(rename = "Mj-EventPayload")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mj_event_payload: Option<String>,
}

impl Message {
    /// Creates a new `Message` Send API v3 instance
    /// with the minimum requirements for a `Message`
    /// and remaining fields set as `None`.
    /// 
    /// Note that recipients must be defined later,
    /// either by using `push_recipient`, `push_many_recipients` or
    /// `set_receivers`.
    /// 
    pub fn new(
        from_email: &str,
        from_name: &str,
        subject: Option<String>,
        text_part: Option<String>,
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
            vars: None,
            mj_template_id: None,
            use_mj_template_language: None,
            mj_custom_id: None,
            mj_event_payload: None,
        }
    }

    /// Pushes a `Recipient` to the `Recipients` field of the `Message`
    pub fn push_recipient(&mut self, recipient: Recipient) {
        if self.have_email_fields_filled() {
            panic!(PUSHING_RECIPIENTS_WITH_RECEIVERS_ERROR_MESSAGE);
        }

        self.recipients.get_or_insert_with(Vec::new).push(recipient);
    }

    /// Pushes every `Recipient` object into the `Recipients` field
    /// of the `Message`
    pub fn push_many_recipients(&mut self, recipients: Recipients) {
        if self.have_email_fields_filled() {
            panic!(PUSHING_RECIPIENTS_WITH_RECEIVERS_ERROR_MESSAGE);
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
    pub fn set_receivers(
        &mut self,
        to: Recipients,
        cc: Option<Recipients>,
        bcc: Option<Recipients>,
    ) {
        if self.recipients.is_some() {
            panic!(SETTING_RECEIVERS_WITH_RECIPIENTS_ERROR_MESSAGE);
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
        self.attachments
            .get_or_insert_with(Vec::new)
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
        self.inline_attachments
            .get_or_insert_with(Vec::new)
            .push(attachment)
    }

    /// Sets the `Mj-TemplateID` property for the `Message` and also
    /// turns `true` the `Mj-TemplateLanguage`.
    ///
    /// This method is used when using a template language for your
    /// `Message`
    pub fn set_template_id(&mut self, id: usize) {
        self.mj_template_id = Some(id);
        self.use_mj_template_language = Some(true);
    }

    /// Tag Email Messages
    ///
    /// Sets the `Mj-CustomID` property for the `Message`.
    ///
    /// ## Mailjet SendAPI V3
    ///
    /// Sometimes you need to use your own ID in addition to
    /// ours to be able to trace back the message in our system easily.
    /// For this purpose we let you insert your own ID in the message.
    /// To achieve this, just pass the ID you want to use in the Mj-CustomID
    /// property.
    ///
    /// From then, your CustomID is linked to our own Message ID.
    /// You can also retrieve the message later by providing it to the `/message`
    /// resource CustomID filter.
    ///
    /// ```bash
    /// curl -s \
    ///     -X GET \
    ///     --user "$MJ_APIKEY_PUBLIC:$MJ_APIKEY_PRIVATE" \
    ///     https://api.mailjet.com/v3/REST/message?CustomID=<Your Custom ID>
    /// ```
    pub fn set_custom_id(&mut self, id: String) {
        self.mj_custom_id = Some(id);
    }

    /// Sets the `Mj-EventPayload` property for the `Message`.
    ///
    /// ## Mailjet SendAPI V3
    ///
    /// Sometimes, you need more than just an ID to represent the context
    /// to what a specific message is attached to. For this purpose, we let
    /// you insert a payload in the message which can be of any format (XML, JSON, CSV, etc).
    /// To take advantage of this, just pass the payload you want in the `Mj-EventPayLoad` property.
    pub fn set_event_payload(&mut self, payload: String) {
        self.mj_custom_id = Some(payload);
    }

    /// Checks for any of `To`, `Cc` or `Bcc` to be `Some`.
    ///
    /// Used to validate if the `Recipients` could be filled or not
    fn have_email_fields_filled(&self) -> bool {
        self.to.is_some() || self.cc.is_some() || self.bcc.is_some()
    }
}

fn serialize_email_field<'a, S>(
    recipients: &'a std::option::Option<Recipients>,
    s: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    if recipients.is_some() {
        let repc = recipients.as_deref().unwrap();

        let as_comma_separated = repc
            .into_iter()
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_creates_a_message_instance() {
        let message = Message::new(
            "test@company.com",
            "Company",
            Some("Subject".to_string()),
            Some("Text Part".to_string()),
        );

        assert_eq!(message.from_email, "test@company.com".to_string());
        assert_eq!(message.from_name, "Company".to_string());
        assert_eq!(message.subject.unwrap(), "Subject".to_string());
        assert_eq!(message.text_part.unwrap(), "Text Part".to_string());
        assert_eq!(message.html_part, None);
        assert_eq!(message.vars, None);
        assert_eq!(message.mj_template_id, None);
        assert_eq!(message.use_mj_template_language, None);
        assert_eq!(message.mj_custom_id, None);
        assert_eq!(message.mj_event_payload, None);
    }

    #[test]
    #[should_panic(
        expected = "Attempt to define `Recipients` fields with any of `To`, `Cc` and `Bcc` already defined. You must either define one or the other"
    )]
    fn it_panics_if_push_recipients_with_receivers() {
        let mut message = Message::new(
            "test@company.com",
            "Company",
            Some("Subject".to_string()),
            Some("Text Part".to_string()),
        );

        message.set_receivers(vec![], None, None);

        message.push_recipient(Recipient::new("test@company.com"));
    }

    #[test]
    #[should_panic(
        expected = "Attempt to define `Recipients` fields with any of `To`, `Cc` and `Bcc` already defined. You must either define one or the other"
    )]
    fn it_panics_if_push_many_recipients_with_receivers() {
        let mut message = Message::new(
            "test@company.com",
            "Company",
            Some("Subject".to_string()),
            Some("Text Part".to_string()),
        );

        message.set_receivers(vec![], None, None);

        message.push_many_recipients(vec![Recipient::new("test@company.com")]);
    }

    #[test]
    #[should_panic(
        expected = "Attempt to define `To`, `Cc` and `Bcc` fields with `Recipients` already defined. You must either define one or the other"
    )]
    fn it_panics_if_setting_receivers_with_recipients() {
        let mut message = Message::new(
            "test@company.com",
            "Company",
            Some("Subject".to_string()),
            Some("Text Part".to_string()),
        );

        message.push_recipient(Recipient::new("test@company.com"));

        message.set_receivers(vec![], None, None);
    }

    #[test]
    fn it_attaches_an_attachment() {
        let mut message = Message::new(
            "test@company.com",
            "Company",
            Some("Subject".to_string()),
            Some("Text Part".to_string()),
        );

        let attachment = Attachment::new("text/plain", "filename", "base64");

        message.attach(attachment);

        let message_attachment = message.attachments.unwrap();
        let message_attachment = message_attachment.get(0).unwrap();

        assert_eq!(message_attachment.content_type, "text/plain");
        assert_eq!(message_attachment.filename, "filename");
        assert_eq!(message_attachment.content, "base64");
    }

    #[test]
    fn it_attaches_an_inline_attachment() {
        let mut message = Message::new(
            "test@company.com",
            "Company",
            Some("Subject".to_string()),
            Some("Text Part".to_string()),
        );

        let attachment = Attachment::new("text/plain", "filename", "base64");

        message.attach_inline(attachment);

        let message_attachment = message.inline_attachments.unwrap();
        let message_attachment = message_attachment.get(0).unwrap();

        assert_eq!(message_attachment.content_type, "text/plain");
        assert_eq!(message_attachment.filename, "filename");
        assert_eq!(message_attachment.content, "base64");
    }

    #[test]
    fn it_sets_template_id() {
        let mut message = Message::new(
            "test@company.com",
            "Company",
            Some("Subject".to_string()),
            Some("Text Part".to_string()),
        );

        message.set_template_id(1);

        assert_eq!(message.mj_template_id, Some(1));
        assert_eq!(message.use_mj_template_language, Some(true));
    }

    #[test]
    fn it_sets_event_payload() {
        let mut message = Message::new(
            "test@company.com",
            "Company",
            Some("Subject".to_string()),
            Some("Text Part".to_string()),
        );

        message.set_custom_id("1".to_string());

        assert_eq!(message.mj_custom_id, Some("1".to_string()));
    }

    #[test]
    fn it_checks_for_receivers() {
        let mut message = Message::new(
            "test@company.com",
            "Company",
            Some("Subject".to_string()),
            Some("Text Part".to_string()),
        );

        assert_eq!(message.have_email_fields_filled(), false);

        message.set_receivers(vec![], None, None);

        assert_eq!(message.have_email_fields_filled(), true);
    }
}

