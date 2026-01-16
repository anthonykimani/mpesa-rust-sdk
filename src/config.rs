
#[derive(Debug, Clone)]
pub enum Environment {
    Production,
    Sandbox
}


#[derive(Debug, Clone)]
pub struct Config {
    pub consumer_key: String,
    pub consumer_secret: String,
    pub environment: Environment,
    pub base_url: String,
}

impl Config {
    pub fn new(
        consumer_key: impl Into<String>,
        consumer_secret: impl Into<String>,
        environment: impl Into<Environment>,
        base_url: impl Into<String>,
    ) -> Self {
        Self {
            consumer_key: consumer_key.into(),
            consumer_secret: consumer_secret.into(),
            environment: environment.into(),
            base_url: base_url.into()
        }
    }


}
