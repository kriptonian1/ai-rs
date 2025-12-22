use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub(crate) struct ToolApprovalResponse {
    pub approval_id: String,
    pub approved: bool,
    reason: Option<String>,
}