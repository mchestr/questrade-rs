use chrono::Utc;
use oauth2::{
    basic::{
        BasicErrorResponse, BasicRevocationErrorResponse, BasicTokenIntrospectionResponse,
        BasicTokenType,
    },
    reqwest::async_http_client,
    ExtraTokenFields, RefreshToken, StandardRevocableToken, StandardTokenResponse, TokenResponse,
};
use serde::{Deserialize, Serialize};

use crate::{client::Client, errors::QuestradeError};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ApiServer {
    pub(crate) api_server: String,
}

impl ExtraTokenFields for ApiServer {}

pub(crate) type AuthClient = oauth2::Client<
    BasicErrorResponse,
    StandardTokenResponse<ApiServer, BasicTokenType>,
    BasicTokenType,
    BasicTokenIntrospectionResponse,
    StandardRevocableToken,
    BasicRevocationErrorResponse,
>;

#[derive(Debug)]
pub struct ApiToken {
    pub access_token: String,
    pub refresh_token: String,
    pub api_server: url::Url,
    pub expires_at: chrono::DateTime<Utc>,
}

impl From<StandardTokenResponse<ApiServer, BasicTokenType>> for ApiToken {
    fn from(t: StandardTokenResponse<ApiServer, BasicTokenType>) -> Self {
        ApiToken {
            access_token: t.access_token().secret().into(),
            refresh_token: t.refresh_token().unwrap().secret().into(),
            api_server: url::Url::parse(&t.extra_fields().api_server.clone()).unwrap(),
            expires_at: Utc::now() + chrono::Duration::from_std(t.expires_in().unwrap()).unwrap(),
        }
    }
}

impl Client {
    pub async fn refresh_token(&self, refresh_token: &str) -> Result<ApiToken, QuestradeError> {
        let token = RefreshToken::new(refresh_token.into());
        Ok(self
            .auth_client
            .exchange_refresh_token(&token)
            .request_async(&async_http_client)
            .await
            .map_err(|err| QuestradeError::InternalError(err.to_string()))?
            .into())
    }
}
