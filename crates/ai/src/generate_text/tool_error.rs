use serde::{Deserialize, Serialize};
use serde_json::Value;
use super::super::types::ProviderMetadata;

fn default_tool_error_type() -> String {
    "tool-error".to_string()
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StaticToolError {
    #[serde(default = "default_tool_error_type", rename = "type")]
    pub kind: String,
    pub tool_call_id: String,
    pub tool_name: String,
    pub input: Value,
    pub error: Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_executed: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_metadata: Option<ProviderMetadata>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DynamicToolError {
    #[serde(default = "default_tool_error_type", rename = "type")]
    pub kind: String,
    pub tool_call_id: String,
    pub tool_name: String,
    pub input: Value,
    pub error: Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_executed: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_metadata: Option<ProviderMetadata>,
    pub dynamic: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

#[derive(Clone, Debug, Serialize)]
#[serde(untagged)]
pub enum TypedToolError {
    Static(StaticToolError),
    Dynamic(DynamicToolError),
}

impl<'de> Deserialize<'de> for TypedToolError {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let helper: serde_json::Value = Deserialize::deserialize(deserializer)?;
        let is_dynamic = helper
            .get("dynamic")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);

        if is_dynamic {
            let dynamic_error: DynamicToolError = serde_json::from_value(helper)
                .map_err(serde::de::Error::custom)?;
            Ok(TypedToolError::Dynamic(dynamic_error))
        } else {
            let static_error: StaticToolError = serde_json::from_value(helper)
                .map_err(serde::de::Error::custom)?;
            Ok(TypedToolError::Static(static_error))
        }
    }
}