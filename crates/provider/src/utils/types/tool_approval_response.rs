use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub struct ToolApprovalResponse {
    pub approval_id: String,
    pub approved: bool,
    pub reason: Option<String>,
}