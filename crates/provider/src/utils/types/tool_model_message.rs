use serde::{Deserialize, Serialize};
use super::{
    provider_options::ProviderOptions,
    tool_approval_response::ToolApprovalResponse,
    content_part::ToolResultPart,
};

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "role", rename_all = "lowercase")]
pub struct ToolModelMessage {
    pub content: ToolContent,

    #[serde(rename = "providerOptions", skip_serializing_if = "Option::is_none")]
    pub provider_options: Option<ProviderOptions>,
}

/// Content of a tool message. It is an array of tool result parts or approval responses.
pub type ToolContent = Vec<ToolContentPart>;

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ToolContentPart {
    ToolResult(ToolResultPart),
    ToolApproval(ToolApprovalResponse),
}