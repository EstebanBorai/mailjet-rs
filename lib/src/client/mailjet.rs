use http_auth_basic::Credentials;
use hyper::{Request, Response, Body};
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
    pub fn new(public_key: &str, private_key: &str) -> Self {
        // Creates a basic authentication `Credentials` struct used to authenticate to the
        // Email API.
        //
        // The `user_id` is represented by the `public_key` and the `password` by the `private_key`.
        //
        // Reference: https://dev.mailjet.com/email/guides/
        //
        let keys = Credentials::new(public_key, private_key);
        let encoded_credentials = keys.as_http_header();
        let https = HttpsConnector::new();
        let http_client = HyperClient::builder().build::<_, hyper::Body>(https);
        let api_base = String::from("https://api.mailjet.com/v3.1");

        Self {
            api_base,
            encoded_credentials,
            http_client,
            keys,
        }
    }

    pub async fn send(&self) {
        let body = r#"{
            "Messages":[
                    {
                            "From": {
                                    "Email": "sender!",
                                    "Name": "Me"
                            },
                            "To": [
                                    {
                                            "Email": "estebanborai@gmail.com",
                                            "Name": "You"
                                    }
                            ],
                            "Subject": "My first Mailjet Email!",
                            "TextPart": "Greetings from Mailjet!",
                            "HTMLPart": "<h3>Dear passenger 1, welcome to <a href=\"https://www.mailjet.com/\">Mailjet</a>!</h3><br />May the delivery force be with you!"
                    }
            ]
        }"#;

        let response = self.post(Body::from(body), "/send").await;

        if let Ok(res) = response {
            // handle response
        } else {
            eprintln!("Something came wrong {}", response.unwrap().status());
        }
    }

    async fn post(&self, body: Body, uri: &str) -> Result<Response<Body>, HyperError> {
        let uri = format!("{}{}", self.api_base, uri);
        println!("{}", uri);

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
