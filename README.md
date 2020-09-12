<div>
  <div align="center" style="display: block; text-align: center;">
    <img src="https://raw.githubusercontent.com/EstebanBorai/mailjet-rs/main/assets/mailjet-rs.png" height="120" width="120" />
  </div>
  <h1 align="center">mailjet-rs</h1>
  <h4 align="center">Mailjet API Client for Rust</h4>
</div>

> This still a work in progress

## Installation

```toml
mailjet-rs = "0.0.0"
tokio = { version = "0.2", features = ["full"] }
```

## Usage

In order to get your API keys, you must first sign up to Mailjet for a free plan,
gather your `public` and `private` keys and run the following code.

```rust
use mailjet_rs::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let client = Client::new("your_mailjet_public_key", "your_mailjet_private_key");

    client.send().await;
    Ok(())
}
```

[Example Source Code](https://github.com/EstebanBorai/mailjet-rs/blob/main/example/src/main.png)

## Release

To release a new version you must tag with git and push to the `main` branch.

```bash
git tag -a v0.1.0 -m "First Release"
git push origin main --follow-tags
```
