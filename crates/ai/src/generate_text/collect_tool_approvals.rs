use provider::{
    utils::types::{
        ModelMessage,
        ModelMessages,
        ToolApprovalRequest,
        ToolApprovalResponse,
        ContentPart,
        AssistantContent,
        AssistantContentPart,
        ToolContentPart,
    },
};
use super::tool_call::{TypedToolCall, StaticToolCall};
use std::collections::{HashMap, HashSet};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CollectedToolApprovals {
    pub approval_request: ToolApprovalRequest,
    pub approval_response: ToolApprovalResponse,
    pub tool_call: TypedToolCall,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ToolApprovalsResult {
    pub approved_tool_approvals: Vec<CollectedToolApprovals>,
    pub denied_tool_approvals: Vec<CollectedToolApprovals>,
}

pub fn collect_tool_approvals(messages: &ModelMessages) -> ToolApprovalsResult {
    let last_message = match messages.last() {
        Some(msg) => msg,
        None => return ToolApprovalsResult {
            approved_tool_approvals: vec![],
            denied_tool_approvals: vec![],
        },
    };

    // Check if last message is a tool message
    let last_tool_message = match last_message {
        ModelMessage::Tool(m) => m,
        _ => return ToolApprovalsResult {
            approved_tool_approvals: vec![],
            denied_tool_approvals: vec![],
        },
    };

    // Gather tool calls
    let mut tool_calls_by_id: HashMap<String, TypedToolCall> = HashMap::new();
    for message in messages {
        if let ModelMessage::Assistant(assistant_msg) = message {
            if let AssistantContent::Parts(parts) = &assistant_msg.content {
                for part in parts {
                    if let AssistantContentPart::ContentPart(ContentPart::ToolCall(tool_call_part)) = part {
                        let static_tool_call = StaticToolCall {
                            tool_call_id: tool_call_part.tool_call_id.clone(),
                            tool_name: tool_call_part.tool_name.clone(),
                            input: tool_call_part.input.clone(),
                            dynamic: None,
                            invalid: None,
                            title: None,
                        };
                        tool_calls_by_id.insert(
                            tool_call_part.tool_call_id.clone(),
                            TypedToolCall::Static(static_tool_call),
                        );
                    }
                }
            }
        }
    }

    // Gather approval requests
    let mut approval_requests_by_id: HashMap<String, ToolApprovalRequest> = HashMap::new();
    for message in messages {
        if let ModelMessage::Assistant(assistant_msg) = message {
            if let AssistantContent::Parts(parts) = &assistant_msg.content {
                for part in parts {
                    if let AssistantContentPart::ToolApprovalRequest(request) = part {
                        if let ToolApprovalRequest::ToolApprovalRequest { approval_id, .. } = request {
                             approval_requests_by_id.insert(approval_id.clone(), request.clone());
                        }
                    }
                }
            }
        }
    }

    // Gather tool results from last message
    let mut tool_results_ids: HashSet<String> = HashSet::new();
    for part in &last_tool_message.content {
        if let ToolContentPart::ToolResult(result_part) = part {
            tool_results_ids.insert(result_part.tool_call_id.clone());
        }
    }

    let mut approved_tool_approvals = Vec::new();
    let mut denied_tool_approvals = Vec::new();

    // Process approval responses
    for part in &last_tool_message.content {
        if let ToolContentPart::ToolApproval(approval_response) = part {
            let approval_id = &approval_response.approval_id;

            if let Some(approval_request) = approval_requests_by_id.get(approval_id) {
                 let ToolApprovalRequest::ToolApprovalRequest { tool_call_id, .. } = approval_request;
                 if tool_results_ids.contains(tool_call_id) {
                     continue;
                 }

                 if let Some(tool_call) = tool_calls_by_id.get(tool_call_id) {
                     let collected = CollectedToolApprovals {
                         approval_request: approval_request.clone(),
                         approval_response: approval_response.clone(),
                         tool_call: tool_call.clone(),
                     };

                     if approval_response.approved {
                         approved_tool_approvals.push(collected);
                     } else {
                         denied_tool_approvals.push(collected);
                     }
                 }
            }
        }
    }

    ToolApprovalsResult {
        approved_tool_approvals,
        denied_tool_approvals,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use provider::utils::types::{
        AssistantModelMessage, ToolModelMessage, UserModelMessage,
        AssistantContent, AssistantContentPart, ToolContentPart,
        ContentPart, ToolCallPart, ToolApprovalRequest, ToolApprovalResponse,
        ToolResultPart, ToolResultOutput, TextOutput, ExecutionDeniedOutput,
        UserContent,
    };
    use serde_json::json;

    #[test]
    fn test_should_not_return_any_tool_approvals_when_last_message_is_not_tool_message() {
        let messages = vec![
            ModelMessage::User(UserModelMessage {
                role: "user".to_string(),
                content: UserContent::String("Hello, world!".to_string()),
                provider_options: None,
            })
        ];

        let result = collect_tool_approvals(&messages);
        let json_result = serde_json::to_value(&result).unwrap();

        assert_eq!(json_result, json!({
            "approved_tool_approvals": [],
            "denied_tool_approvals": []
        }));
    }

    #[test]
    fn test_should_ignore_approval_request_without_response() {
        let messages = vec![
            ModelMessage::Assistant(AssistantModelMessage {
                role: "assistant".to_string(),
                content: AssistantContent::Parts(vec![
                    AssistantContentPart::ContentPart(ContentPart::ToolCall(ToolCallPart {
                        tool_call_id: "call-1".to_string(),
                        tool_name: "tool1".to_string(),
                        input: json!({ "value": "test-input" }),
                        provider_options: None,
                        provider_executed: false,
                    })),
                    AssistantContentPart::ToolApprovalRequest(ToolApprovalRequest::ToolApprovalRequest {
                        approval_id: "approval-id-1".to_string(),
                        tool_call_id: "call-1".to_string(),
                    }),
                ]),
                provider_options: None,
            }),
            ModelMessage::Tool(ToolModelMessage {
                content: vec![],
                provider_options: None,
            }),
        ];

        let result = collect_tool_approvals(&messages);
        let json_result = serde_json::to_value(&result).unwrap();

        assert_eq!(json_result, json!({
            "approved_tool_approvals": [],
            "denied_tool_approvals": []
        }));
    }

    #[test]
    fn test_should_return_approved_approval_with_approved_response() {
        let messages = vec![
            ModelMessage::Assistant(AssistantModelMessage {
                role: "assistant".to_string(),
                content: AssistantContent::Parts(vec![
                    AssistantContentPart::ContentPart(ContentPart::ToolCall(ToolCallPart {
                        tool_call_id: "call-1".to_string(),
                        tool_name: "tool1".to_string(),
                        input: json!({ "value": "test-input" }),
                        provider_options: None,
                        provider_executed: false,
                    })),
                    AssistantContentPart::ToolApprovalRequest(ToolApprovalRequest::ToolApprovalRequest {
                        approval_id: "approval-id-1".to_string(),
                        tool_call_id: "call-1".to_string(),
                    }),
                ]),
                provider_options: None,
            }),
            ModelMessage::Tool(ToolModelMessage {
                content: vec![
                    ToolContentPart::ToolApproval(ToolApprovalResponse {
                        approval_id: "approval-id-1".to_string(),
                        approved: true,
                        reason: None,
                    }),
                ],
                provider_options: None,
            }),
        ];

        let result = collect_tool_approvals(&messages);
        let json_result = serde_json::to_value(&result).unwrap();

        assert_eq!(json_result, json!({
            "approved_tool_approvals": [
                {
                    "approval_request": {
                        "type": "tool-approval-request",
                        "approvalID": "approval-id-1",
                        "toolCallID": "call-1"
                    },
                    "approval_response": {
                        "type": "tool-approval-response",
                        "approval_id": "approval-id-1",
                        "approved": true,
                        "reason": null
                    },
                    "tool_call": {
                        "toolCallId": "call-1",
                        "toolName": "tool1",
                        "input": {
                            "value": "test-input"
                        }
                    }
                }
            ],
            "denied_tool_approvals": []
        }));
    }

    #[test]
    fn test_should_return_processed_approval_with_approved_response_and_tool_result() {
        let messages = vec![
            ModelMessage::Assistant(AssistantModelMessage {
                role: "assistant".to_string(),
                content: AssistantContent::Parts(vec![
                    AssistantContentPart::ContentPart(ContentPart::ToolCall(ToolCallPart {
                        tool_call_id: "call-1".to_string(),
                        tool_name: "tool1".to_string(),
                        input: json!({ "value": "test-input" }),
                        provider_options: None,
                        provider_executed: false,
                    })),
                    AssistantContentPart::ToolApprovalRequest(ToolApprovalRequest::ToolApprovalRequest {
                        approval_id: "approval-id-1".to_string(),
                        tool_call_id: "call-1".to_string(),
                    }),
                ]),
                provider_options: None,
            }),
            ModelMessage::Tool(ToolModelMessage {
                content: vec![
                    ToolContentPart::ToolApproval(ToolApprovalResponse {
                        approval_id: "approval-id-1".to_string(),
                        approved: true,
                        reason: None,
                    }),
                    ToolContentPart::ToolResult(ToolResultPart {
                        tool_call_id: "call-1".to_string(),
                        tool_name: "tool1".to_string(),
                        output: ToolResultOutput::Text(TextOutput {
                            value: "test-output".to_string(),
                            provider_options: None,
                        }),
                        provider_options: None,
                    }),
                ],
                provider_options: None,
            }),
        ];

        let result = collect_tool_approvals(&messages);
        let json_result = serde_json::to_value(&result).unwrap();

        assert_eq!(json_result, json!({
            "approved_tool_approvals": [],
            "denied_tool_approvals": []
        }));
    }

    #[test]
    fn test_should_return_denied_approval_with_denied_response() {
        let messages = vec![
            ModelMessage::Assistant(AssistantModelMessage {
                role: "assistant".to_string(),
                content: AssistantContent::Parts(vec![
                    AssistantContentPart::ContentPart(ContentPart::ToolCall(ToolCallPart {
                        tool_call_id: "call-1".to_string(),
                        tool_name: "tool1".to_string(),
                        input: json!({ "value": "test-input" }),
                        provider_options: None,
                        provider_executed: false,
                    })),
                    AssistantContentPart::ToolApprovalRequest(ToolApprovalRequest::ToolApprovalRequest {
                        approval_id: "approval-id-1".to_string(),
                        tool_call_id: "call-1".to_string(),
                    }),
                ]),
                provider_options: None,
            }),
            ModelMessage::Tool(ToolModelMessage {
                content: vec![
                    ToolContentPart::ToolApproval(ToolApprovalResponse {
                        approval_id: "approval-id-1".to_string(),
                        approved: false,
                        reason: Some("test-reason".to_string()),
                    }),
                ],
                provider_options: None,
            }),
        ];

        let result = collect_tool_approvals(&messages);
        let json_result = serde_json::to_value(&result).unwrap();

        assert_eq!(json_result, json!({
            "approved_tool_approvals": [],
            "denied_tool_approvals": [
                {
                    "approval_request": {
                        "type": "tool-approval-request",
                        "approvalID": "approval-id-1",
                        "toolCallID": "call-1"
                    },
                    "approval_response": {
                        "type": "tool-approval-response",
                        "approval_id": "approval-id-1",
                        "approved": false,
                        "reason": "test-reason"
                    },
                    "tool_call": {
                        "toolCallId": "call-1",
                        "toolName": "tool1",
                        "input": {
                            "value": "test-input"
                        }
                    }
                }
            ]
        }));
    }

    #[test]
    fn test_should_return_processed_approval_with_denied_response_and_tool_result() {
        let messages = vec![
            ModelMessage::Assistant(AssistantModelMessage {
                role: "assistant".to_string(),
                content: AssistantContent::Parts(vec![
                    AssistantContentPart::ContentPart(ContentPart::ToolCall(ToolCallPart {
                        tool_call_id: "call-1".to_string(),
                        tool_name: "tool1".to_string(),
                        input: json!({ "value": "test-input" }),
                        provider_options: None,
                        provider_executed: false,
                    })),
                    AssistantContentPart::ToolApprovalRequest(ToolApprovalRequest::ToolApprovalRequest {
                        approval_id: "approval-id-1".to_string(),
                        tool_call_id: "call-1".to_string(),
                    }),
                ]),
                provider_options: None,
            }),
            ModelMessage::Tool(ToolModelMessage {
                content: vec![
                    ToolContentPart::ToolApproval(ToolApprovalResponse {
                        approval_id: "approval-id-1".to_string(),
                        approved: false,
                        reason: Some("test-reason".to_string()),
                    }),
                    ToolContentPart::ToolResult(ToolResultPart {
                        tool_call_id: "call-1".to_string(),
                        tool_name: "tool1".to_string(),
                        output: ToolResultOutput::ExecutionDenied(ExecutionDeniedOutput {
                            reason: Some("test-reason".to_string()),
                            provider_options: None,
                        }),
                        provider_options: None,
                    }),
                ],
                provider_options: None,
            }),
        ];

        let result = collect_tool_approvals(&messages);
        let json_result = serde_json::to_value(&result).unwrap();

        assert_eq!(json_result, json!({
            "approved_tool_approvals": [],
            "denied_tool_approvals": []
        }));
    }

    #[test]
    fn test_should_work_for_mixed_approvals_and_rejections() {
        let messages = vec![
            ModelMessage::Assistant(AssistantModelMessage {
                role: "assistant".to_string(),
                content: AssistantContent::Parts(vec![
                    AssistantContentPart::ContentPart(ContentPart::ToolCall(ToolCallPart {
                        tool_call_id: "call-approval-1".to_string(),
                        tool_name: "tool1".to_string(),
                        input: json!({ "value": "test-input-1" }),
                        provider_options: None,
                        provider_executed: false,
                    })),
                    AssistantContentPart::ToolApprovalRequest(ToolApprovalRequest::ToolApprovalRequest {
                        approval_id: "approval-id-1".to_string(),
                        tool_call_id: "call-approval-1".to_string(),
                    }),
                    AssistantContentPart::ContentPart(ContentPart::ToolCall(ToolCallPart {
                        tool_call_id: "call-approval-2".to_string(),
                        tool_name: "tool1".to_string(),
                        input: json!({ "value": "test-input-2" }),
                        provider_options: None,
                        provider_executed: false,
                    })),
                    AssistantContentPart::ToolApprovalRequest(ToolApprovalRequest::ToolApprovalRequest {
                        approval_id: "approval-id-2".to_string(),
                        tool_call_id: "call-approval-2".to_string(),
                    }),
                    AssistantContentPart::ContentPart(ContentPart::ToolCall(ToolCallPart {
                        tool_call_id: "call-approval-3".to_string(),
                        tool_name: "tool1".to_string(),
                        input: json!({ "value": "test-input-3" }),
                        provider_options: None,
                        provider_executed: false,
                    })),
                    AssistantContentPart::ToolApprovalRequest(ToolApprovalRequest::ToolApprovalRequest {
                        approval_id: "approval-id-3".to_string(),
                        tool_call_id: "call-approval-3".to_string(),
                    }),
                    AssistantContentPart::ContentPart(ContentPart::ToolCall(ToolCallPart {
                        tool_call_id: "call-approval-4".to_string(),
                        tool_name: "tool1".to_string(),
                        input: json!({ "value": "test-input-4" }),
                        provider_options: None,
                        provider_executed: false,
                    })),
                    AssistantContentPart::ToolApprovalRequest(ToolApprovalRequest::ToolApprovalRequest {
                        approval_id: "approval-id-4".to_string(),
                        tool_call_id: "call-approval-4".to_string(),
                    }),
                    AssistantContentPart::ContentPart(ContentPart::ToolCall(ToolCallPart {
                        tool_call_id: "call-approval-5".to_string(),
                        tool_name: "tool1".to_string(),
                        input: json!({ "value": "test-input-5" }),
                        provider_options: None,
                        provider_executed: false,
                    })),
                    AssistantContentPart::ToolApprovalRequest(ToolApprovalRequest::ToolApprovalRequest {
                        approval_id: "approval-id-5".to_string(),
                        tool_call_id: "call-approval-5".to_string(),
                    }),
                    AssistantContentPart::ContentPart(ContentPart::ToolCall(ToolCallPart {
                        tool_call_id: "call-approval-6".to_string(),
                        tool_name: "tool1".to_string(),
                        input: json!({ "value": "test-input-6" }),
                        provider_options: None,
                        provider_executed: false,
                    })),
                    AssistantContentPart::ToolApprovalRequest(ToolApprovalRequest::ToolApprovalRequest {
                        approval_id: "approval-id-6".to_string(),
                        tool_call_id: "call-approval-6".to_string(),
                    }),
                ]),
                provider_options: None,
            }),
            ModelMessage::Tool(ToolModelMessage {
                content: vec![
                    ToolContentPart::ToolApproval(ToolApprovalResponse {
                        approval_id: "approval-id-1".to_string(),
                        approved: true,
                        reason: None,
                    }),
                    ToolContentPart::ToolApproval(ToolApprovalResponse {
                        approval_id: "approval-id-2".to_string(),
                        approved: true,
                        reason: None,
                    }),
                    ToolContentPart::ToolApproval(ToolApprovalResponse {
                        approval_id: "approval-id-3".to_string(),
                        approved: false,
                        reason: Some("test-reason".to_string()),
                    }),
                    ToolContentPart::ToolApproval(ToolApprovalResponse {
                        approval_id: "approval-id-4".to_string(),
                        approved: false,
                        reason: None,
                    }),
                    ToolContentPart::ToolApproval(ToolApprovalResponse {
                        approval_id: "approval-id-5".to_string(),
                        approved: true,
                        reason: None,
                    }),
                    ToolContentPart::ToolResult(ToolResultPart {
                        tool_call_id: "call-approval-5".to_string(),
                        tool_name: "tool1".to_string(),
                        output: ToolResultOutput::Text(TextOutput {
                            value: "test-output-5".to_string(),
                            provider_options: None,
                        }),
                        provider_options: None,
                    }),
                    ToolContentPart::ToolApproval(ToolApprovalResponse {
                        approval_id: "approval-id-6".to_string(),
                        approved: false,
                        reason: None,
                    }),
                    ToolContentPart::ToolResult(ToolResultPart {
                        tool_call_id: "call-approval-6".to_string(),
                        tool_name: "tool1".to_string(),
                        output: ToolResultOutput::ExecutionDenied(ExecutionDeniedOutput {
                            reason: None,
                            provider_options: None,
                        }),
                        provider_options: None,
                    }),
                ],
                provider_options: None,
            }),
        ];

        let result = collect_tool_approvals(&messages);
        let json_result = serde_json::to_value(&result).unwrap();

        assert_eq!(json_result, json!({
            "approved_tool_approvals": [
                {
                    "approval_request": {
                        "type": "tool-approval-request",
                        "approvalID": "approval-id-1",
                        "toolCallID": "call-approval-1"
                    },
                    "approval_response": {
                        "type": "tool-approval-response",
                        "approval_id": "approval-id-1",
                        "approved": true,
                        "reason": null
                    },
                    "tool_call": {
                        "toolCallId": "call-approval-1",
                        "toolName": "tool1",
                        "input": {
                            "value": "test-input-1"
                        }
                    }
                },
                {
                    "approval_request": {
                        "type": "tool-approval-request",
                        "approvalID": "approval-id-2",
                        "toolCallID": "call-approval-2"
                    },
                    "approval_response": {
                        "type": "tool-approval-response",
                        "approval_id": "approval-id-2",
                        "approved": true,
                        "reason": null
                    },
                    "tool_call": {
                        "toolCallId": "call-approval-2",
                        "toolName": "tool1",
                        "input": {
                            "value": "test-input-2"
                        }
                    }
                }
            ],
            "denied_tool_approvals": [
                {
                    "approval_request": {
                        "type": "tool-approval-request",
                        "approvalID": "approval-id-3",
                        "toolCallID": "call-approval-3"
                    },
                    "approval_response": {
                        "type": "tool-approval-response",
                        "approval_id": "approval-id-3",
                        "approved": false,
                        "reason": "test-reason"
                    },
                    "tool_call": {
                        "toolCallId": "call-approval-3",
                        "toolName": "tool1",
                        "input": {
                            "value": "test-input-3"
                        }
                    }
                },
                {
                    "approval_request": {
                        "type": "tool-approval-request",
                        "approvalID": "approval-id-4",
                        "toolCallID": "call-approval-4"
                    },
                    "approval_response": {
                        "type": "tool-approval-response",
                        "approval_id": "approval-id-4",
                        "approved": false,
                        "reason": null
                    },
                    "tool_call": {
                        "toolCallId": "call-approval-4",
                        "toolName": "tool1",
                        "input": {
                            "value": "test-input-4"
                        }
                    }
                }
            ]
        }));
    }
}
