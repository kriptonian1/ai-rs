use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type", rename = "tool-approval-response")] // rename_all = "kebab-case" is not working here
pub struct ToolApprovalResponse {
    #[serde(rename = "approval_id")]
    pub approval_id: String,
    pub approved: bool,
    pub reason: Option<String>,
}