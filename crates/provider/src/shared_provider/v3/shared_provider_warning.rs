use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum SharedProviderWarningV3 {
    /// A feature is not supported by the model.
    Unsupported {
        /// The feature that is not supported.
        feature: String,
        /// Additional details about the warning.
        #[serde(skip_serializing_if = "Option::is_none")]
        details: Option<String>,
    },
    /// A compatibility feature is used that might lead to suboptimal results.
    Compatibility {
        /// The feature that is used in a compatibility mode.
        feature: String,
        /// Additional details about the warning.
        #[serde(skip_serializing_if = "Option::is_none")]
        details: Option<String>,
    },
    /// Other warning.
    Other {
        /// The message of the warning.
        message: String,
    },
}