use url::Url;

use super::request::HttpClient;
use super::send::{ApiClient, SEND_API_V3_URL};

pub struct Client {
    base_url: Url,
    http_client: HttpClient,
    public_key: String,
    private_key: String,
}

impl ApiClient for Client {
    fn new(private_key: &str, public_key: &str) -> Self {
        let base_url: Url = SEND_API_V3_URL.parse().unwrap();
        let http_client = HttpClient::new(base_url.clone(), private_key, public_key);

        Self {
            base_url,
            http_client,
            private_key: private_key.to_string(),
            public_key: public_key.to_string(),
        }
    }

    fn base_url(&self) -> Url {
        self.base_url.clone()
    }

    fn custom_base_url(&mut self, url: &str) {
        self.base_url = url.parse::<Url>().unwrap();
    }

    fn private_key(&self) -> String {
        self.private_key.clone()
    }

    fn public_key(&self) -> String {
        self.public_key.clone()
    }
}
