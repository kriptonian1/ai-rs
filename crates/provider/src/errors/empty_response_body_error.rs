use std::{error::Error, fmt};

use crate::errors::sdk_errors::{AISDKError, MARKER, extended_marker};

const NAME: &str = "AI_EmptyResponseBodyError";

#[allow(dead_code)]
#[derive(Debug)]
pub struct EmptyResponseBodyError {
    pub base: AISDKError,
}

#[allow(dead_code)]
impl EmptyResponseBodyError {
    pub fn new(message: Option<String>) -> Self {
        let message = message.unwrap_or_else(|| "Empty response body".to_string());

        Self {
            base: AISDKError::new(NAME, message, None),
        }
    }

    pub fn is_instance(error: &EmptyResponseBodyError) -> bool {
        AISDKError::has_marker(&error.base, extended_marker(MARKER, NAME).as_str())
    }
}

impl fmt::Display for EmptyResponseBodyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.base)
    }
}

impl Error for EmptyResponseBodyError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.base.source()
    }
}

#[cfg(test)]
mod test_empty_response_body {

    use super::*;

    #[test]
    fn empty_response_body_error_constructor() {
        let message = "New Error";
        let empty_response_body_error = EmptyResponseBodyError::new(Some(message.to_string()));

        assert_eq!(
            format!("{}", empty_response_body_error),
            format!("{NAME}: {message}")
        )
    }

    #[test]
    fn test_is_instance() {
        let message = "New Error";
        let empty_response_body_error = EmptyResponseBodyError::new(Some(message.to_string()));

        assert_eq!(
            EmptyResponseBodyError::is_instance(&empty_response_body_error),
            true
        );
    }
}
