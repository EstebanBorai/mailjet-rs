use mailjet_rs::common::Recipient;
use mailjet_rs::v3::{Attachment, Message};
use mailjet_rs::{Client, SendAPIVersion};
use mailjet_rs::{Map, Value};
use std::collections::HashMap;

/// Base64 representation of the Mailjet logo found in the Mailjet SendAPI V3 docs
const MAILJET_LOGO_BASE64: &str = "iVBORw0KGgoAAAANSUhEUgAAABQAAAALCAYAAAB/Ca1DAAAACXBIWXMAAA7EAAAOxAGVKw4bAAAAB3RJTUUH4wIIChcxurq5eQAAAAd0RVh0QXV0aG9yAKmuzEgAAAAMdEVYdERlc2NyaXB0aW9uABMJISMAAAAKdEVYdENvcHlyaWdodACsD8w6AAAADnRFWHRDcmVhdGlvbiB0aW1lADX3DwkAAAAJdEVYdFNvZnR3YXJlAF1w/zoAAAALdEVYdERpc2NsYWltZXIAt8C0jwAAAAh0RVh0V2FybmluZwDAG+aHAAAAB3RFWHRTb3VyY2UA9f+D6wAAAAh0RVh0Q29tbWVudAD2zJa/AAAABnRFWHRUaXRsZQCo7tInAAABV0lEQVQokaXSPWtTYRTA8d9N7k1zm6a+RG2x+FItgpu66uDQxbFurrr5OQQHR9FZnARB3PwSFqooddAStCBoqmLtS9omx+ESUXuDon94tnP+5+1JYm057GyQjZFP+l+S6G2FzlNe3WHtHc2TNI8zOlUUGLxsD1kDyR+EEQE2P/L8Jm/uk6RUc6oZaYM0JxtnpEX9AGPTtM6w7yzVEb61EaSNn4QD3j5m4QabH6hkVFLSUeqHyCeot0ib6BdNVGscPM/hWWr7S4Tw9TUvbpFUitHTnF6XrS+sL7O6VBSausT0FZonSkb+nZUFFm+z8Z5up5Btr1Lby7E5Zq4yPrMrLR263ZV52g+LvfW3iy6PXubUNVrnhqYNF3bmiZ1i1MmLnL7OxIWh4T+IMpYeRNyrRzyZjWg/ioh+aVgZu4WfXxaixbsRve5fiwb8epTo8+kZjSPFf/sHvgNC0/mbjJbxPAAAAABJRU5ErkJggg==";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Create an instance of the Mailjet API client
    // used to send the `Message` and also define your API
    // credentials
    let client = Client::new(SendAPIVersion::V3, "public_key", "private_key");

    // Create your a `Message` instance with the minimum required values
    let mut message = Message::new(
        "mailjet_sender@company.com",
        "Mailjet Rust",
        Some("Your email flight plan!".to_string()),
        Some("Dear passenger, welcome to Mailjet! May the delivery force be with you!".to_string()),
    );

    message.push_recipient(Recipient::new("receiver@company.com"));

    // Set some HTML for your email
    //
    // Note that here we are using `cid:logo.png` as the src value for our image
    // this is using the `inline_attachment` with `filename` "logo.png" as the
    // image source
    message.html_part = Some("<h3>Dear [[var:name]] [[var:last]], welcome to <img src=\"cid:logo.png\"> <a href=\"https://www.mailjet.com/\">Mailjet</a>!<br />May the delivery force be with you!".to_string());

    // Attach inline files providing its base64 representation
    // content-type and a name.
    // The name of the file can be used to reference this file in your HTML content
    let mailjet_logo_inline = Attachment::new("image/png", "logo.png", MAILJET_LOGO_BASE64);

    // Attach the `Attachment` as an Inline Attachment
    // this function can also be used to attach common Attachments
    message.attach_inline(mailjet_logo_inline);

    // Creates a txt file Attachment
    let txt_file_attachment = Attachment::new(
        "text/plain",
        "test.txt",
        "VGhpcyBpcyB5b3VyIGF0dGFjaGVkIGZpbGUhISEK",
    );

    // Attaches the TXT file as an email Attachment
    message.attach(txt_file_attachment);

    // Provide variables for your template
    // `Map` and `Value` are reexported from
    // `serde_json`
    let mut vars = Map::new();

    vars.insert(String::from("name"), Value::from("Foo"));
    vars.insert(String::from("last"), Value::from("Bar"));

    message.vars = Some(vars);

    // Set the headers to a custom Reply-To address
    message.set_headers(HashMap::from([(
        "Reply-To".to_string(),
        "copilot@mailjet.com".to_string(),
    )]));

    // Finally send the message using the `Client`
    let response = client.send(message).await;

    // Do something with the response from Mailjet
    // Ok(Response { sent: [Sent { email: "your_receiver@company.com", message_id: 000, message_uuid: "message-uuid" }] })
    println!("{:?}", response);

    Ok(())
}
