use serde::{Deserialize, Serialize};
use serde_json::Value;
use super::tool_error::TypedToolError;
use super::tool_result::TypedToolResult;

#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum ToolOutput {
    Result(TypedToolResult),
    Error(TypedToolError),
}

impl<'de> Deserialize<'de> for ToolOutput {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let v: Value = Deserialize::deserialize(deserializer)?;
        let type_val = v.get("type").and_then(|t| t.as_str());

        match type_val {
            Some("tool-result") => {
                let result: TypedToolResult = serde_json::from_value(v).map_err(serde::de::Error::custom)?;
                Ok(ToolOutput::Result(result))
            }
            Some("tool-error") => {
                let error: TypedToolError = serde_json::from_value(v).map_err(serde::de::Error::custom)?;
                Ok(ToolOutput::Error(error))
            }
            _ => Err(serde::de::Error::custom("Missing or invalid 'type' field for ToolOutput")),
        }
    }
}
