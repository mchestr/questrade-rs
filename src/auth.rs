use serde::{Deserialize, Serialize};

use crate::{client::Client, errors::QuestradeError};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub(crate) struct ApiServer {
    api_server: String,
}

#[derive(Debug, Deserialize)]
pub struct ApiToken {
    pub access_token: String,
    pub token_type: String,
    pub refresh_token: String,
    pub api_server: String,
    pub expires_in: usize,
}

impl Client {
    pub async fn refresh_token(&self, refresh_token: &str) -> Result<ApiToken, QuestradeError> {
        let params = [
            ("client_id", self.consumer_key.clone()),
            ("refresh_token", String::from(refresh_token)),
            ("grant_type", String::from("refresh_token")),
        ];
        self.send(
            self.http
                .request(reqwest::Method::POST, self.env.token_url()?)
                .form(&params),
        )
        .await
    }
}
