use crate::config::Config;
use crate::error::MpesaError;


#[derive(Debug, Clone)]
pub struct Mpesa {
    config: Config,

}

impl Mpesa {
    pub fn new(config: Config) -> Result<Self, MpesaError> {
        if config.consumer_key.is_empty() {
            return Err(MpesaError::MissingConsumerKey)
        }

        if config.consumer_secret.is_empty() {
            return Err(MpesaError::MissingConsumerSecret)
        }

        Ok(Self {
            config
        })
    }

}