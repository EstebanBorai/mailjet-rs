use reqwest::header::{self, HeaderMap, HeaderValue};
use reqwest::{Client, Request, Response};
use serde::{Deserialize, Serialize};
use url::Url;

use super::{ClientError, Result};

#[derive(Debug)]
pub struct HttpClient {
    pub(crate) base_url: Url,
    pub(crate) http_client: Client,
    pub(crate) private_key: String,
    pub(crate) public_key: String,
}

impl HttpClient {
    pub fn new<T: ToString>(base_url: Url, private_key: T, public_key: T) -> Self {
        let http_client = Client::new();

        HttpClient {
            base_url,
            http_client,
            private_key: private_key.to_string(),
            public_key: public_key.to_string(),
        }
    }

    pub async fn post<T>(&self, path: &str, body: &Option<T>) -> Result<Response>
    where
        T: Serialize,
    {
        let url = self.base_url.join(path).unwrap();
        let mut headers = HeaderMap::new();

        headers.append(
            header::CONTENT_TYPE,
            HeaderValue::from_static("application/json"),
        );

        let request = self
            .http_client
            .post(url.clone())
            .json(body)
            .basic_auth(self.public_key.clone(), Some(self.private_key.clone()))
            .headers(headers)
            .build()
            .map_err(|err| ClientError::InvalidRequest(err))?;

        self.http_client
            .execute(request)
            .await
            .map_err(|err| ClientError::MailjetServerError(url, err))
    }
}
