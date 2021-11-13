use chrono::{DateTime, Utc};
use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{
    auth::ApiToken, client::Client, errors::QuestradeError, AccountStatus, AccountType,
    ActivityType, ClientAccountType, Currency, StateFilter,
};

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
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

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
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

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Balances {
    pub per_currency_balances: Vec<Balance>,
    pub combined_balances: Vec<Balance>,
    pub sod_per_currency_balances: Vec<Balance>,
    pub sod_combined_balances: Vec<Balance>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Balance {
    pub currency: Currency,
    pub cash: f64,
    pub market_value: f64,
    pub total_equity: f64,
    pub buying_power: f64,
    pub maintenance_excess: f64,
    pub is_real_time: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Position {
    pub symbol: String,
    pub symbol_id: u64,
    pub open_quantity: f64,
    pub closed_quantity: f64,
    pub current_market_value: f64,
    pub current_price: f64,
    pub average_entry_price: f64,
    pub closed_pnl: f64,
    pub open_pnl: f64,
    pub total_cost: f64,
    pub is_real_time: bool,
    pub is_under_reorg: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Execution {
    pub symbol: String,
    pub symbol_id: i64,
    pub quantity: i64,
    pub side: String,
    pub price: f64,
    pub id: i64,
    pub order_id: i64,
    pub order_chain_id: i64,
    pub exchange_exec_id: String,
    pub timestamp: DateTime<Utc>,
    pub notes: String,
    pub venue: String,
    pub total_cost: f64,
    pub order_placement_commission: f64,
    pub commission: f64,
    pub execution_fee: f64,
    pub sec_fee: f64,
    pub canadian_execution_fee: i64,
    pub parent_id: i64,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Order {}

impl Client {
    pub async fn accounts(&self, token: &ApiToken) -> Result<Vec<Account>, QuestradeError> {
        #[derive(Deserialize)]
        pub struct Accounts {
            pub accounts: Vec<Account>,
        }

        let data: Accounts = self
            .send(self.base_request(Method::GET, token, "v1/accounts"))
            .await?;
        Ok(data.accounts)
    }

    pub async fn account_activities(
        &self,
        token: &ApiToken,
        account_id: &str,
        start: &DateTime<Utc>,
        end: &DateTime<Utc>,
    ) -> Result<Vec<Activity>, QuestradeError> {
        #[derive(Deserialize)]
        pub struct Activities {
            pub activities: Vec<Activity>,
        }

        let data: Activities = self
            .send(
                self.base_request(
                    Method::GET,
                    token,
                    &format!("v1/accounts/{}/activities", account_id),
                )
                .query(&[
                    ("startTime", start.to_rfc3339()),
                    ("endTime", end.to_rfc3339()),
                ]),
            )
            .await?;
        Ok(data.activities)
    }

    pub async fn account_balances(
        &self,
        token: &ApiToken,
        account_id: &str,
    ) -> Result<Balances, QuestradeError> {
        self.send(self.base_request(
            Method::GET,
            token,
            &format!("v1/accounts/{}/balances", account_id),
        ))
        .await
    }

    pub async fn account_positions(
        &self,
        token: &ApiToken,
        account_id: &str,
    ) -> Result<Vec<Position>, QuestradeError> {
        #[derive(Deserialize)]
        pub struct Positions {
            pub positions: Vec<Position>,
        }

        let data: Positions = self
            .send(self.base_request(
                Method::GET,
                token,
                &format!("v1/accounts/{}/positions", account_id),
            ))
            .await?;
        Ok(data.positions)
    }

    pub async fn account_executions(
        &self,
        token: &ApiToken,
        account_id: &str,
        start: Option<&DateTime<Utc>>,
        end: Option<&DateTime<Utc>>,
    ) -> Result<Vec<Execution>, QuestradeError> {
        #[derive(Deserialize)]
        pub struct Executions {
            pub executions: Vec<Execution>,
        }

        let builder = self.base_request(
            Method::GET,
            token,
            &format!("v1/accounts/{}/executions", account_id),
        );

        let mut query_params: Vec<(&str, String)> = Vec::new();
        if let Some(start) = start {
            query_params.push(("startTime", start.to_rfc3339()));
        };
        if let Some(end) = end {
            query_params.push(("endTime", end.to_rfc3339()));
        }

        let data: Executions = self.send(builder.query(query_params.as_slice())).await?;
        Ok(data.executions)
    }

    pub async fn account_orders(
        &self,
        token: &ApiToken,
        account_id: &str,
        start: Option<&DateTime<Utc>>,
        end: Option<&DateTime<Utc>>,
        state_filter: Option<StateFilter>,
    ) -> Result<Vec<Order>, QuestradeError> {
        let builder = self.base_request(
            Method::GET,
            token,
            &format!("v1/accounts/{}/orders", account_id),
        );

        let mut query_params: Vec<(&str, String)> = Vec::new();
        if let Some(start) = start {
            query_params.push(("startTime", start.to_rfc3339()));
        };
        if let Some(end) = end {
            query_params.push(("endTime", end.to_rfc3339()));
        }
        if let Some(state) = state_filter {
            query_params.push(("stateFilter", format!("{}", state)));
        }
        self.send(builder).await
    }

    pub async fn account_order(
        &self,
        token: &ApiToken,
        account_id: &str,
        order_id: i64,
    ) -> Result<Order, QuestradeError> {
        self.send(self.base_request(
            Method::GET,
            token,
            &format!("v1/accounts/{}/orders/{}", account_id, order_id),
        ))
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accounts_deserialize_works() {
        #[derive(Deserialize)]
        struct Data {
            accounts: Vec<Account>,
        }
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
        let expected = &Account {
            type_: AccountType::Margin,
            number: "26598145".into(),
            status: AccountStatus::Active,
            is_billing: true,
            is_primary: true,
            client_account_type: ClientAccountType::Individual,
        };
        let d: Data = serde_json::from_str(data).expect("failed to deserialize JSON");
        let account = d.accounts.get(0).unwrap();
        assert_eq!(expected, account)
    }

    #[test]
    fn account_activities_deserialize_works() {
        #[derive(Deserialize)]
        struct Data {
            activities: Vec<Activity>,
        }
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
        let expected = &Activity {
            trade_date: DateTime::parse_from_rfc3339("2011-02-16T05:00:00+00:00")
                .unwrap()
                .with_timezone(&Utc),
            transaction_date: DateTime::parse_from_rfc3339("2011-02-16T05:00:00+00:00")
                .unwrap()
                .with_timezone(&Utc),
            settlement_date: DateTime::parse_from_rfc3339("2011-02-16T05:00:00+00:00")
                .unwrap()
                .with_timezone(&Utc),
            action: "".into(),
            symbol: "".into(),
            symbol_id: 0,
            description: "INT FR 02/04 THRU02/15@ 4 3/4%BAL 205,006 AVBAL 204,966".into(),
            currency: Currency::USD,
            quantity: 0.0,
            price: 0.0,
            gross_amount: 0.0,
            commission: 0.0,
            net_amount: -320.08,
            type_: ActivityType::Interest,
        };
        let d: Data = serde_json::from_str(data).expect("failed to deserialize JSON");
        let activity = d.activities.get(0).unwrap();
        assert_eq!(expected, activity);
    }

    #[test]
    fn account_balances_deserialize_works() {
        let data = r#"
        {
            "perCurrencyBalances": [
                {
                    "currency": "CAD",
                    "cash": 243971.7,
                    "marketValue":  6017,
                    "totalEquity":  249988.7,
                    "buyingPower": 496367.3,
                    "maintenanceExcess": 248183.65,
                    "isRealTime": false
                }
            ],
            "combinedBalances": [
                {
                    "currency": "CAD",
                    "cash": 243971.7,
                    "marketValue":  6017,
                    "totalEquity":  249988.7,
                    "buyingPower": 496367.3,
                    "maintenanceExcess": 248183.65,
                    "isRealTime": false
                }
            ],
            "sodPerCurrencyBalances": [
                {
                    "currency": "CAD",
                    "cash": 243971.7,
                    "marketValue":  6017,
                    "totalEquity":  249988.7,
                    "buyingPower": 496367.3,
                    "maintenanceExcess": 248183.65,
                    "isRealTime": false
                }
            ],
            "sodCombinedBalances": [
                {
                    "currency": "CAD",
                    "cash": 243971.7,
                    "marketValue":  6017,
                    "totalEquity":  249988.7,
                    "buyingPower": 496367.3,
                    "maintenanceExcess": 248183.65,
                    "isRealTime": false
                }
            ]
        }
        "#;
        let expected_balance = vec![Balance {
            currency: Currency::CAD,
            cash: 243971.7,
            market_value: 6017.0,
            total_equity: 249988.7,
            buying_power: 496367.3,
            maintenance_excess: 248183.65,
            is_real_time: false,
        }];
        let expected = Balances {
            combined_balances: expected_balance.clone(),
            per_currency_balances: expected_balance.clone(),
            sod_combined_balances: expected_balance.clone(),
            sod_per_currency_balances: expected_balance.clone(),
        };
        let data: Balances = serde_json::from_str(data).expect("failed to deserialize JSON");
        assert_eq!(expected, data);
    }

    #[test]
    fn account_positions_deserialize_works() {
        #[derive(Deserialize)]
        struct Data {
            positions: Vec<Position>,
        }
        let data = r#"
        {
            "positions": [
                {
                    "symbol": "THI.TO",
                    "symbolId": 38738,
                    "openQuantity": 100,
                    "closedQuantity": 100,
                    "currentMarketValue": 6017,
                    "currentPrice": 60.17,
                    "averageEntryPrice": 60.23,
                    "closedPnl": 0,
                    "openPnl": -6,
                    "totalCost": 10.0,
                    "isRealTime": true,
                    "isUnderReorg": false
                }
            ]
        }
        "#;
        let expected = &Position {
            symbol: "THI.TO".into(),
            symbol_id: 38738,
            open_quantity: 100.0,
            closed_quantity: 100.0,
            current_market_value: 6017.0,
            current_price: 60.17,
            average_entry_price: 60.23,
            closed_pnl: 0.0,
            open_pnl: -6.0,
            total_cost: 10.0,
            is_real_time: true,
            is_under_reorg: false,
        };
        let d: Data = serde_json::from_str(data).expect("failed to deserialize JSON");
        let position = d.positions.get(0).unwrap();
        assert_eq!(expected, position);
    }

    #[test]
    fn account_executions_deserialize_works() {
        #[derive(Deserialize)]
        struct Data {
            executions: Vec<Execution>,
        }
        let data = r#"
        {
            "executions": [
                {
                    "symbol": "AAPL",
                    "symbolId": 8049,
                    "quantity":   10,
                    "side":  "Buy",
                    "price": 536.87,
                    "id": 53817310,
                    "orderId": 177106005,
                    "orderChainId": 17710600,
                    "exchangeExecId": "XS1771060050147",
                    "timestamp":  "2014-03-31T13:38:29.000000-04:00",
                    "notes":  "",
                    "venue":  "LAMP",
                    "totalCost":   5368.7,
                    "orderPlacementCommission": 0,
                    "commission":    4.95,
                    "executionFee": 0,
                    "secFee": 0,
                    "canadianExecutionFee": 0,
                    "parentId": 0
                }
            ]
        }
        "#;
        let expected = &Execution {
            symbol: "AAPL".into(),
            symbol_id: 8049,
            quantity: 10,
            side: "Buy".into(),
            price: 536.87,
            id: 53817310,
            order_id: 177106005,
            order_chain_id: 17710600,
            exchange_exec_id: "XS1771060050147".into(),
            timestamp: DateTime::parse_from_rfc3339("2014-03-31T13:38:29.000000-04:00")
                .unwrap()
                .with_timezone(&Utc),
            notes: "".into(),
            venue: "LAMP".into(),
            total_cost: 5368.7,
            order_placement_commission: 0.0,
            commission: 4.95,
            execution_fee: 0.0,
            sec_fee: 0.0,
            canadian_execution_fee: 0,
            parent_id: 0,
        };
        let d: Data = serde_json::from_str(data).expect("failed to deserialize JSON");
        let position = d.executions.get(0).unwrap();
        assert_eq!(expected, position);
    }
}
