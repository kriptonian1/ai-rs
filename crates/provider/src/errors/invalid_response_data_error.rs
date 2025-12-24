use super::sdk_errors::{
    AISDKError,
    MARKER,
    extended_marker
};
use std::error::Error;
use std::fmt;
use serde_json::Value;

const NAME: &str = "AI_InvalidResponseDataError";

#[derive(Debug)]
pub struct InvalidResponseDataError {
    pub data: Value,
    pub message: String,
}

impl InvalidResponseDataError {
    pub fn new(
        data: &Value,
        message: Option<String>,
    ) -> Self {
        let message = message.unwrap_or_else(|| {
            format!("Invalid response data: {}.", data)
        });
        Self {
            data: data.clone(),
            message,
        }
    }

    pub fn is_instance(error: &(dyn Error + 'static)) -> bool {
        Self::has_marker(error, extended_marker(MARKER, NAME).as_str())
    }

    fn has_marker(error: &(dyn Error + 'static), marker: &str) -> bool {
        error.downcast_ref::<InvalidResponseDataError>().is_some()
            || AISDKError::has_marker(error, marker)
    }
}

impl fmt::Display for InvalidResponseDataError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", NAME, self.message)
    }
}

impl Error for InvalidResponseDataError {}

impl From<InvalidResponseDataError> for AISDKError {
    fn from(error: InvalidResponseDataError) -> Self {
        AISDKError::new(
            NAME,
            error.message.clone(),
            Some(Box::new(error)),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_constructor() {
        let data = json!({"key": "value"});
        let error = InvalidResponseDataError::new(&data, None);

        assert_eq!(error.data, data);
        assert_eq!(error.message, format!("Invalid response data: {}.", data));
    }

    #[test]
    fn test_new_with_default_message() {
        let data = json!({"foo": "bar"});
        let error = InvalidResponseDataError::new(&data, None);

        assert_eq!(error.data, data);
        assert_eq!(error.message, format!("Invalid response data: {}.", data));
    }

    #[test]
    fn test_new_with_custom_message() {
        let data = json!({"foo": "bar"});
        let message = "Custom error message".to_string();
        let error = InvalidResponseDataError::new(&data, Some(message.clone()));

        assert_eq!(error.data, data);
        assert_eq!(error.message, message);
    }

    #[test]
    fn test_display() {
        let data = json!({"foo": "bar"});
        let error = InvalidResponseDataError::new(&data, None);
        assert_eq!(format!("{}", error), format!("{}: {}", NAME, error.message));
    }

    #[test]
    fn test_conversion_to_aisdk_error() {
        let data = json!({"foo": "bar"});
        let error = InvalidResponseDataError::new(&data, None);
        let aisdk_error: AISDKError = error.into();

        assert_eq!(aisdk_error.name, NAME);
        assert!(aisdk_error.message.contains("Invalid response data"));
    }

    #[test]
    fn test_is_instance() {
        let data = json!({"foo": "bar"});
        let error = InvalidResponseDataError::new(&data, None);
        let aisdk_error: AISDKError = error.into();
        assert!(InvalidResponseDataError::is_instance(&aisdk_error));
    }
}

