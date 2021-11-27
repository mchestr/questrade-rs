use chrono::Utc;
use reqwest::{Method, RequestBuilder};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::{
    auth::ApiToken,
    errors::{ApiResponse, QuestradeError},
    Environment,
};

pub struct Client {
    pub(crate) http: reqwest::Client,
    pub(crate) env: Environment,
    pub(crate) consumer_key: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Time {
    pub time: chrono::DateTime<Utc>,
}

impl Client {
    pub fn new(
        http_client: reqwest::Client,
        consumer_key: String,
        env: Environment,
    ) -> Result<Self, QuestradeError> {
        Ok(Client {
            http: http_client,
            env,
            consumer_key,
        })
    }

    pub fn builder() -> ClientBuilder {
        ClientBuilder::default()
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

#[derive(Debug, Default)]
pub struct ClientBuilder {
    http_client: Option<reqwest::Client>,
    consumer_key: Option<String>,
    env: Option<Environment>,
}

impl ClientBuilder {
    pub fn http_client(mut self, http_client: reqwest::Client) -> Self {
        self.http_client = Some(http_client);
        self
    }

    pub fn consumer_key(mut self, consumer_key: String) -> Self {
        self.consumer_key = Some(consumer_key);
        self
    }

    pub fn env(mut self, env: Environment) -> Self {
        self.env = Some(env);
        self
    }

    pub fn build(self) -> Result<Client, QuestradeError> {
        let http_client = self.http_client.ok_or_else(|| {
            QuestradeError::Builder(String::from("http_client must be specified"))
        })?;
        let consumer_key = self.consumer_key.ok_or_else(|| {
            QuestradeError::Builder(String::from("consumer_key must be specified"))
        })?;
        let env = self.env.unwrap_or(Environment::Production);

        Client::new(http_client, consumer_key, env)
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
