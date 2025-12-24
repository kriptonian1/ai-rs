use super::sdk_errors::{
    AISDKError,
    MARKER,
    extended_marker
};
use std::error::Error;
use std::fmt;

const NAME: &str = "AI_LoadAPIKeyError";

#[derive(Debug)]
pub struct LoadAPIKeyError {
    message: String,
    base: AISDKError,
}

impl LoadAPIKeyError {
    pub fn new(
        message: &String,
    ) -> Self {
        Self {
            message: message.clone(),
            base: AISDKError::new(NAME, format!("Failed to load API key: {}", message), None),
        }
    }

    pub fn is_instance(error: &LoadAPIKeyError) -> bool {
        AISDKError::has_marker(&error.base, extended_marker(MARKER, NAME).as_str())
    }
}

impl fmt::Display for LoadAPIKeyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", NAME, self.message)
    }
}
impl Error for LoadAPIKeyError {}

impl From<LoadAPIKeyError> for AISDKError {
    fn from(error: LoadAPIKeyError) -> Self {
        AISDKError::new(
            NAME,
            format!("Failed to load API key: {}", error.message),
            Some(Box::new(error)),
        )
    }
}

#[cfg(test)]
mod tests_load_api_key_error {
    use super::*;

    #[test]
    fn test_constructor() {
        let message = String::from("API key not found");
        let load_api_key_error = LoadAPIKeyError::new(&message);

        assert_eq!(load_api_key_error.message, message);
        assert_eq!(
            format!("{}", load_api_key_error.base),
            format!("{}: Failed to load API key: {}", NAME, message)
        );
    }

    #[test]
    fn test_is_instance() {
        let message = String::from("API key not found");
        let load_api_key_error = LoadAPIKeyError::new(&message);

        assert!(LoadAPIKeyError::is_instance(&load_api_key_error));
    }

    #[test]
    fn test_display() {
        let message = String::from("API key not found");
        let load_api_key_error = LoadAPIKeyError::new(&message);

        assert_eq!(
            format!("{}", load_api_key_error),
            format!("{}: {}", NAME, message)
        );
    }

    #[test]
    fn test_api_key_error_to_aidsk_error() {
        let message = String::from("API key not found");
        let load_api_key_error = LoadAPIKeyError::new(&message);
        let aisdk_error: AISDKError = load_api_key_error.into();

        assert_eq!(
            format!("{}", aisdk_error),
            format!("{}: Failed to load API key: {}", NAME, message)
        );
    }
}