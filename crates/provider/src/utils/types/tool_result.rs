use serde::{Deserialize, Serialize};
use crate::json_value::json_value::JSONValue;

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct ToolResult {
    #[serde(rename = "toolCallId")]
    pub tool_call_id: String,
    #[serde(rename = "toolName")]
    pub tool_name: String,
    pub input: JSONValue,
    pub output: JSONValue,
    #[serde(rename = "providerExecuted", skip_serializing_if = "Option::is_none")]
    pub provider_executed: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic: Option<bool>,
}