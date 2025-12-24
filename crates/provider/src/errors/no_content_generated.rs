use super::sdk_errors::{
    AISDKError,
    MARKER,
    extended_marker
};
use std::error::Error;
use std::fmt;

const NAME: &str = "AI_NoContentGeneratedError";

#[derive(Debug)]
pub struct NoContentGeneratedError {
    message: String,
    base: AISDKError,
}

impl NoContentGeneratedError {
    pub fn new(
        message: &String,
    ) -> Self {
        Self {
            message: message.clone(),
            base: AISDKError::new(NAME, format!("No Content Generated: {}", message), None),
        }
    }

    pub fn is_instance(error: &NoContentGeneratedError) -> bool {
        AISDKError::has_marker(&error.base, extended_marker(MARKER, NAME).as_str())
    }
}

impl fmt::Display for NoContentGeneratedError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", NAME, self.message)
    }
}
impl Error for NoContentGeneratedError {}

impl From<NoContentGeneratedError> for AISDKError {
    fn from(error: NoContentGeneratedError) -> Self {
        AISDKError::new(
            NAME,
            format!("No Content Generated: {}", error.message),
            Some(Box::new(error)),
        )
    }
}

#[cfg(test)]
mod test_no_content_generated_error {
    use super::*;

    #[test]
    fn test_constructor() {
        let message = String::from("No Content Generated");
        let load_no_content_generated_error = NoContentGeneratedError::new(&message);

        assert_eq!(load_no_content_generated_error.message, message);
        assert_eq!(
            format!("{}", load_no_content_generated_error.base),
            format!("{}: No Content Generated: {}", NAME, message)
        );
    }

    #[test]
    fn test_is_instance() {
        let message = String::from("No Content Generated");
        let load_no_content_generated_error = NoContentGeneratedError::new(&message);

        assert!(NoContentGeneratedError::is_instance(&load_no_content_generated_error));
    }

    #[test]
    fn test_display() {
        let message = String::from("No Content Generated");
        let load_api_key_error = NoContentGeneratedError::new(&message);

        assert_eq!(
            format!("{}", load_api_key_error),
            format!("{}: {}", NAME, message)
        );
    }

    #[test]
    fn test_load_api_key_error_to_aisdk_error() {
        let message = String::from("No Content Generated");
        let load_api_key_error = NoContentGeneratedError::new(&message);
        let aisdk_error: AISDKError = load_api_key_error.into();

        assert_eq!(
            format!("{}", aisdk_error),
            format!("{}: No Content Generated: {}", NAME, message)
        );
    }
}