use url::Url;

pub const SEND_API_V3_URL: &str = "https://api.mailjet.com/v3/send";
pub const SEND_API_V3_1_URL: &str = "https://api.mailjet.com/v3.1/send";

pub trait ApiClient {
    fn new(private_key: &str, public_key: &str) -> Self;
    fn base_url(&self) -> Url;
    fn custom_base_url(&mut self, url: &str);
    fn private_key(&self) -> String;
    fn public_key(&self) -> String;
}
