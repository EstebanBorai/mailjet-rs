use serde::{Deserialize, Serialize};

/// Attachments to a email which should be clicked in
/// order to display
#[derive(Debug, Serialize, Deserialize)]
pub struct Attachments {
  #[serde(rename = "Content-type")]
  pub content_type: String,
  #[serde(rename = "Filename")]
  pub filename: String,
  pub content: String,
}

/// An attachment that is visible directly in the body of 
/// the message depending of the email client support
///
/// When using an inline Attachment, it's possible to insert the
/// file inside the HTML code of the email by using cid:FILENAME.EXT
/// where FILENAME.EXT is the Filename specified in the declaration of the Attachment.
#[derive(Debug, Serialize, Deserialize)]
pub struct InlineAttachments {
  #[serde(rename = "Content-type")]
  pub content_type: String,
  #[serde(rename = "Filename")]
  pub filename: String,
  pub content: String,
}

#[derive(Debug, Serialize, Deserialize)]
/// Email Attachment
pub enum Attachment {
  #[serde(rename = "Attachments")]
  Attachments(Vec<Attachments>),
  #[serde(rename = "Inline_attachments")]
  InlineAttachments(InlineAttachments)
}
