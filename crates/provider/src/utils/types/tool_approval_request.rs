use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub enum ToolApprovalRequest {
    ToolApprovalRequest {
        #[serde(rename = "approvalID")]
        approval_id: String,
        #[serde(rename = "toolCallID")]
        tool_call_id: String,
    },
}