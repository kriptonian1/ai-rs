use super::provider_options::ProviderOptions;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct SystemModelMessage {
    pub role: String,
    pub content: String,
    #[serde(rename = "providerOptions", skip_serializing_if = "Option::is_none")]
    pub provider_options: Option<ProviderOptions>,
}