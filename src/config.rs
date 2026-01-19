
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Environment {
    Production,
    Sandbox
}


#[derive(Debug, Clone)]
pub struct Config {
    pub consumer_key: String,
    pub consumer_secret: String,
    pub environment: Environment,
}

impl Config {
    pub fn new(
        consumer_key: impl Into<String>,
        consumer_secret: impl Into<String>,
        environment: Environment,
    ) -> Self {
        Self {
            consumer_key: consumer_key.into(),
            consumer_secret: consumer_secret.into(),
            environment: environment,
        }
    }

    pub fn base_url(&self) -> &str {
        match self.environment {
            Environment::Production => "https://api.safaricom.co.ke",
            Environment::Sandbox => "https://sandbox.safaricom.co.ke"
        }
    }
}
