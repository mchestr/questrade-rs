# questrade-rs 

[![Rust](https://github.com/mchestr/questrade-rs/actions/workflows/rust.yml/badge.svg)](https://github.com/mchestr/questrade-rs/actions/workflows/rust.yml)
![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)

Async Questrade client for Rust, currently a work in progress

## Usage

Create a consumer key on [Questrade](https://apphub.questrade.com/UI/UserApps.aspx). Update the code below, or export the consumer key as `QT_CONSUMER_KEY`.

Create a new device, and generate a new token, export it as `QT_REFRESH_TOKEN`.

Run the following:

```rust
use std::time::Duration;
use questrade::client::Environment;

static QT_CONSUMER_KEY: &str = "`QT_CONSUMER_KEY`";
static QT_REFRESH_TOKEN: &str = "QT_REFRESH_TOKEN";

#[tokio::main]
async fn main() {

    let http_client = reqwest::Client::builder()
        .timeout(Duration::from_secs(30))
        .connect_timeout(Duration::from_secs(10))
        .pool_idle_timeout(Duration::from_secs(300))
        .build()
        .unwrap();

    let consumer_key = std::env::var(QT_CONSUMER_KEY)
        .expect(&format!("{} env variable must be set", QT_CONSUMER_KEY));
    let refresh_token = std::env::var(QT_REFRESH_TOKEN)
        .expect(&format!("{} env variable must be set", QT_REFRESH_TOKEN));

    let c = questrade::client::Client::new(http_client, &consumer_key, Environment::Production).unwrap();
    let token = c.refresh_token(&refresh_token).await.unwrap();
    let accounts = c.accounts(&token).await.unwrap();
    println!("{:#?}", accounts);
}
```