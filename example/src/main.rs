use dotenv;
use mailjet_rs::common::Recipient;
use mailjet_rs::v3::Message;
use mailjet_rs::{Client, SendAPIVersion};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    dotenv::dotenv().ok().expect("Unable to read .env file");

    let public_key = env::var("PUBLIC_KEY").expect("Unable to find PUBLIC_KEY");
    let private_key = env::var("PRIVATE_KEY").expect("Unable to find PRIVATE_KEY");
    let sender_email = env::var("SENDER_EMAIL").expect("Unable to find SENDER_EMAIL");
    let email = env::var("RECIPIENT_EMAIL").expect("Unable to find RECIPIENT_EMAIL");

    let client = Client::new(
        SendAPIVersion::V3,
        public_key.as_str(),
        private_key.as_str(),
    );

    let to = vec![Recipient::new(email.as_str())];

    let message = Message::new(
        sender_email.as_str(),
        "Rust Venezuela",
        Some(String::from("Come join us and have fun!")),
        "The text part!",
        None,
        to,
    );

    client.send(message).await;
    Ok(())
}
