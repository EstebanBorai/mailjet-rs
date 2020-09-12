use serde::{Deserialize, Serialize};

/// An email attachment
#[derive(Debug, Serialize, Deserialize)]
pub struct Attachment {
    #[serde(rename = "Content-type")]
    pub content_type: String,
    #[serde(rename = "Filename")]
    pub filename: String,
    pub content: String,
}

impl Attachment {
    pub fn new(content_type: String, filename: String, content: String) -> Self {
        Self {
            content_type,
            filename,
            content,
        }
    }
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
