use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use serde_json::Value;
use super::sdk_errors::{
    MARKER,
    AISDKError,
    extended_marker,
};

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

    pub fn is_instance(error: &(dyn Error + 'static)) -> bool {
        Self::has_marker(error, extended_marker(MARKER, NAME).as_str())
    }

    fn has_marker(error: &(dyn Error + 'static), _marker: &str) -> bool {
        error.downcast_ref::<APICallError>().is_some()
    }
}

impl fmt::Display for APICallError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", NAME, self.message)
    }
}

impl Error for APICallError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.cause.as_ref().map(|e| &**e as &(dyn Error + 'static))
    }
}

impl From<APICallError> for AISDKError {
    fn from(error: APICallError) -> Self {
        AISDKError::new(
            NAME,
            error.message.clone(),
            Some(Box::new(error)),
        )
    }
}

#[cfg(test)]
mod test_api_call_error {
    use super::*;
    use serde_json::json;
    use std::collections::HashMap;

    #[test]
    fn test_new_api_call_error_basic() {
        let message = "Something went wrong".to_string();
        let url = "https://api.example.com".to_string();
        let request_body = json!({"key": "value"});
        let status_code = Some(400);
        let response_headers = None;
        let response_body = Some("Bad Request".to_string());
        let cause = None;
        let is_retryable = None;
        let data = None;

        let error = APICallError::new(
            message.clone(),
            url.clone(),
            request_body.clone(),
            status_code,
            response_headers,
            response_body.clone(),
            cause,
            is_retryable,
            data,
        );

        assert_eq!(error.message, message);
        assert_eq!(error.url, url);
        assert_eq!(error.status_code, status_code);
        assert_eq!(error.response_body, response_body);
        // 400 is not retryable by default
        assert!(!error.is_retryable);
    }

    #[test]
    fn test_retryable_status_codes() {
        let retryable_codes = vec![408, 409, 429, 500, 502, 503, 504];

        for code in retryable_codes {
            let error = APICallError::new(
                "Error".to_string(),
                "https://test.com".to_string(),
                json!({}),
                Some(code),
                None,
                None,
                None,
                None, // Let default logic take over
                None,
            );
            assert!(error.is_retryable, "Status code {} should be retryable", code);
        }
    }

    #[test]
    fn test_non_retryable_status_codes() {
        let non_retryable_codes = vec![200, 400, 401, 403, 404];

        for code in non_retryable_codes {
            let error = APICallError::new(
                "Error".to_string(),
                "https://test.com".to_string(),
                json!({}),
                Some(code),
                None,
                None,
                None,
                None, // Let default logic take over
                None,
            );
            assert!(!error.is_retryable, "Status code {} should not be retryable", code);
        }
    }

    #[test]
    fn test_explicit_retryable_override() {
        // 400 is usually not retryable, but we force it to true
        let error_true = APICallError::new(
            "Error".to_string(),
            "http://test.com".to_string(),
            json!({}),
            Some(400),
            None,
            None,
            None,
            Some(true),
            None,
        );
        assert!(error_true.is_retryable);

        // 500 is usually retryable, but we force it to false
        let error_false = APICallError::new(
            "Error".to_string(),
            "http://test.com".to_string(),
            json!({}),
            Some(500),
            None,
            None,
            None,
            Some(false),
            None,
        );
        assert!(!error_false.is_retryable);
    }

    #[test]
    fn test_display_implementation() {
        let error = APICallError::new(
            "Connection failed".to_string(),
            "http://test.com".to_string(),
            json!({}),
            None,
            None,
            None,
            None,
            None,
            None,
        );

        assert_eq!(format!("{}", error), "AI_APICallError: Connection failed");
    }

    #[test]
    fn test_with_headers_and_data() {
        let mut headers = HashMap::new();
        headers.insert("Content-Type".to_string(), "application/json".to_string());
        let data = json!({"details": "extra info"});

        let error = APICallError::new(
            "Error".to_string(),
            "http://test.com".to_string(),
            json!({}),
            Some(500),
            Some(headers.clone()),
            None,
            None,
            None,
            Some(data.clone()),
        );

        assert_eq!(error.response_headers, Some(headers));
        assert_eq!(error.data, Some(data));
    }

    #[test]
    fn test_error_source() {
        let cause = std::io::Error::new(std::io::ErrorKind::Other, "underlying IO error");
        let api_call_error = APICallError::new(
            "API call failed".to_string(),
            "http://test.com".to_string(),
            json!({}),
            None,
            None,
            None,
            Some(Box::new(cause)),
            None,
            None,
        );

        let source = api_call_error.source();
        assert!(source.is_some());
        assert_eq!(source.unwrap().to_string(), "underlying IO error");
    }

    #[test]
    fn test_display_with_cause() {
        let cause = std::io::Error::new(std::io::ErrorKind::Other, "underlying IO error");
        let api_call_error = APICallError::new(
            "API call failed".to_string(),
            "http://test.com".to_string(),
            json!({}),
            None,
            None,
            None,
            Some(Box::new(cause)),
            None,
            None,
        );

        assert_eq!(format!("{}", api_call_error), "AI_APICallError: API call failed");
    }


    #[test]
    fn test_is_instance_method() {
        let message = "Something went wrong".to_string();
        let url = "https://api.example.com".to_string();
        let request_body = json!({"key": "value"});
        let status_code = Some(400);
        let response_headers = None;
        let response_body = Some("Bad Request".to_string());
        let cause = None;
        let is_retryable = None;
        let data = None;

        let api_call_error = APICallError::new(
            message.clone(),
            url.clone(),
            request_body.clone(),
            status_code,
            response_headers,
            response_body.clone(),
            cause,
            is_retryable,
            data,
        );

        assert!(APICallError::is_instance(&api_call_error));

        // Now lets test if APICallError Instance is actually AISDKError
        let sdk_error: AISDKError = api_call_error.into();
        assert!(AISDKError::is_instance(&sdk_error));
    }
}