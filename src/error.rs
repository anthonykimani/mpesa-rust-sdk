
pub enum MpesaError {
    MissingConsumerKey,
    MissingConsumerSecret,
    NotAllowedInProduction,
    RequestFailed,
    JsonParseFailed
}