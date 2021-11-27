# questrade-rs 

[![Rust](https://github.com/mchestr/questrade-rs/actions/workflows/rust.yml/badge.svg)](https://github.com/mchestr/questrade-rs/actions/workflows/rust.yml)
![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)

Async Questrade client for Rust, currently a work in progress

## Usage

See [examples](./examples) for a variety of different examples.

```rust
use std::time::Duration;

use tracing::info;

static QT_CONSUMER_KEY: &str = "QT_CONSUMER_KEY";
static QT_REFRESH_TOKEN: &str = "QT_REFRESH_TOKEN";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    let filter = std::env::var("RUST_LOG").unwrap_or_else(|_| "debug".to_owned());
    tracing_subscriber::fmt().with_env_filter(filter).init();

    let consumer_key = std::env::var(QT_CONSUMER_KEY)
        .expect(&format!("{} env variable must be set", QT_CONSUMER_KEY));
    let refresh_token = std::env::var(QT_REFRESH_TOKEN)
        .expect(&format!("{} env variable must be set", QT_REFRESH_TOKEN));

    let http_client = reqwest::Client::builder()
        .timeout(Duration::from_secs(30))
        .connect_timeout(Duration::from_secs(10))
        .pool_idle_timeout(Duration::from_secs(300))
        .build()
        .unwrap();

    let c = questrade::client::Client::builder()
        .http_client(http_client)
        .consumer_key(consumer_key)
        .build()?;

    let token = c.refresh_token(&refresh_token).await.unwrap();
    info!("got token: {:#?}", token);

    let accounts = c.accounts(&token).await.unwrap();
    info!("got accounts: {:#?}", accounts);
    Ok(())
}
```