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
        "Mailjet Rust",
        Some(String::from("Testing Mailjet Rust with Send API v3 Message")),
        "This is a test on mailjet-rs with Send API v3 sending a basic email",
        Some(String::from("<h1>Some HTML to give it a try</h1>")),
        to,
    );

    let response = client.send(message).await;

    println!("{:?}", response);

    Ok(())
}
