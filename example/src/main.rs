use mailjet_rs::{Client, Message, Recipient, Messages};
use dotenv;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    dotenv::dotenv().ok().expect("Unable to read .env file");

    let public_key = env::var("PUBLIC_KEY").expect("Unable to find PUBLIC_KEY");
    let private_key = env::var("PRIVATE_KEY").expect("Unable to find PRIVATE_KEY");
    let sender_email = env::var("SENDER_EMAIL").expect("Unable to find SENDER_EMAIL");
    let email = env::var("RECIPIENT_EMAIL").expect("Unable to find RECIPIENT_EMAIL");

    let client = Client::new(public_key.as_str(), private_key.as_str());

    let from = Recipient::new(sender_email, None);
    let to = vec![ Recipient::new(email, None) ];
    
    let message = Message::new(from,
        to, 
        String::from("Hello, World!"), 
        String::from("Hello, World! I'm the body of this email!"), 
        String::from("<h1>heloooo from the othersideee</h1>"));

    let messages_list = Messages::new(message);

    client.send(messages_list).await;
    Ok(())
}
