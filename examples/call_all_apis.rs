use std::time::Duration;

use chrono::{DateTime, Utc};
use questrade::{auth::ApiToken, Interval};
use reqwest::Proxy;
use tracing::info;

static QT_CONSUMER_KEY: &str = "QT_CONSUMER_KEY";
static QT_REFRESH_TOKEN: &str = "QT_REFRESH_TOKEN";

fn save_new_token(consumer_key: &str, api_token: &ApiToken) {
    std::fs::write(
        ".env",
        format!(
            r#"
export {}={}
export {}={}
    "#,
            QT_CONSUMER_KEY, consumer_key, QT_REFRESH_TOKEN, api_token.refresh_token
        ),
    )
    .unwrap();
}

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
        .proxy(Proxy::http("http://127.0.0.1:8080")?)
        .proxy(Proxy::https("http://127.0.0.1:8080")?)
        .danger_accept_invalid_certs(true)
        .build()
        .unwrap();

    let c = questrade::client::Client::builder()
        .http_client(http_client)
        .consumer_key(consumer_key.clone())
        .build()?;

    let token = c.refresh_token(&refresh_token).await.unwrap();
    info!("got token: {:#?}", token);

    // Optional Save the new refresh token to .env file since the old token is now invalid
    save_new_token(&consumer_key, &token);

    let start = DateTime::parse_from_rfc3339("2021-11-01T05:00:00+00:00")
        .unwrap()
        .with_timezone(&Utc);
    let end = DateTime::parse_from_rfc3339("2021-11-20T05:00:00+00:00")
        .unwrap()
        .with_timezone(&Utc);

    let accounts = c.accounts(&token).await.unwrap();
    info!("got accounts: {:#?}", accounts);

    for account in accounts {
        let activites = c
            .account_activities(&token, &account.number, &start, &end)
            .await?;
        info!("got activies: {:#?}", activites);

        let balances = c.account_balances(&token, &account.number).await?;
        info!("got balances: {:#?}", balances);

        let executions = c
            .account_executions(&token, &account.number, Some(&start), Some(&end))
            .await?;
        info!("got executions: {:#?}", executions);

        let positions = c.account_positions(&token, &account.number).await?;
        info!("got positions: {:#?}", positions);

        let orders = c
            .account_orders(&token, &account.number, Some(&start), Some(&end), None)
            .await?;
        info!("got orders: {:#?}", orders);
    }

    let markets = c.markets(&token).await?;
    info!("got markets: {:#?}", markets);

    let candles = c
        .market_candles(&token, 9292, &start, &end, Interval::OneDay)
        .await?;
    info!("got candles: {:#?}", candles);

    let quotes = c.market_quotes_symbol(&token, 9292).await?;
    info!("got quotes: {:#?}", quotes);
    Ok(())
}
