use mailjet_rs::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let client = Client::new("your_mailjet_public_key", "your_mailjet_private_key");

    client.send().await;
    Ok(())
}
