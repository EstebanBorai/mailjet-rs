use crate::api::common::Payload;
use crate::client::error::Error as MailjetError;
use crate::client::response::Response as MailjetResponse;
use crate::client::status_code::StatusCode as MailjetStatusCode;
use crate::client::version::SendAPIVersion;
use http_auth_basic::Credentials;
use hyper::client::HttpConnector;
use hyper::Client as HyperClient;
use hyper::Error as HyperError;
use hyper::{Body, Request, Response};
use hyper_tls::HttpsConnector;

/// Mailjet's Email API uses the API keys provided by Mailjet for your account [here](https://app.mailjet.com/account/api_keys).
///
/// These are used to create an instance of the `Client` as follows:
///
/// ```ignore
/// let client = Client::new(
///     SendAPIVersion::V3,
///     "public_key",
///     "private_key",
/// );
/// ```
///
pub struct Client {
    pub keys: Credentials,
    pub encoded_credentials: String,
    http_client: HyperClient<HttpsConnector<HttpConnector>>,
    api_base: String,
}

impl Client {
    /// Creates an authenticated Mailjet client by using the provided
    /// `public_key` and `private_key`
    pub fn new(send_api_version: SendAPIVersion, public_key: &str, private_key: &str) -> Self {
        // Creates a basic authentication `Credentials` struct used to authenticate to the
        // Email API.
        //
        // The `user_id` is represented by the `public_key` and the `password` by the `private_key`.
        //
        // Reference: https://dev.mailjet.com/email/guides/
        //

        if public_key == "" || private_key == "" {
            panic!("Invalid `public_key` or `private_key` provided");
        }

        let keys = Credentials::new(public_key, private_key);
        let encoded_credentials = keys.as_http_header();
        let https = HttpsConnector::new();
        let http_client = HyperClient::builder().build::<_, hyper::Body>(https);

        Self {
            api_base: send_api_version.get_api_url(),
            encoded_credentials,
            http_client,
            keys,
        }
    }

    pub async fn send(&self, messages: impl Payload) -> Result<MailjetResponse, MailjetError> {
        let as_json = messages.to_json();

        println!("{}", as_json);

        let response = self.post(Body::from(as_json), "/send").await.unwrap();
        let (parts, body) = response.into_parts();

        if parts.status.is_client_error() || parts.status.is_server_error() {
            let mailjet_error =
                MailjetError::from_api_response(MailjetStatusCode::from(parts.status), body).await;

            return Err(mailjet_error);
        }

        Ok(MailjetResponse::from_api_response(body).await)
    }

    async fn post(&self, body: Body, uri: &str) -> Result<Response<Body>, HyperError> {
        let uri = format!("{}{}", self.api_base, uri);

        let req = Request::builder()
            .method("POST")
            .header("Content-Type", "application/json")
            .header("Authorization", self.encoded_credentials.as_str())
            .uri(uri)
            .body(body)
            .expect("Failed to build POST request");

        self.http_client.request(req).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_creates_a_client_instance_send_api_v3() {
        let have = Client::new(SendAPIVersion::V3, "public_key", "private_key");

        assert_eq!(have.api_base, "https://api.mailjet.com/v3");
        assert_eq!(have.keys.user_id, "public_key");
        assert_eq!(have.keys.password, "private_key");
    }

    #[test]
    fn it_creates_a_client_instance_send_api_v3_1() {
        let have = Client::new(SendAPIVersion::V3_1, "public_key", "private_key");

        assert_eq!(have.api_base, "https://api.mailjet.com/v3.1");
        assert_eq!(have.keys.user_id, "public_key");
        assert_eq!(have.keys.password, "private_key");
    }

    #[test]
    #[should_panic(expected = "Invalid `public_key` or `private_key` provided")]
    fn it_panics_if_invalid_keys_are_provided() {
        Client::new(SendAPIVersion::V3_1, "", "");
    }
}
