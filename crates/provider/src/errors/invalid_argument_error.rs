use super::sdk_errors::{
    AISDKError,
    MARKER,
    extended_marker
};
use std::error::Error;
use std::fmt;


const NAME: &str = "AI_InvalidArgumentError";

#[derive(Debug)]
pub struct InvalidArgumentError {
    pub argument: String,
    pub message: String,
    pub cause: Option<Box<dyn Error + Send + Sync + 'static>>,
}

impl InvalidArgumentError {
    pub fn new(
        argument: String,
        message: String,
        cause: Option<Box<dyn Error + Send + Sync + 'static>>,
    ) -> Self {
        Self {
            argument,
            message,
            cause,
        }
    }

    pub fn is_instance(error: &(dyn Error + 'static)) -> bool {
        Self::has_marker(error, extended_marker(MARKER, NAME).as_str())
    }

    fn has_marker(error: &(dyn Error + 'static), marker: &str) -> bool {
        error.downcast_ref::<InvalidArgumentError>().is_some()
            || AISDKError::has_marker(error, marker)
    }
}

impl fmt::Display for InvalidArgumentError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.argument, self.message)
    }
}

impl Error for InvalidArgumentError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.cause.as_ref().map(|e| &**e as &(dyn Error + 'static))
    }
}


impl From<InvalidArgumentError> for AISDKError {
    fn from(error: InvalidArgumentError) -> Self {
        AISDKError::new(
            NAME,
            format!("Invalid argument '{}': {}", error.argument, error.message),
            Some(Box::new(error)),
        )
    }
}

#[cfg(test)]
mod test_invalid_argument_error {
    use super::*;
    pub(super) use super::super::sdk_errors::AISDKError;

    #[test]
    fn test_invalid_argument_error_creation() {
        let arg_error = InvalidArgumentError::new(
            "param1".to_string(),
            "must be a positive integer".to_string(),
            None,
        );

        assert_eq!(arg_error.argument, "param1");
        assert_eq!(arg_error.message, "must be a positive integer");
        assert!(arg_error.cause.is_none());
    }

    #[test]
    fn test_invalid_argument_error_display() {
        let arg_error = InvalidArgumentError::new(
            "param1".to_string(),
            "must be a positive integer".to_string(),
            None,
        );
        assert_eq!(
            format!("{}", arg_error),
            "param1: must be a positive integer"
        );
    }

    #[test]
    fn test_invalid_argument_error_source() {
        let cause = std::io::Error::new(std::io::ErrorKind::Other, "io error");
        let arg_error = InvalidArgumentError::new(
            "param1".to_string(),
            "must be a positive integer".to_string(),
            Some(Box::new(cause)),
        );
        let source = arg_error.source();
        assert!(source.is_some());
        assert_eq!(source.unwrap().to_string(), "io error");
    }

    #[test]
    fn test_invalid_argument_error_to_aisdk_error() {
        let arg_error = InvalidArgumentError::new(
            "param1".to_string(),
            "must be a positive integer".to_string(),
            None,
        );
        let aisdk_error: AISDKError = arg_error.into();
        assert_eq!(aisdk_error.name, NAME);
        assert_eq!(
            aisdk_error.message,
            "Invalid argument 'param1': must be a positive integer"
        );
    }

    #[test]
    fn test_is_instance() {
        let arg_error = InvalidArgumentError::new(
            "param1".to_string(),
            "must be a positive integer".to_string(),
            None,
        );
        assert!(InvalidArgumentError::is_instance(&arg_error));

        let aisdk_error: AISDKError = arg_error.into();
        assert!(InvalidArgumentError::is_instance(&aisdk_error));
    }
}