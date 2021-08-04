mod request;
mod send;

pub mod v3;
pub mod v3_1;

use thiserror::Error;
use url::Url;

pub const PUBLIC_API_KEY_ENV_VAR: &str = "MJ_APIKEY_PUBLIC";
pub const PRIVATE_API_KEY_ENV_VAR: &str = "MJ_APIKEY_PRIVATE";

#[derive(Error, Debug)]
pub enum ClientError {
    #[error(r#"Missing "{0}" environment variables"#)]
    MissingKeyEnvironmentVariables(&'static str),
    #[error("An error ocurred communicating with Mailjet API servers at: {0}. {1}")]
    MailjetServerError(Url, reqwest::Error),
    #[error("Invalid value(s) were provided to the HTTP request. Unable to build a valid HTTP request with the provided values.")]
    InvalidRequest(reqwest::Error),
    #[error("Invalid base URL provided for Mailjet Client. You must provide either an API version or a custom base URL")]
    InvalidBaseUrl(Option<Url>),
    #[error("Missing Private Key for Mailjet API Client")]
    MissingPrivateKey,
    #[error("Missing Public Key for Mailjet API Client")]
    MissingPublicKey,
    #[error("Missing API Version for Send API Client")]
    MissingSendApiVersion,
}

pub type Result<T> = std::result::Result<T, ClientError>;
