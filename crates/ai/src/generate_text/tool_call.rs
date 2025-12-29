use provider::utils::types::Tool;
use super::{
    super::types::ProviderMetadata,
    tool_set::ToolSet,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone)]
struct BaseToolCallOptions {
    pub tool_call_id: String,
    pub tool: Tool,
    pub tool_set: ToolSet,
    pub provider_metadata: ProviderMetadata,
}

struct StaticToolCallOptions {
    pub base: BaseToolCallOptions,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StaticToolCall {
    pub tool_call_id: String,
    pub tool_name: String,
    pub input: Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invalid: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DynamicToolCall {
    pub tool_call_id: String,
    pub tool_name: String,
    pub input: Value,
    pub dynamic: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invalid: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum TypedToolCall {
    Static(StaticToolCall),
    Dynamic(DynamicToolCall),
}

impl<'de> Deserialize<'de> for TypedToolCall {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let v: Value = Deserialize::deserialize(deserializer)?;
        let is_dynamic = v.get("dynamic").and_then(|d| d.as_bool()).unwrap_or(false);

        if is_dynamic {
            let call: DynamicToolCall = serde_json::from_value(v).map_err(serde::de::Error::custom)?;
            Ok(TypedToolCall::Dynamic(call))
        } else {
            let call: StaticToolCall = serde_json::from_value(v).map_err(serde::de::Error::custom)?;
            Ok(TypedToolCall::Static(call))
        }
    }
}
