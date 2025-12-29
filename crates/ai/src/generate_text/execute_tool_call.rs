use std::any::Any;
use serde_json::Value;
use futures::StreamExt;
use provider::{
    json_value::json_value::JSONValue,
    utils::{
        is_async_iterable::IsAsyncIterable,
        types::{
            execute_tool,
            ToolOutput as ExecuteToolOutput,
            ModelMessages,
            ToolExecutionOptions,
        },
    }
};

use super::{
    tool_call::TypedToolCall,
    tool_error::TypedToolError,
    tool_output::ToolOutput,
    tool_result::{TypedToolResult, StaticToolResult, DynamicToolResult},
    tool_set::ToolSet,
};

pub async fn execute_tool_call(
    tool_call: TypedToolCall,
    tools: &ToolSet,
    messages: ModelMessages,
    abort_signal: Option<Value>,
    experimental_context: Option<Box<dyn Any + Send + Sync>>,
    on_preliminary_tool_result: Option<impl Fn(TypedToolResult) + Send + Sync>,
) -> Option<ToolOutput> {
    let (tool_name, tool_call_id, input, dynamic) = match &tool_call {
        TypedToolCall::Static(c) => (&c.tool_name, &c.tool_call_id, &c.input, false),
        TypedToolCall::Dynamic(c) => (&c.tool_name, &c.tool_call_id, &c.input, true),
    };

    let tool = tools.get(tool_name.as_str())?;

    if tool.execute.is_none() {
        return None;
    }

    let execute_fn = tool.execute.clone().unwrap();

    let execute_wrapper = |input: Value, options: ToolExecutionOptions| async move {
        let input_json: JSONValue = serde_json::from_value(input).unwrap_or(JSONValue::Null);
        match execute_fn(input_json, options).await {
            Ok(val) => Ok(IsAsyncIterable::Value(val)),
            Err(e) => Err(e),
        }
    };

    let abort_signal_json = abort_signal.map(|v| serde_json::from_value(v).unwrap_or(JSONValue::Null));

    let mut output: Option<Value> = None;
    let mut stream = Box::pin(execute_tool(
        execute_wrapper,
        input.clone(),
        ToolExecutionOptions {
            tool_call_id: tool_call_id.clone(),
            messages,
            abort_signal: abort_signal_json,
            experimental_context,
        },
    ));

    while let Some(result) = stream.next().await {
        match result {
            Ok(part) => match part {
                ExecuteToolOutput::Preliminary(out) => {
                    let out_val: Value = serde_json::to_value(out).unwrap_or(Value::Null);
                    if let Some(callback) = &on_preliminary_tool_result {
                        let result = if dynamic {
                            TypedToolResult::Dynamic(DynamicToolResult {
                                kind: "tool-result".to_string(),
                                tool_call_id: tool_call_id.clone(),
                                tool_name: tool_name.clone(),
                                input: input.clone(),
                                output: out_val,
                                provider_executed: None,
                                provider_metadata: None,
                                dynamic: true,
                                preliminary: Some(true),
                                title: None,
                            })
                        } else {
                            TypedToolResult::Static(StaticToolResult {
                                kind: "tool-result".to_string(),
                                tool_call_id: tool_call_id.clone(),
                                tool_name: tool_name.clone(),
                                input: input.clone(),
                                output: out_val,
                                provider_executed: None,
                                provider_metadata: None,
                                dynamic: None,
                                preliminary: Some(true),
                                title: None,
                            })
                        };
                        callback(result);
                    }
                }
                ExecuteToolOutput::Final(out) => {
                    let out_val: Value = serde_json::to_value(out).unwrap_or(Value::Null);
                    output = Some(out_val);
                }
            },
            Err(e) => {
                let error_val = serde_json::json!(e.to_string());
                let error = if dynamic {
                    TypedToolError::Dynamic(super::tool_error::DynamicToolError {
                        kind: "tool-error".to_string(),
                        tool_call_id: tool_call_id.clone(),
                        tool_name: tool_name.clone(),
                        input: input.clone(),
                        error: error_val,
                        provider_executed: None,
                        provider_metadata: None,
                        dynamic: true,
                        title: None,
                    })
                } else {
                    TypedToolError::Static(super::tool_error::StaticToolError {
                        kind: "tool-error".to_string(),
                        tool_call_id: tool_call_id.clone(),
                        tool_name: tool_name.clone(),
                        input: input.clone(),
                        error: error_val,
                        provider_executed: None,
                        provider_metadata: None,
                        dynamic: None,
                        title: None,
                    })
                };
                return Some(ToolOutput::Error(error));
            }
        }
    }

    if let Some(out) = output {
        let result = if dynamic {
            TypedToolResult::Dynamic(DynamicToolResult {
                kind: "tool-result".to_string(),
                tool_call_id: tool_call_id.clone(),
                tool_name: tool_name.clone(),
                input: input.clone(),
                output: out,
                provider_executed: None,
                provider_metadata: None,
                dynamic: true,
                preliminary: None,
                title: None,
            })
        } else {
            TypedToolResult::Static(StaticToolResult {
                kind: "tool-result".to_string(),
                tool_call_id: tool_call_id.clone(),
                tool_name: tool_name.clone(),
                input: input.clone(),
                output: out,
                provider_executed: None,
                provider_metadata: None,
                dynamic: None,
                preliminary: None,
                title: None,
            })
        };
        return Some(ToolOutput::Result(result));
    }

    None
}
