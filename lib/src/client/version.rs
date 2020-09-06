/// Mailjet SendAPI version to use
pub enum SendAPIVersion {
    /// Consumes the SendAPI Version 3
    ///
    /// https://dev.mailjet.com/email/guides/send-api-v31/
    V3,
    /// Consumes the SendAPI Version 3.1
    ///
    /// https://dev.mailjet.com/email/guides/send-api-v3/
    V3_1,
}

impl SendAPIVersion {
    /// Retrieve the API URL to be used for the version
    pub fn get_api_url(&self) -> String {
        match self {
            SendAPIVersion::V3 => String::from("https://api.mailjet.com/v3"),
            SendAPIVersion::V3_1 => String::from("https://api.mailjet.com/v3.1"),
        }
    }
}
