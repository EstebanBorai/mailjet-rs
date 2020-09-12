use dotenv;
use mailjet_rs::common::Recipient;
use mailjet_rs::v3::{Message, Attachment};
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

    let mut message = Message::new(
        sender_email.as_str(),
        "Mailjet Rust",
        Some(String::from("May the delivery force be with you!")),
        None,
    );

    let attachment = Attachment::new(
        "image/png",
        "logo.png",
        "iVBORw0KGgoAAAANSUhEUgAAABQAAAALCAYAAAB/Ca1DAAAACXBIWXMAAA7EAAAOxAGVKw4bAAAAB3RJTUUH4wIIChcxurq5eQAAAAd0RVh0QXV0aG9yAKmuzEgAAAAMdEVYdERlc2NyaXB0aW9uABMJISMAAAAKdEVYdENvcHlyaWdodACsD8w6AAAADnRFWHRDcmVhdGlvbiB0aW1lADX3DwkAAAAJdEVYdFNvZnR3YXJlAF1w/zoAAAALdEVYdERpc2NsYWltZXIAt8C0jwAAAAh0RVh0V2FybmluZwDAG+aHAAAAB3RFWHRTb3VyY2UA9f+D6wAAAAh0RVh0Q29tbWVudAD2zJa/AAAABnRFWHRUaXRsZQCo7tInAAABV0lEQVQokaXSPWtTYRTA8d9N7k1zm6a+RG2x+FItgpu66uDQxbFurrr5OQQHR9FZnARB3PwSFqooddAStCBoqmLtS9omx+ESUXuDon94tnP+5+1JYm057GyQjZFP+l+S6G2FzlNe3WHtHc2TNI8zOlUUGLxsD1kDyR+EEQE2P/L8Jm/uk6RUc6oZaYM0JxtnpEX9AGPTtM6w7yzVEb61EaSNn4QD3j5m4QabH6hkVFLSUeqHyCeot0ib6BdNVGscPM/hWWr7S4Tw9TUvbpFUitHTnF6XrS+sL7O6VBSausT0FZonSkb+nZUFFm+z8Z5up5Btr1Lby7E5Zq4yPrMrLR263ZV52g+LvfW3iy6PXubUNVrnhqYNF3bmiZ1i1MmLnL7OxIWh4T+IMpYeRNyrRzyZjWg/ioh+aVgZu4WfXxaixbsRve5fiwb8epTo8+kZjSPFf/sHvgNC0/mbjJbxPAAAAABJRU5ErkJggg==",
    );

    message.attach_inline(attachment);

    let response = client.send(message).await;

    println!("{:?}", response);

    Ok(())
}
