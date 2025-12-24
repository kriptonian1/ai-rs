use super::sdk_errors::{
    AISDKError,
    MARKER,
    extended_marker
};
use serde_json::Value;
use std::collections::HashMap;
use std::error::Error;
use std::fmt;

/// The name of the APICallError type.
const NAME: &str = "AI_APICallError";

/// Represents an error that occurs during an API call.
/// This error contains detailed information about the API request and response,
/// including the URL, request body, status code, response headers, and response body.
/// It also indicates whether the error is retryable based on the status code.
/// For more information, see the documentation for `APICallError`.
/// # Fields
/// - `url`: The URL of the API endpoint that was called.
/// - `request_body_values`: The JSON values sent in the request body.
/// - `status_code`: The HTTP status code returned by the API (if available).
/// - `response_headers`: The headers returned in the API response (if available).
/// - `response_body`: The body of the API response as a string (if available).
/// - `is_retryable`: Indicates whether the error is retryable based on the status code
/// - `data`: Additional JSON data related to the error (if available).
/// - `message`: A descriptive message about the error.
/// - `cause`: An optional underlying cause of the error.
/// # Example
/// ```rust
/// use serde_json::json;
/// use your_crate::errors::APICallError;
/// let api_error = APICallError::new(
///     "Failed to call API".to_string(),
///     "https://api.example.com/endpoint".to_string(),
///     json!({"param": "value"}),
///     Some(500),
///     None,
///     Some("Internal Server Error".to_string()),
///     None,
///     None,
///     None,
/// );
/// assert!(api_error.is_retryable);
/// ```
/// # See Also
/// - `AISDKError`: The base error type for the AI SDK.
/// - `extended_marker`: Function to create extended error markers.
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


/// Creates a new `APICallError` instance.
/// /// # Arguments
/// - `message`: A descriptive message about the error.
/// - `url`: The URL of the API endpoint that was called.
/// - `request_body_values`: The JSON values sent in the request body.
/// - `status_code`: The HTTP status code returned by the API (if available).
/// - `response_headers`: The headers returned in the API response (if available).
/// - `response_body`: The body of the API response as a string (if available).
/// - `cause`: An optional underlying cause of the error.
/// - `is_retryable`: An optional flag indicating whether the error is retryable.
///  If not provided, it is determined based on the status code.
/// - `data`: Additional JSON data related to the error (if available).
/// # Returns
/// A new instance of `APICallError`.
/// # Example
/// ```rust
/// use serde_json::json;
/// use your_crate::errors::APICallError;
/// let api_error = APICallError::new(
///     "Failed to call API".to_string(),
///     "https://api.example.com/endpoint".to_string(),
///     json!({"param": "value"}),
///     Some(500),
///     None,
///     Some("Internal Server Error".to_string()),
///     None,
///     None,
///     None,
/// );
/// assert!(api_error.is_retryable);
/// ```
///
/// # See Also
/// - `AISDKError`: The base error type for the AI SDK.
/// - `extended_marker`: Function to create extended error markers.
/// - `APICallError`: The error type representing API call errors.
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
            message,
            url,
            request_body_values,
            status_code,
            response_headers,
            response_body,
            cause,
            is_retryable,
            data,
        }
    }


    /// Checks if the given error is an instance of `APICallError`.
    /// This method checks both direct instances of `APICallError` and those
    /// wrapped within an `AISDKError`.
    /// # Arguments
    /// - `error`: A reference to a trait object implementing `Error`.
    /// # Returns
    /// `true` if the error is an instance of `APICallError`, `false` otherwise.
    /// # Example
    /// ```rust
    /// use your_crate::errors::{APICallError, AISDKError};
    /// let api_error = APICallError::new(
    ///     "Failed to call API".to_string(),
    ///     "https://api.example.com/endpoint".to_string(),
    ///     serde_json::json!({"param": "value"}),
    ///     Some(500),
    ///     None,
    ///     Some("Internal Server Error".to_string()),
    ///     None,
    ///     None,
    ///     None,
    /// );
    /// assert!(APICallError::is_instance(&api_error)); // true
    /// let sdk_error: AISDKError = api_error.into();
    /// assert!(APICallError::is_instance(&sdk_error)); // true because it's wrapped
    /// ```
    pub fn is_instance(error: &(dyn Error + 'static)) -> bool {
        Self::has_marker(error, extended_marker(MARKER, NAME).as_str())
            && (error.downcast_ref::<APICallError>().is_some()
                || (if let Some(sdk_error) = error.downcast_ref::<AISDKError>() {
                    if let Some(cause) = sdk_error.source() {
                        cause.downcast_ref::<APICallError>().is_some()
                    } else {
                        false
                    }
                } else {
                    false
                }))
    }

    fn has_marker(error: &(dyn Error + 'static), marker: &str) -> bool {
        error.downcast_ref::<APICallError>().is_some()
            || AISDKError::has_marker(error, marker)
    }
}


/// Implements the `Display` trait for `APICallError` to provide a user-friendly
/// string representation of the error.
/// # Example
/// ```rust
/// use serde_json::json;
/// use your_crate::errors::APICallError;
/// let api_error = APICallError::new(
///     "Failed to call API".to_string(),
///     "https://api.example.com/endpoint".to_string(),
///     json!({"param": "value"}),
///     Some(500),
///     None,
///     Some("Internal Server Error".to_string()),
///     None,
///     None,
///     None,
/// );
/// println!("{}", api_error); // Outputs: AI_APICallError: Failed to call API
/// ```
impl fmt::Display for APICallError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", NAME, self.message)
    }
}


