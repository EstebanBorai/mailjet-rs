use serde::{Deserialize, Serialize};

/// An email attachment for both inline and not inline
/// attachment
#[derive(Debug, Serialize, Deserialize)]
pub struct Attachment {
    #[serde(rename = "Content-type")]
    pub content_type: String,
    #[serde(rename = "Filename")]
    pub filename: String,
    pub content: String,
}

impl Attachment {
    pub fn new(content_type: &str, filename: &str, content: &str) -> Self {
        Self {
            content_type: String::from(content_type),
            filename: String::from(filename),
            content: String::from(content),
        }
    }
}
