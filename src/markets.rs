use chrono::{DateTime, Utc};
use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{auth::ApiToken, errors::QuestradeError, Client, Interval};

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Candle {
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
    pub low: f64,
    pub high: f64,
    pub open: f64,
    pub close: f64,
    pub volume: i64,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Quote {
    pub symbol: String,
    pub symbol_id: i64,
    pub tier: String, // TODO enum
    pub bid_price: f64,
    pub bid_size: i64,
    pub ask_price: f64,
    pub ask_size: i64,
    #[serde(rename = "lastTradePriceTrHrs")]
    pub last_trade_price_trade_hours: f64,
    pub last_trade_price: f64,
    pub last_trade_size: i64,
    pub last_trade_tick: String, // TODO enum
    pub last_trade_time: DateTime<Utc>,
    pub volume: i64,
    pub open_price: f64,
    pub high_price: f64,
    pub low_price: f64,
    pub delay: i64,
    pub is_halted: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Market {
    pub name: String,
    pub trading_venues: Vec<String>,
    pub default_trading_venue: String,       // TODO enum
    pub primary_order_routes: Vec<String>,   // TODO enum
    pub secondary_order_routes: Vec<String>, // TODO enum
    pub level_1_feeds: Vec<String>,          // TODO enum
    pub level_2_feeds: Vec<String>,          // TODO enum
    pub extended_start_time: DateTime<Utc>,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub extended_end_time: Option<DateTime<Utc>>,
    pub snap_quotes_limit: i64,
}

impl Client {
    pub async fn market_candles(
        &self,
        token: &ApiToken,
        symbol_id: i64,
        start: &DateTime<Utc>,
        end: &DateTime<Utc>,
        interval: Interval,
    ) -> Result<Vec<Candle>, QuestradeError> {
        #[derive(Deserialize)]
        pub struct Data {
            pub candles: Vec<Candle>,
        }

        let data: Data = self
            .send(
                self.base_request(
                    Method::GET,
                    token,
                    &format!("v1/markets/candles/{}", symbol_id),
                )
                .query(&[
                    ("startTime", start.to_rfc3339()),
                    ("endTime", end.to_rfc3339()),
                    ("interval", interval.to_string()),
                ]),
            )
            .await?;
        Ok(data.candles)
    }

    pub async fn market_quotes_symbol(
        &self,
        token: &ApiToken,
        symbol_id: i64,
    ) -> Result<Vec<Quote>, QuestradeError> {
        #[derive(Deserialize)]
        pub struct Data {
            pub quotes: Vec<Quote>,
        }

        let data: Data = self
            .send(self.base_request(
                Method::GET,
                token,
                &format!("v1/markets/quotes/{}", symbol_id),
            ))
            .await?;
        Ok(data.quotes)
    }

    pub async fn market_quotes_symbols(
        &self,
        token: &ApiToken,
        symbol_ids: Vec<i64>,
    ) -> Result<Vec<Quote>, QuestradeError> {
        #[derive(Deserialize)]
        pub struct Data {
            pub quotes: Vec<Quote>,
        }

        let data: Data = self
            .send(
                self.base_request(Method::GET, token, &format!("v1/markets/quotes"))
                    .query(&[("ids", symbol_ids)]),
            )
            .await?;
        Ok(data.quotes)
    }

    pub async fn markets(&self, token: &ApiToken) -> Result<Vec<Market>, QuestradeError> {
        #[derive(Deserialize)]
        pub struct Data {
            pub markets: Vec<Market>,
        }

        let data: Data = self
            .send(self.base_request(Method::GET, token, &format!("v1/markets")))
            .await?;
        Ok(data.markets)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn candles_deserialize_works() {
        #[derive(Deserialize)]
        struct Data {
            candles: Vec<Candle>,
        }
        let data = r#"
        {
            "candles": [
                {
                    "start": "2014-01-02T00:00:00.000000-05:00",
                    "end": "2014-01-03T00:00:00.000000-05:00",
                    "low": 70.3,
                    "high": 70.78,
                    "open": 70.68,
                    "close": 70.73,
                    "volume": 983609
                }
             ]
        }
        "#;
        let expected = &Candle {
            start: DateTime::parse_from_rfc3339("2014-01-02T00:00:00.000000-05:00")
                .unwrap()
                .with_timezone(&Utc),
            end: DateTime::parse_from_rfc3339("2014-01-03T00:00:00.000000-05:00")
                .unwrap()
                .with_timezone(&Utc),
            low: 70.3,
            high: 70.78,
            open: 70.68,
            close: 70.73,
            volume: 983609,
        };
        let d: Data = serde_json::from_str(data).expect("failed to deserialize JSON");
        let account = d.candles.get(0).unwrap();
        assert_eq!(expected, account)
    }

    #[test]
    fn markets_deserialize_works() {
        #[derive(Deserialize)]
        struct Data {
            markets: Vec<Market>,
        }
        let data = r#"
        {
            "markets": [{
                "name": "TSX",
                "tradingVenues": [
                    "TSX",
                    "ALPH",
                    "CHIC",
                    "OMGA",
                    "PURE"
                ],
                "defaultTradingVenue": "AUTO",
                "primaryOrderRoutes": [
                    "AUTO"
                ],            
                "level2Feeds": [
                    "PINX"
                ],
                "secondaryOrderRoutes": [
                    "TSX",
                    "AUTO" 
                ],
                "level1Feeds": [
                    "ALPH",
                    "CHIC",
                    "OMGA",
                    "PURE",
                    "TSX"
                ],
                "extendedStartTime": "2014-10-06T07:00:00.000000-04:00",
                "startTime": "2014-10-06T09:30:00.000000-04:00",
                "endTime": "2014-10-06T09:30:00.000000-04:00",
                "snapQuotesLimit": 99999
            }]
         }
        "#;
        let expected = &Market {
            name: String::from("TSX"),
            trading_venues: vec![
                String::from("TSX"),
                String::from("ALPH"),
                String::from("CHIC"),
                String::from("OMGA"),
                String::from("PURE"),
            ],
            default_trading_venue: String::from("AUTO"),
            primary_order_routes: vec![String::from("AUTO")],
            secondary_order_routes: vec![String::from("TSX"), String::from("AUTO")],
            level_1_feeds: vec![
                String::from("ALPH"),
                String::from("CHIC"),
                String::from("OMGA"),
                String::from("PURE"),
                String::from("TSX"),
            ],
            level_2_feeds: vec![String::from("PINX")],
            extended_start_time: DateTime::parse_from_rfc3339("2014-10-06T07:00:00.000000-04:00")
                .unwrap()
                .with_timezone(&Utc),
            start_time: DateTime::parse_from_rfc3339("2014-10-06T09:30:00.000000-04:00")
                .unwrap()
                .with_timezone(&Utc),
            end_time: DateTime::parse_from_rfc3339("2014-10-06T09:30:00.000000-04:00")
                .unwrap()
                .with_timezone(&Utc),
            extended_end_time: None,
            snap_quotes_limit: 99999,
        };
        let d: Data = serde_json::from_str(data).expect("failed to deserialize JSON");
        let account = d.markets.get(0).unwrap();
        assert_eq!(expected, account)
    }

    #[test]
    fn quotes_deserialize_works() {
        #[derive(Deserialize)]
        struct Data {
            quotes: Vec<Quote>,
        }
        let data = r#"
        {
            "quotes": [
                {
                    "symbol": "THI.TO",
                    "symbolId": 38738,
                    "tier": " ", 
                    "bidPrice": 83.65, 
                    "bidSize":6500, 
                    "askPrice":  83.67,
                    "askSize": 9100,
                    "lastTradePriceTrHrs": 83.66, 
                    "lastTradePrice": 83.66,
                    "lastTradeSize": 3100,
                    "lastTradeTick": "Equal",
                    "lastTradeTime": "2014-10-24T20:06:40.131000-04:00",
                    "volume": 80483500, 
                    "openPrice":  83.66, 
                    "highPrice": 83.86, 
                    "lowPrice": 83.66,
                    "delay": 0, 
                    "isHalted": false
                }
              ]
          }
        "#;
        let expected = &Quote {
            symbol: String::from("THI.TO"),
            symbol_id: 38738,
            tier: String::from(" "),
            bid_price: 83.65,
            bid_size: 6500,
            ask_price: 83.67,
            ask_size: 9100,
            last_trade_price_trade_hours: 83.66,
            last_trade_price: 83.66,
            last_trade_size: 3100,
            last_trade_tick: String::from("Equal"),
            last_trade_time: DateTime::parse_from_rfc3339("2014-10-24T20:06:40.131000-04:00")
                .unwrap()
                .with_timezone(&Utc),
            volume: 80483500,
            open_price: 83.66,
            high_price: 83.86,
            low_price: 83.66,
            delay: 0,
            is_halted: false,
        };
        let d: Data = serde_json::from_str(data).expect("failed to deserialize JSON");
        let account = d.quotes.get(0).unwrap();
        assert_eq!(expected, account)
    }
}
