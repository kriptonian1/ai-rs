use std::collections::HashMap;
use crate::json_value::json_value::JSONValue;
use super::{
    data_content::DataContent,
    provider_options::ProviderOptions
};
use serde_json::Value as unknown;
use serde::{ Serialize, Deserialize };

#[derive(Serialize, Deserialize, Debug)]
pub(super) enum DataOrURL {
    Data(DataContent),
    URL(String),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum FileIdType {
    String(String),
    Map(HashMap<String, String>),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ToolCallPart {
    #[serde(rename = "toolCallId")]
    pub tool_call_id: String,
    #[serde(rename = "toolName")]
    pub tool_name: String,
    pub input: unknown,
    #[serde(rename = "providerOptions", skip_serializing_if = "Option::is_none")]
    pub provider_options: Option<ProviderOptions>,
    #[serde(rename = "providerExecuted", default)]
    pub provider_executed: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ToolResultPart {
    #[serde(rename = "toolCallId")]
    pub tool_call_id: String,
    #[serde(rename = "toolName")]
    pub tool_name: String,
    pub output: ToolResultOutput,
    #[serde(rename = "providerOptions", skip_serializing_if = "Option::is_none")]
    pub provider_options: Option<ProviderOptions>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub enum ContentPart {
    Text {
        text: String,
        #[serde(rename = "providerOptions", skip_serializing_if = "Option::is_none")]
        provider_options: Option<ProviderOptions>,
    },
    Image {
        image: DataOrURL,
        #[serde(rename = "mediaType", skip_serializing_if = "Option::is_none")]
        media_type: Option<String>,
        #[serde(rename = "providerOptions", skip_serializing_if = "Option::is_none")]
        provider_options: Option<ProviderOptions>,
    },
    File {
        data: DataOrURL,
        #[serde(skip_serializing_if = "Option::is_none")]
        filename: Option<String>,
        #[serde(rename = "mediaType")]
        media_type: String,
        #[serde(rename = "providerOptions", skip_serializing_if = "Option::is_none")]
        provider_options: Option<ProviderOptions>,
    },
    Reasoning {
        text: String,
        #[serde(rename = "providerOptions", skip_serializing_if = "Option::is_none")]
        provider_options: Option<ProviderOptions>,
    },
    ToolCallPart(ToolCallPart),
    ToolResultPart(ToolResultPart),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub enum ToolResultOutput {
    Text {
        value: String,
        #[serde(rename = "providerOptions", skip_serializing_if = "Option::is_none")]
        provider_options: Option<ProviderOptions>,
    },
    Json {
        value: JSONValue,
        #[serde(rename = "providerOptions", skip_serializing_if = "Option::is_none")]
        provider_options: Option<ProviderOptions>,
    },
    ExecutionDenied {
        #[serde(skip_serializing_if = "Option::is_none")]
        reason: Option<String>,
        #[serde(rename = "providerOptions", skip_serializing_if = "Option::is_none")]
        provider_options: Option<ProviderOptions>,
    },
    ErrorText {
        value: String,
        #[serde(rename = "providerOptions", skip_serializing_if = "Option::is_none")]
        provider_options: Option<ProviderOptions>,
    },
    ErrorJson {
        value: JSONValue,
        #[serde(rename = "providerOptions", skip_serializing_if = "Option::is_none")]
        provider_options: Option<ProviderOptions>,
    },
    Content {
        value: Vec<ToolResultContentPart>,
    },
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub enum ToolResultContentPart {
    Text {
        text: String,
        #[serde(rename = "providerOptions", skip_serializing_if = "Option::is_none")]
        provider_options: Option<ProviderOptions>,
    },
    Media {
        data: String,
        #[serde(rename = "mediaType")]
        media_type: String,
    },
    FileData {
        data: String,
        #[serde(rename = "mediaType")]
        media_type: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        filename: Option<String>,
        #[serde(rename = "providerOptions", skip_serializing_if = "Option::is_none")]
        provider_options: Option<ProviderOptions>,
    },
    FileUrl {
        url: String,
        #[serde(rename = "providerOptions", skip_serializing_if = "Option::is_none")]
        provider_options: Option<ProviderOptions>,
    },
    FileId {
        #[serde(rename = "fileId")]
        file_id: FileIdType,
        #[serde(rename = "providerOptions", skip_serializing_if = "Option::is_none")]
        provider_options: Option<ProviderOptions>,
    },
    ImageData {
        data: String,
        #[serde(rename = "mediaType")]
        media_type: String,
        #[serde(rename = "providerOptions", skip_serializing_if = "Option::is_none")]
        provider_options: Option<ProviderOptions>,
    },
    ImageUrl {
        url: String,
        #[serde(rename = "providerOptions", skip_serializing_if = "Option::is_none")]
        provider_options: Option<ProviderOptions>,
    },
    ImageFileId {
        #[serde(rename = "fileId")]
        file_id: FileIdType,
        #[serde(rename = "providerOptions", skip_serializing_if = "Option::is_none")]
        provider_options: Option<ProviderOptions>,
    },
    Custom {
        #[serde(rename = "providerOptions", skip_serializing_if = "Option::is_none")]
        provider_options: Option<ProviderOptions>,
    },
}