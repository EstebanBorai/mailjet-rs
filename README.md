<div>
  <div align="center" style="display: block; text-align: center;">
    <img src="https://raw.githubusercontent.com/EstebanBorai/mailjet-rs/main/assets/mailjet-rs.png" height="120" width="120" />
  </div>
  <h1 align="center">mailjet-rs</h1>
  <h4 align="center">Mailjet API wrapper for Rust</h4>
</div>

<div align="center">

  ![MIT License](https://img.shields.io/badge/license-MIT-007EC7.svg?style=flat-square)
  [![Crates.io](https://img.shields.io/crates/v/mailjet-rs.svg)](https://crates.io/crates/mailjet-rs)
  [![Documentation](https://docs.rs/mailjet-rs/badge.svg)](https://docs.rs/mailjet-rs)

</div>

## Overview

This crate contains an unofficial wrapper for the Mailjet API.

The official resources are available in the [Mailjet Developer Guides](https://dev.mailjet.com/email/guides/) webiste.

The official [Mailjet Organization in GitHub](https://github.com/mailjet) provides wrappers
for other programming languages such as Go, PHP, JavaScript (NodeJS), Ruby and more.

## Contents

- [Installation](#installation)
- [Client](#client)
  - [Authentication](#authentication)
  - [API Version](#api-version)
- [Send Messages](#send-messages)
  - [Examples Requirements](#examples-requirements)
  - [Consuming the API Wrapper](#consuming-the-api-wrapper)
  - [Basic Message](#basic-message)
  - [Send to multiple recipients](#send-to-multiple-recipients)
  - [Using `To`, `Cc` and `Bcc` instead of `Recipients`](#using-to-cc-and-bcc-instead-of-recipients)
  - [Send Inline Attachments](#send-inline-attachments)
  - [Full Featured Example on v3](#full-featured-example-on-v3)

### Installation

```toml
mailjet-rs = "0.2.0"

# Used by `Hyper` which is the HTTP request solution behind the Client
tokio = { version = "1", features = ["full"] }
```

### Client

The `Client` struct performs API related tasks such as handling authentication and defning the API version
that must be used for every request.

### Authentication

Mailjet's Email API uses the API keys provided by Mailjet for your account [here](https://app.mailjet.com/account/api_keys).

These are used to create an instance of the `Client` as follows:

```rust
let client = Client::new(
    SendAPIVersion::V3,
    "public_key",
    "private_key",
);
```

### API Version

Mailjet's API has 3 versions available at the moment, the following table describes each version and its
support in this crate

Version | Name | Supported
--- | --- | ---
v3 | The Email API | ✔️
v3.1 | Email Send API v3.1 (Latest Version) | ❌ (Early Development)
v4 | SMS API | ❌ (TBD)

> As you can see at the moment this crate supports only the version 3 of the Email API. Support for the version 3.1 is in early development

The version of the API to use is provided to the `Client` using the `SendAPIVersion` enum.

### Send Messages

To send a `Message` you must create a `Client` instance, then define `Recipients` and finally build your `Message`.

The `Client`'s method `send` receives a `Payload` trait implementator, this trait is implemented by `Message` and every struct which is
sent to the Mailjet's API throught the `Client`.

A call to `send` will return a `Future` which wraps a `Result<MailjetResponse, MailjetError>`.

### Examples Requirements

To run any of the following examples you must have a public and private key for Mailjet (a free plan is available to consume the API)
and also install the tokio runtime.

```toml
mailjet-rs = "0.2.0"

# Used by `Hyper` which is the HTTP request solution behind the Client
tokio = { version = "1", features = ["full"] }
```

### Consuming the API Wrapper

Theres two ways to consume this wrapper, either by using the methods provided by the `Message` struct or not to using these
methods.

All of the fields of the `Message` struct are public, this is because sometimes its a bit of verbose/tedious to build
a simple struct by calling multiple methods.

The methods provided are meant to validate the fields of the `Message` struct with the API specificiations but you are free
to provide values to these fields.

### Basic Message

**mailjet-rs** makes use of the Tokio runtime to perform asynchronous operations,
Hyper is being used undet the hood to perform HTTP requests to the Mailjet API.

Here a `Message` is created and sent to the `Recipient` defined.
This message neither contains HTML or is consuming a template, instead this `Message` contains
raw text.

```rust
use mailjet_rs::common::Recipient;
use mailjet_rs::v3::Message;
use mailjet_rs::{Client, SendAPIVersion};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Create an instance of the Mailjet API client
    // used to send the `Message` and also define your API
    // credentials
    let client = Client::new(
        SendAPIVersion::V3,
        "public_key",
        "private_key",
    );

    // Create your a `Message` instance with the minimum required values
    let mut message = Message::new(
        "mailjet_sender@company.com",
        "Mailjet Rust",
        Some("Your email flight plan!".to_string()),
        Some("Dear passenger, welcome to Mailjet! May the delivery force be with you!".to_string())
    );

    message.push_recipient(Recipient::new("receiver@company.com"));

    // Finally send the message using the `Client`
    let response = client.send(message).await;

    // Do something with the response from Mailjet
    // Ok(Response { sent: [Sent { email: "your_receiver@company.com", message_id: 000, message_uuid: "message-uuid" }] })
    println!("{:?}", response);

    Ok(())
}
```

### Send to multiple recipients

```rust
use mailjet_rs::common::Recipient;
use mailjet_rs::v3::Message;
use mailjet_rs::{Client, SendAPIVersion};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let client = Client::new(
        SendAPIVersion::V3,
        "public_key",
        "private_key",
    );

    let mut message = Message::new(
        "mailjet_sender@company.com",
        "Mailjet Rust",
        Some("Your email flight plan!".to_string()),
        Some("Dear passenger, welcome to Mailjet! May the delivery force be with you!".to_string())
    );

    let recipients = vec![
        Recipient::new("receiver1@company.com"),
        Recipient::new("receiver2@company.com"),
        Recipient::new("receiver3@company.com"),
    ];

    message.push_many_recipients(recipients);

    let response = client.send(message).await;

    println!("{:?}", response);

    Ok(())
}
```

### Using `To`, `Cc` and `Bcc` instead of `Recipients`

> Note: If a recipient does not exist in any of your contact list it will be created from scratch, keep that in mind if you are planning on sending a welcome email and then you're trying to add the email to a list as the contact effectively exists already. [Mailjet's API Documentation](https://dev.mailjet.com/email/guides/send-api-V3/#send-a-basic-email)

```rust
use mailjet_rs::common::Recipient;
use mailjet_rs::v3::Message;
use mailjet_rs::{Client, SendAPIVersion};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let client = Client::new(
        SendAPIVersion::V3,
        "public_key",
        "private_key",
    );

    let mut message = Message::new(
        "mailjet_sender@company.com",
        "Mailjet Rust",
        Some("Your email flight plan!".to_string()),
        Some("Dear passenger, welcome to Mailjet! May the delivery force be with you!".to_string())
    );

    message.set_receivers(
        vec![
            Recipient::new("bar@foo.com"),
        ],
        Some(vec![
            Recipient::new("bee@foo.com"),
        ]),
        None
    );

    let response = client.send(message).await;

    println!("{:?}", response);

    Ok(())
}
```

### Send Inline Attachments

```rust
use mailjet_rs::common::Recipient;
use mailjet_rs::v3::{Message, Attachment};
use mailjet_rs::{Client, SendAPIVersion};

/// Base64 representation of the Mailjet logo found in the Mailjet SendAPI V3 docs
const MAILJET_LOGO_BASE64: &str = "iVBORw0KGgoAAAANSUhEUgAAABQAAAALCAYAAAB/Ca1DAAAACXBIWXMAAA7EAAAOxAGVKw4bAAAAB3RJTUUH4wIIChcxurq5eQAAAAd0RVh0QXV0aG9yAKmuzEgAAAAMdEVYdERlc2NyaXB0aW9uABMJISMAAAAKdEVYdENvcHlyaWdodACsD8w6AAAADnRFWHRDcmVhdGlvbiB0aW1lADX3DwkAAAAJdEVYdFNvZnR3YXJlAF1w/zoAAAALdEVYdERpc2NsYWltZXIAt8C0jwAAAAh0RVh0V2FybmluZwDAG+aHAAAAB3RFWHRTb3VyY2UA9f+D6wAAAAh0RVh0Q29tbWVudAD2zJa/AAAABnRFWHRUaXRsZQCo7tInAAABV0lEQVQokaXSPWtTYRTA8d9N7k1zm6a+RG2x+FItgpu66uDQxbFurrr5OQQHR9FZnARB3PwSFqooddAStCBoqmLtS9omx+ESUXuDon94tnP+5+1JYm057GyQjZFP+l+S6G2FzlNe3WHtHc2TNI8zOlUUGLxsD1kDyR+EEQE2P/L8Jm/uk6RUc6oZaYM0JxtnpEX9AGPTtM6w7yzVEb61EaSNn4QD3j5m4QabH6hkVFLSUeqHyCeot0ib6BdNVGscPM/hWWr7S4Tw9TUvbpFUitHTnF6XrS+sL7O6VBSausT0FZonSkb+nZUFFm+z8Z5up5Btr1Lby7E5Zq4yPrMrLR263ZV52g+LvfW3iy6PXubUNVrnhqYNF3bmiZ1i1MmLnL7OxIWh4T+IMpYeRNyrRzyZjWg/ioh+aVgZu4WfXxaixbsRve5fiwb8epTo8+kZjSPFf/sHvgNC0/mbjJbxPAAAAABJRU5ErkJggg==";


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let client = Client::new(
        SendAPIVersion::V3,
        "public_key",
        "private_key",
    );

    let mut message = Message::new(
        "mailjet_sender@company.com",
        "Mailjet Rust",
        Some("Your email flight plan!".to_string()),
        Some("Dear passenger, welcome to Mailjet! May the delivery force be with you!".to_string())
    );

    message.set_receivers(
        vec![
            Recipient::new("bar@foo.com"),
        ],
        Some(vec![
            Recipient::new("bee@foo.com"),
        ]),
        None
    );

    let mailjet_logo = Attachment::new(
        "image/png", 
        "logo.png", 
        MAILJET_LOGO_BASE64);

    message.attach_inline(mailjet_logo);

    message.html_part = Some("<h3>Dear [[var:name]] [[var:last]], welcome to <img src=\"cid:logo.png\"> <a href=\"https://www.mailjet.com/\">Mailjet</a>!<br />May the delivery force be with you!".to_string());


    let response = client.send(message).await;

    println!("{:?}", response);

    Ok(())
}
```

### Full Featured Example on v3

The following is an example using the Mailjet's Send API v3 where the following
features are covered:

- Attach inline images
- Attach files
- Use template variables

```rust
use mailjet_rs::common::Recipient;
use mailjet_rs::v3::{Message, Attachment};
use mailjet_rs::{Client, SendAPIVersion};
use mailjet_rs::{Map, Value};

/// Base64 representation of the Mailjet logo found in the Mailjet SendAPI V3 docs
const MAILJET_LOGO_BASE64: &str = "iVBORw0KGgoAAAANSUhEUgAAABQAAAALCAYAAAB/Ca1DAAAACXBIWXMAAA7EAAAOxAGVKw4bAAAAB3RJTUUH4wIIChcxurq5eQAAAAd0RVh0QXV0aG9yAKmuzEgAAAAMdEVYdERlc2NyaXB0aW9uABMJISMAAAAKdEVYdENvcHlyaWdodACsD8w6AAAADnRFWHRDcmVhdGlvbiB0aW1lADX3DwkAAAAJdEVYdFNvZnR3YXJlAF1w/zoAAAALdEVYdERpc2NsYWltZXIAt8C0jwAAAAh0RVh0V2FybmluZwDAG+aHAAAAB3RFWHRTb3VyY2UA9f+D6wAAAAh0RVh0Q29tbWVudAD2zJa/AAAABnRFWHRUaXRsZQCo7tInAAABV0lEQVQokaXSPWtTYRTA8d9N7k1zm6a+RG2x+FItgpu66uDQxbFurrr5OQQHR9FZnARB3PwSFqooddAStCBoqmLtS9omx+ESUXuDon94tnP+5+1JYm057GyQjZFP+l+S6G2FzlNe3WHtHc2TNI8zOlUUGLxsD1kDyR+EEQE2P/L8Jm/uk6RUc6oZaYM0JxtnpEX9AGPTtM6w7yzVEb61EaSNn4QD3j5m4QabH6hkVFLSUeqHyCeot0ib6BdNVGscPM/hWWr7S4Tw9TUvbpFUitHTnF6XrS+sL7O6VBSausT0FZonSkb+nZUFFm+z8Z5up5Btr1Lby7E5Zq4yPrMrLR263ZV52g+LvfW3iy6PXubUNVrnhqYNF3bmiZ1i1MmLnL7OxIWh4T+IMpYeRNyrRzyZjWg/ioh+aVgZu4WfXxaixbsRve5fiwb8epTo8+kZjSPFf/sHvgNC0/mbjJbxPAAAAABJRU5ErkJggg==";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {

    // Create an instance of the Mailjet API client
    // used to send the `Message` and also define your API
    // credentials
    let client = Client::new(
        SendAPIVersion::V3,
        "public_key",
        "private_key",
    );

    // Create your a `Message` instance with the minimum required values
    let mut message = Message::new(
        "mailjet_sender@company.com",
        "Mailjet Rust",
        Some("Your email flight plan!".to_string()),
        Some("Dear passenger, welcome to Mailjet! May the delivery force be with you!".to_string())
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
    let mailjet_logo_inline = Attachment::new(
      "image/png", 
      "logo.png", 
      MAILJET_LOGO_BASE64);

    // Attach the `Attachment` as an Inline Attachment
    // this function can also be used to attach common Attachments
    message.attach_inline(mailjet_logo_inline);

    // Creates a txt file Attachment
    let txt_file_attachment = Attachment::new(
      "text/plain", 
      "test.txt", 
      "VGhpcyBpcyB5b3VyIGF0dGFjaGVkIGZpbGUhISEK");

    // Attaches the TXT file as an email Attachment
    message.attach(txt_file_attachment);

    // Provide variables for your template
    // `Map` and `Value` are reexported from
    // `serde_json`
    let mut vars = Map::new();

    vars.insert(String::from("name"), Value::from("Foo"));
    vars.insert(String::from("last"), Value::from("Bar"));

    message.vars = Some(vars);

    // Finally send the message using the `Client`
    let response = client.send(message).await;

    // Do something with the response from Mailjet
    // Ok(Response { sent: [Sent { email: "your_receiver@company.com", message_id: 000, message_uuid: "message-uuid" }] })
    println!("{:?}", response);

    Ok(())
}
```

## Release

To release a new version you must tag with git and push to the `main` branch.

```bash
git tag -a v0.1.0 -m "First Release"
git push origin main --follow-tags
```

## Contribute

Feel free to contribute!

## License

This project is licensed under the MIT License to match the same licensing as Mailjet's official wrappers
