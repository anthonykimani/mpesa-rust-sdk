pub mod config;
pub mod mpesa;
pub mod error;
pub mod types;

pub use config::{Config, Environment};
pub use mpesa::Mpesa;
pub use error::MpesaError;



#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Config, Environment};

    #[tokio::test]
    async fn test_oauth() {
        let config = Config::new(
            std::env::var("MPESA_CONSUMER_KEY").unwrap(),
            std::env::var("MPESA_SECRET_KEY").unwrap(),
            Environment::Sandbox
        );

        let mpesa = Mpesa::new(config).unwrap();
        let token = mpesa.oauth().await.unwrap();

        println!("{:?}", token);
    }
}
