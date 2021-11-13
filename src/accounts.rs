use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{
    auth::ApiToken, client::Client, errors::QuestradeError, AccountStatus, AccountType,
    ClientAccountType,
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

impl Client {
    pub async fn accounts(&self, token: &ApiToken) -> Result<Accounts, QuestradeError> {
        self.send(self.base_request(Method::GET, token, "v1/accounts"))
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
}
