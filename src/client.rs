use chrono::Utc;
use oauth2::{AuthUrl, ClientId, TokenUrl};
use reqwest::{Method, RequestBuilder};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::{
    auth::{ApiToken, AuthClient},
    errors::{ApiResponse, QuestradeError},
    Environment,
};

pub struct Client {
    pub(crate) http: reqwest::Client,
    pub(crate) auth_client: AuthClient,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Time {
    pub time: chrono::DateTime<Utc>,
}

impl Client {
    pub fn new(
        http_client: reqwest::Client,
        consumer_key: &str,
        env: Environment,
    ) -> Result<Self, QuestradeError> {
        let client_id = ClientId::new(consumer_key.into());
        let auth_url = AuthUrl::new(env.authorize_url()?.into()).unwrap();
        let token_url = TokenUrl::new(env.token_url()?.into()).unwrap();

        Ok(Client {
            http: http_client,
            auth_client: AuthClient::new(client_id, None, auth_url, Some(token_url)),
        })
    }

    pub(crate) async fn send<T>(
        &self,
        builder: reqwest::RequestBuilder,
    ) -> Result<T, QuestradeError>
    where
        T: DeserializeOwned,
    {
        let response = builder.send().await?.json::<ApiResponse<T>>().await?;

        match response {
            ApiResponse::Ok(data) => Ok(data),
            ApiResponse::Err(err) => Err(QuestradeError::ApiError(err)),
        }
    }

    pub(crate) fn base_request(
        &self,
        method: Method,
        token: &ApiToken,
        path: &str,
    ) -> RequestBuilder {
        self.http
            .request(method, format!("{}{}", token.api_server, path))
            .bearer_auth(&token.access_token)
    }

    pub async fn time(&self, token: &ApiToken) -> Result<Time, QuestradeError> {
        self.send(self.base_request(Method::GET, token, "v1/time"))
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn time_derserialize_works() {
        let data = r#"
        {
            "time": "2014-10-24T12:14:42.730000-04:00"
        }
        "#;
        let _t: Time = serde_json::from_str(data).expect("failed to deserialize JSON");
    }
}
