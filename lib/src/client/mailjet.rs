use crate::api::common::Payload;
use crate::client::version::SendAPIVersion;
use http_auth_basic::Credentials;
use hyper::{Request, Response, Body};
use hyper::body::to_bytes as body_to_bytes;
use hyper::client::HttpConnector;
use hyper_tls::HttpsConnector;
use hyper::Client as HyperClient;
use hyper::Error as HyperError;

/// A Mailjet HTTP Client
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

    pub async fn send(&self, messages: impl Payload) {
        let as_json = messages.to_json();
        println!("Sending: {}", as_json);

        let response = self.post(Body::from(as_json), "/send").await.unwrap();
        let bytes = body_to_bytes(response.into_body()).await.unwrap();
        let body = String::from_utf8(bytes.to_vec()).expect("response was not valid utf-8");

        println!("Receiving: {}", body);
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
