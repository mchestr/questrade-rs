use chrono::Utc;
use oauth2::{AuthUrl, ClientId, TokenUrl};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use strum_macros::Display;
use url::Url;

use crate::{
    auth::{ApiToken, AuthClient},
    errors::{ApiResponse, QuestradeError},
};

pub struct Client {
    pub(crate) http: reqwest::Client,
    pub(crate) auth_client: AuthClient,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Time {
    pub time: chrono::DateTime<Utc>,
}

#[derive(Debug, Display, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
pub enum Environment {
    Production,
}

impl Environment {
    fn host(&self) -> Result<Url, QuestradeError> {
        Ok(match self {
            Environment::Production => Url::parse("https://login.questrade.com")?
        })
    }

    fn authorize_url(&self) -> Result<Url, QuestradeError> {
        Ok(self.host()?.join("/oauth2/authorize")?)
    }

    fn token_url(&self) -> Result<Url, QuestradeError> {
        Ok(self.host()?.join("/oauth2/token")?)
    }
}

impl Client {
    pub fn new(http_client: reqwest::Client, consumer_key: &str, env: Environment) -> Result<Self, QuestradeError> {
        let client_id = ClientId::new(consumer_key.into());
        let auth_url = AuthUrl::new(env.authorize_url()?.into()).unwrap();
        let token_url = TokenUrl::new(env.token_url()?.into()).unwrap();

        Ok(Client {
            http: http_client,
            auth_client: AuthClient::new(client_id, None, auth_url, Some(token_url)),
        })
    }

    pub(crate) async fn request<T>(
        &self,
        method: reqwest::Method,
        path: &str,
        token: &ApiToken,
    ) -> Result<T, QuestradeError>
    where
        T: DeserializeOwned,
    {
        let response = self
            .http
            .request(method, &format!("{}{}", token.api_server, path))
            .bearer_auth(&token.access_token)
            .send()
            .await?
            .json::<ApiResponse<T>>()
            .await?;

        match response {
            ApiResponse::Ok(time) => Ok(time),
            ApiResponse::Err(err) => Err(QuestradeError::ApiError(err)),
        }
    }

    pub async fn time(&self, token: &ApiToken) -> Result<Time, QuestradeError> {
        self.request(reqwest::Method::GET, "v1/time", token).await
    }
}
