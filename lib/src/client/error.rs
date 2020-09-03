use crate::client::StatusCode;
use hyper::body::to_bytes;
use hyper::Body;

#[derive(Debug)]
pub struct Error {
    pub status_code: StatusCode,
    pub message: String,
}

impl Error {
    /// Creates an `Error` instance from the API response
    pub async fn from_api_response(status_code: StatusCode, body: Body) -> Self {
        let bytes = to_bytes(body).await.unwrap();
        let body = String::from_utf8(bytes.to_vec()).expect("response was not valid utf-8");

        Self {
            status_code,
            message: body,
        }
    }
}