/// Implements the `Error` trait for `APICallError`, allowing it to be used
/// as a standard error type in Rust.
/// The `source` method returns the underlying cause of the error, if any.
/// # Example
/// ```rust
/// use serde_json::json;
/// use your_crate::errors::APICallError;
/// let api_error = APICallError::new(
///     "Failed to call API".to_string(),
///     "https://api.example.com/endpoint".to_string(),
///     json!({"param": "value"}),
///     Some(500),
///     None,
///     Some("Internal Server Error".to_string()),
///     None,
///     None,
///     None,
/// );
/// if let Some(source) = api_error.source() {
///    println!("Caused by: {}", source);
/// }
/// ```
impl Error for APICallError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.cause.as_ref().map(|e| &**e as &(dyn Error + 'static))
    }
}


/// Implements conversion from `APICallError` to `AISDKError`.
/// This allows for seamless integration of `APICallError` within the broader
/// error handling framework of the AI SDK.
/// # Example
/// ```rust
/// use serde_json::json;
/// use your_crate::errors::{APICallError, AISDKError};
/// let api_error = APICallError::new(
///     "Failed to call API".to_string(),
///     "https://api.example.com/endpoint".to_string(),
///     json!({"param": "value"}),
///     Some(500),
///     None,
///     Some("Internal Server Error".to_string()),
///     None,
///     None,
///     None,
/// );
/// let sdk_error: AISDKError = api_error.into();
/// println!("{}", sdk_error); // Outputs: AI_APICallError: Failed to call API
/// ```
impl From<APICallError> for AISDKError {
    fn from(error: APICallError) -> Self {
        AISDKError::new(NAME, error.message.clone(), Some(Box::new(error)))
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
            assert!(
                error.is_retryable,
                "Status code {} should be retryable",
                code
            );
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
            assert!(
                !error.is_retryable,
                "Status code {} should not be retryable",
                code
            );
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

        assert_eq!(
            format!("{}", api_call_error),
            "AI_APICallError: API call failed"
        );
    }

    #[test]
    fn test_conversion_to_aisdk_error() {
        let api_call_error = APICallError::new(
            "API call failed".to_string(),
            "http://test.com".to_string(),
            json!({}),
            None,
            None,
            None,
            None,
            None,
            None,
        );

        let aisdk_error: AISDKError = api_call_error.into();
        assert_eq!(aisdk_error.name, NAME);
        assert_eq!(aisdk_error.message, "API call failed");
    }

    #[test]
    fn test_is_instance_method() {
        let api_call_error = APICallError::new(
            "API call failed".to_string(),
            "http://test.com".to_string(),
            json!({}),
            None,
            None,
            None,
            None,
            None,
            None,
        );

        assert!(APICallError::is_instance(&api_call_error));

        // Now lets test if APICallError Instance is actually AISDKError
        let sdk_error: AISDKError = api_call_error.into();
        assert_eq!(APICallError::is_instance(&sdk_error), true);
    }
}
