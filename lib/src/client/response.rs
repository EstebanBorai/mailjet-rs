use hyper::body::to_bytes;
use hyper::Body;
use serde::{Deserialize, Serialize};
use serde_json::from_str;

/// Details from the message sent returned by
/// Mailjet when a request is successful
#[derive(Debug, Serialize, Deserialize)]
pub struct Sent {
    #[serde(rename = "Email")]
    pub email: String,
    #[serde(rename = "MessageID")]
    pub message_id: usize,
    #[serde(rename = "MessageUUID")]
    pub message_uuid: String,
}

/// Response from Mailjet when consuming the Send API
///
/// `Response` struct represents Mailjet's Send API V3 response
/// for the `send` enpoint.
///
/// ```json
///  {
///    "Sent": [
///      {
///        "Email": "passenger@mailjet.com",
///        "MessageID": 111111111111111
///      }
///    ]
///  }
/// ```
#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    #[serde(rename = "Sent")]
    pub sent: Vec<Sent>,
}

impl Response {
    /// Creates an `Error` instance from the API response
    pub async fn from_api_response(body: Body) -> Self {
        let bytes = to_bytes(body).await.unwrap();
        let response = String::from_utf8(bytes.to_vec()).expect("response was not valid utf-8");
        let response: Response =
            from_str(response.as_str()).expect("invalid response from mailjet api");

        response
    }
}
