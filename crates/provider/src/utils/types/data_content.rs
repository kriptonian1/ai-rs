use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub(super) enum DataContent {
    String(String),
    Uint8Array(Vec<u8>),
    ArrayBuffer(Vec<u8>),
    Buffer(Vec<u8>),
}
