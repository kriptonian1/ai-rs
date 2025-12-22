use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use serde_json::Value;

const NAME: &str = "AI_APICallError";

#[derive(Debug)]
pub struct APICallError {
    pub url: String,
    pub request_body_values: Value,
    pub status_code: Option<u16>,
    pub response_headers: Option<HashMap<String, String>>,
    pub response_body: Option<String>,
    pub is_retryable: bool,
    pub data: Option<Value>,

    pub message: String,
    pub cause: Option<Box<dyn Error + Send + Sync + 'static>>,
}

impl APICallError {
    pub fn new(
        message: String,
        url: String,
        request_body_values: Value,
        status_code: Option<u16>,
        response_headers: Option<HashMap<String, String>>,
        response_body: Option<String>,
        cause: Option<Box<dyn Error + Send + Sync + 'static>>,
        is_retryable: Option<bool>,
        data: Option<Value>,
    ) -> Self {
        let is_retryable = is_retryable.unwrap_or_else(|| {
            if let Some(code) = status_code {
                code == 408 || code == 409 || code == 429 || code >= 500
            } else {
                false
            }
        });

        Self {
            url,
            request_body_values,
            status_code,
            response_headers,
            response_body,
            is_retryable,
            data,
            message,
            cause,
        }
    }
}

impl fmt::Display for APICallError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", NAME, self.message)
    }
}

