use serde::{Deserialize, Serialize};
use serde_json::Value;
use super::super::types::ProviderMetadata;

fn default_tool_result_type() -> String {
    "tool-result".to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StaticToolResult {
    #[serde(default = "default_tool_result_type", rename = "type")]
    pub kind: String,
    pub tool_call_id: String,
    pub tool_name: String,
    pub input: Value,
    pub output: Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_executed: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_metadata: Option<ProviderMetadata>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preliminary: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DynamicToolResult {
    #[serde(default = "default_tool_result_type", rename = "type")]
    pub kind: String,
    pub tool_call_id: String,
    pub tool_name: String,
    pub input: Value,
    pub output: Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_executed: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_metadata: Option<ProviderMetadata>,
    pub dynamic: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preliminary: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum TypedToolResult {
    Static(StaticToolResult),
    Dynamic(DynamicToolResult),
}

impl<'de> Deserialize<'de> for TypedToolResult {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let v: Value = Deserialize::deserialize(deserializer)?;
        let is_dynamic = v.get("dynamic").and_then(|d| d.as_bool()).unwrap_or(false);

        if is_dynamic {
            let result: DynamicToolResult = serde_json::from_value(v).map_err(serde::de::Error::custom)?;
            Ok(TypedToolResult::Dynamic(result))
        } else {
            let result: StaticToolResult = serde_json::from_value(v).map_err(serde::de::Error::custom)?;
            Ok(TypedToolResult::Static(result))
        }
    }
}
