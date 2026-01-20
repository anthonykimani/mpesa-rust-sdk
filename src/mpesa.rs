use crate::config::{Config, Environment};
use crate::error::MpesaError;
use crate::types::{AccountBalanceRequest, AccountBalanceResponse, TokenResponse};
use reqwest::Client;
use serde::Serialize;
use serde::de::DeserializeOwned;

#[derive(Debug, Clone)]
pub struct Mpesa {
    config: Config,
    client: Client,
}

impl Mpesa {
    pub fn new(config: Config) -> Result<Self, MpesaError> {
        if config.consumer_key.is_empty() {
            return Err(MpesaError::MissingConsumerKey);
        }

        if config.consumer_secret.is_empty() {
            return Err(MpesaError::MissingConsumerSecret);
        }

        let client = Client::new();

        Ok(Self { config, client })
    }

    pub async fn account_balance(&self, req: AccountBalanceRequest) -> Result<AccountBalanceResponse, MpesaError> {
        let path = "/mpesa/accountbalance/v1/query";
        self.post_json_helper(path, &req).await
    }

    pub async fn c2b_simulate(&self) -> Result<(), MpesaError> {
        if self.config.environment == Environment::Production {
            return Err(MpesaError::NotAllowedInProduction);
        }
        Ok(())
    }

    pub fn base_url(&self) -> &str {
        self.config.base_url()
    }

    pub async fn oauth(&self) -> Result<TokenResponse, MpesaError> {
        let path = "/oauth/v1/generate?grant_type=client_credentials";
        let url = format!("{}{}", self.base_url(), path);
        let response = self
            .client
            .get(url)
            .basic_auth(
                self.config.consumer_key.clone(),
                Some(self.config.consumer_secret.clone()),
            )
            .send()
            .await
            .map_err(|_| MpesaError::RequestFailed)?;

        if !response.status().is_success() {
            return Err(MpesaError::RequestFailed);
        }

        Ok(response
            .json::<TokenResponse>()
            .await
            .map_err(|_| MpesaError::JsonParseFailed)?)
    }

    async fn request_helper(&self, path: &str) -> Result<String, MpesaError> {
        let url = format!("{}{}", self.base_url(), path);
        let response = self
            .client
            .get(url)
            .send()
            .await
            .map_err(|_| MpesaError::RequestFailed)?;
        let body = response
            .text()
            .await
            .map_err(|_| MpesaError::RequestFailed)?;
        Ok(body)
    }

    async fn get_authed_text(&self, path: &str) -> Result<String, MpesaError> {
        let url = format!("{}{}", self.base_url(), path);
        let token = self.oauth().await?;
        let response = self
            .client
            .get(url)
            .bearer_auth(token.access_token)
            .send()
            .await
            .map_err(|_| MpesaError::RequestFailed)?;
        let body = response
            .text()
            .await
            .map_err(|_| MpesaError::RequestFailed)?;
        Ok(body)
    }

    async fn post_json_helper<T, R>(&self, path: &str, body: &T) -> Result<R, MpesaError>
    where
        T: Serialize,
        R: DeserializeOwned,
    {
        let url = format!("{}{}", self.base_url(), path);
        let token = self.oauth().await?;
        let response = self
            .client
            .post(url)
            .bearer_auth(token.access_token)
            .json(body)
            .send()
            .await
            .map_err(|_| MpesaError::RequestFailed)?;
        let body = response
            .json::<R>()
            .await
            .map_err(|_| MpesaError::JsonParseFailed)?;
        Ok(body)
    }
}
