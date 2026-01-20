use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct TokenResponse {
    pub access_token: String,
    pub expires_in: String
}

#[derive(Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct AccountBalanceRequest {
    pub initiator: String,
    pub security_credential: String,
    pub command_id: String,
    pub party_a: String,
    pub identifier_type: String,
    pub remarks: String,
    pub queue_time_out_url: String,
    pub result_url: String
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AccountBalanceResponse {
    pub conversation_id: String,
    pub originator_conversation_id: String,
    pub response_code: String,
    pub response_description: String
}

