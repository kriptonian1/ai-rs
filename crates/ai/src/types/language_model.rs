use gateway::gateway_language_model_settings::GatewayModelId;
use provider::{
    language_model::v3::language_model_v3_source::LanguageModelV3Source,
    shared_provider::v3::shared_provider_warning::SharedProviderWarningV3,
};
use serde::{Deserialize, Serialize};

/// Enum to register custom model IDs.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RegisteredProviderModels {}

///Global provider model ID type that contains GatewayModelId but can be augmented
///by third-party packages via declaration merging.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum GlobalProviderModelId {
    Provider(RegisteredProviderModels),
    GatewayModel(GatewayModelId),
}

// TODO: Finish the V3 and V2 of language model @sambit003
/// Language model that is used by the AI SDK.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum LanguageModel {
    GlobalProviderModelId(GlobalProviderModelId),
    LanguageModelV3(),
    LanguageModelV2(),
}

///Reason why a language model finished generating a response.
///
/// Can be one of the following:
/// - `stop`: model generated stop sequence
/// - `length`: model generated maximum number of tokens
/// - `content-filter`: content filter violation stopped the model
/// - `tool-calls`: model triggered tool calls
/// - `error`: model stopped because of an error
/// - `other`: model stopped for other reasons
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum FinishReason {
    Stop,
    Length,
    ContentFilter,
    ToolCalls,
    Error,
    Other,
}

/// Warning from the model provider for this call. The call will proceed, but e.g.
/// some settings might not be supported, which can lead to suboptimal results.
pub type CallWarning = SharedProviderWarningV3;

/// A source that has been used as input to generate the response.
pub type Source = LanguageModelV3Source;

/// Tool choice for the generation. It supports the following settings:
/// - `Auto` (default): the model can choose whether and which tools to call.
/// - `Required`: the model must call a tool. It can choose which tool to call.
/// - `None`: the model must not call tools
/// - `ToolChoice::Tool { tool_name: String }`: the model must call the specified tool
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ToolChoice {
    Auto,
    None,
    Required,
    Tool { tool_name: String },
}
