use serde::{Deserialize, Serialize};
use super::{
    content_part::ContentPart,
    provider_options::ProviderOptions,
    tool_approval_request::ToolApprovalRequest,
};

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct AssistantModelMessage {
    pub role: String,
    pub content: AssistantContent,
    #[serde(rename = "providerOptions", skip_serializing_if = "Option::is_none")]
    pub provider_options: Option<ProviderOptions>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub(crate) enum AssistantContent {
    String(String),
    Parts(Vec<AssistantContentPart>),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub(crate) enum AssistantContentPart {
    ContentPart(ContentPart),
    ToolApprovalRequest(ToolApprovalRequest),
}