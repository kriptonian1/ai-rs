use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolCall<N, I> {
    /// ID of the tool call. Matches the tool call with the tool result.
    pub tool_call_id: String,

    /// Name of the tool that is being called.
    pub tool_name: N,

    /// Arguments of the tool call.
    pub input: I,

    /// Whether the tool call will be executed by the provider.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_executed: Option<bool>,

    /// Whether the tool is dynamic.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic: Option<bool>,
}