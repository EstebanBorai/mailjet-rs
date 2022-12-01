use serde::{Deserialize, Serialize};

/// An email attachment for both inline and not inline
/// attachments
///
/// This struct is set either behind the `Attachments` or
/// `Inline_attachments` to the `Message`.
///
/// ## Attachments
///
/// ```json
/// "Attachments":[{"Content-type":"text/plain","Filename":"test.txt","content":"VGhpc..."}]
/// ```
///
/// ## Inline Attachments
///
/// ```json
/// "Inline_attachments":[{"Content-type":"image/png","Filename":"logo.png","content":"iVBOR..."}]
/// ```
///
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Attachment {
    #[serde(rename = "Content-type")]
    pub content_type: String,
    #[serde(rename = "Filename")]
    pub filename: String,
    pub content: String,
}

impl Attachment {
    /// Creates a new `Attachment` instance
    pub fn new(content_type: &str, filename: &str, content: &str) -> Self {
        Self {
            content_type: String::from(content_type),
            filename: String::from(filename),
            content: String::from(content),
        }
    }
}
