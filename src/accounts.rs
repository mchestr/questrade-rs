use chrono::{DateTime, Utc};
use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{
    auth::ApiToken, client::Client, errors::QuestradeError, AccountStatus, AccountType,
    ActivityType, ClientAccountType, Currency,
};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Accounts {
    pub accounts: Vec<Account>,
    pub client_id: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Account {
    #[serde(rename = "type")]
    pub type_: AccountType,
    pub number: String,
    pub status: AccountStatus,
    pub is_primary: bool,
    pub is_billing: bool,
    pub client_account_type: ClientAccountType,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Activities {
    pub activities: Vec<Activity>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Activity {
    pub trade_date: DateTime<Utc>,
    pub transaction_date: DateTime<Utc>,
    pub settlement_date: DateTime<Utc>,
    pub action: String,
    pub symbol: String,
    pub symbol_id: u64,
    pub description: String,
    pub currency: Currency,
    pub quantity: f64,
    pub price: f64,
    pub gross_amount: f64,
    pub commission: f64,
    pub net_amount: f64,
    #[serde(rename = "type")]
    pub type_: ActivityType,
}

impl Client {
    pub async fn accounts(&self, token: &ApiToken) -> Result<Accounts, QuestradeError> {
        self.send(self.base_request(Method::GET, token, "v1/accounts"))
            .await
    }

    pub async fn account_activities(
        &self,
        token: &ApiToken,
        account_id: &str,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> Result<(), QuestradeError> {
        self.send(
            self.base_request(
                Method::GET,
                token,
                &format!("v1/accounts/{}/activities", account_id),
            )
            .query(&[
                ("startTime", DateTime::to_rfc3339(&start)),
                ("endTime", DateTime::to_rfc3339(&end)),
            ]),
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accounts_deserialize_works() {
        let data = r#"
        {
            "accounts": [
                {
                    "type": "Margin",
                    "number": "26598145",
                    "status": "Active",
                    "isPrimary": true,
                    "isBilling": true,
                    "clientAccountType": "Individual"
                }
            ]
        }
        "#;
        let _v: Accounts = serde_json::from_str(data).expect("failed to deserialize JSON");
    }

    #[test]
    fn account_activities_deserialize_works() {
        let data = r#"
        {
            "activities": [
                 {
                    "tradeDate": "2011-02-16T00:00:00.000000-05:00",
                    "transactionDate": "2011-02-16T00:00:00.000000-05:00",
                    "settlementDate":  "2011-02-16T00:00:00.000000-05:00",
                    "action": "",
                    "symbol":  "",
                    "symbolId": 0,
                    "description": "INT FR 02/04 THRU02/15@ 4 3/4%BAL 205,006 AVBAL 204,966",
                    "currency": "USD",
                    "quantity": 0,
                    "price":  0,
                    "grossAmount":  0,
                    "commission":  0,
                    "netAmount": -320.08,
                    "type": "Interest"
                  }
               ]
          }
        "#;
        let _v: Activities = serde_json::from_str(data).expect("failed to deserialize JSON");
        println!("{:#?}", _v);
    }
}
