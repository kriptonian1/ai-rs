use std::any::Any;
use std::collections::HashMap;
use std::fmt;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use super::{
    super::{
        super::json_value::json_value::JSONValue
    },
    content_part::ToolResultOutput,
    model_message::ModelMessages,
    provider_options::ProviderOptions,
};

pub type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;

pub struct ToolExecutionOptions {
    pub tool_call_id: String,
    pub messages: ModelMessages,
    pub abort_signal: Option<JSONValue>,
    pub experimental_context: Option<Box<dyn Any + Send + Sync>>,
}

impl fmt::Debug for ToolExecutionOptions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ToolExecutionOptions")
            .field("tool_call_id", &self.tool_call_id)
            .field("messages", &self.messages)
            .field("abort_signal", &self.abort_signal)
            .field("experimental_context", &"Box<dyn Any + Send + Sync>")
            .finish()
    }
}

pub struct ToolNeedsApprovalOptions {
    pub tool_call_id: String,
    pub messages: ModelMessages,
    pub experimental_context: Option<Box<dyn Any + Send + Sync>>,
}

pub type ToolExecuteFn = dyn Fn(JSONValue, ToolExecutionOptions) -> BoxFuture<'static, Result<JSONValue, Box<dyn std::error::Error + Send + Sync>>>
    + Send
    + Sync;

pub type ToolNeedsApprovalFn =
    dyn Fn(JSONValue, ToolNeedsApprovalOptions) -> BoxFuture<'static, bool> + Send + Sync;

pub type ToolOnInputStartFn =
    dyn Fn(ToolExecutionOptions) -> BoxFuture<'static, ()> + Send + Sync;

pub type ToolOnInputDeltaFn =
    dyn Fn(String, ToolExecutionOptions) -> BoxFuture<'static, ()> + Send + Sync;

pub type ToolOnInputAvailableFn =
    dyn Fn(JSONValue, ToolExecutionOptions) -> BoxFuture<'static, ()> + Send + Sync;

pub type ToolToModelOutputFn =
    dyn Fn(ToolModelOutputOptions) -> BoxFuture<'static, ToolResultOutput> + Send + Sync;

pub struct ToolModelOutputOptions {
    pub tool_call_id: String,
    pub input: JSONValue,
    pub output: JSONValue,
}

#[derive(Clone, Debug)]
pub enum ToolType {
    Function,
    Dynamic,
    Provider(ProviderTool),
}

#[derive(Clone, Debug)]
pub struct ProviderTool {
    pub id: String,
    pub args: HashMap<String, JSONValue>,
    pub supports_deferred_results: Option<bool>,
}

#[derive(Clone)]
pub enum ToolNeedsApproval {
    Boolean(bool),
    Function(Arc<ToolNeedsApprovalFn>),
}

#[derive(Clone)]
pub struct Tool {
    pub description: Option<String>,
    pub title: Option<String>,
    pub provider_options: Option<ProviderOptions>,
    pub input_schema: JSONValue,
    pub input_examples: Option<Vec<JSONValue>>,
    pub needs_approval: Option<ToolNeedsApproval>,
    pub strict: Option<bool>,
    pub on_input_start: Option<Arc<ToolOnInputStartFn>>,
    pub on_input_delta: Option<Arc<ToolOnInputDeltaFn>>,
    pub on_input_available: Option<Arc<ToolOnInputAvailableFn>>,
    pub execute: Option<Arc<ToolExecuteFn>>,
    pub to_model_output: Option<Arc<ToolToModelOutputFn>>,
    pub tool_type: ToolType,
}

impl fmt::Debug for Tool {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Tool")
            .field("description", &self.description)
            .field("title", &self.title)
            .field("provider_options", &self.provider_options)
            .field("input_schema", &self.input_schema)
            .field("input_examples", &self.input_examples)
            .field("needs_approval", &self.needs_approval.is_some())
            .field("strict", &self.strict)
            .field("tool_type", &self.tool_type)
            .finish()
    }
}
