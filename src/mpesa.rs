
use reqwest::{Client};
use crate::config::{Config, Environment};
use crate::error::MpesaError;


#[derive(Debug, Clone)]
pub struct Mpesa {
    config: Config,
    client: Client
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

        Ok(Self {
            config,
            client
        })
    }

    pub fn account_balance(&self) -> Result<(), MpesaError> {
        Ok(())
    }

    pub fn c2b_simulate(&self) -> Result<(), MpesaError> {
        if self.config.environment == Environment::Production {
            return Err(MpesaError::NotAllowedInProduction)
        }
        Ok(())
    }

    pub fn base_url(&self) -> &str {
        self.config.base_url()
    }

}