use std::error::Error;
use std::fmt;

/// Symbol used for identifying AI SDK Error instances.
/// Enables checking if an error is an instance of AISDKError across package versions.
pub(crate) const MARKER: &str = "aisdk.error";

#[inline(always)]
pub(crate) fn extended_marker(base: &str, extension: &str) -> String {
    format!("{}_{}", base, extension)
}

/// Custom error struct for AI SDK related errors.
#[derive(Debug)]
pub(super) struct AISDKError {
    name: String,
    message: String,
    cause: Option<Box<dyn Error + Send + Sync + 'static>>,
}

impl AISDKError {
    /// Creates an AI SDK Error.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the error.
    /// * `message` - The error message.
    /// * `cause` - The underlying cause of the error, if any.
    pub fn new(
        name: impl Into<String>,
        message: impl Into<String>,
        cause: Option<Box<dyn Error + Send + Sync + 'static>>,
    ) -> Self {
        Self {
            name: name.into(),
            message: message.into(),
            cause,
        }
    }

    /// Checks if the given error is an AI SDK Error.
    ///
    /// # Arguments
    ///
    /// * `error` - The error to check.
    ///
    /// # Returns
    ///
    /// `true` if the error is an AI SDK Error, `false` otherwise.
    pub fn is_instance(error: &(dyn Error + 'static)) -> bool {
        Self::has_marker(error, MARKER)
    }

    pub fn has_marker(error: &(dyn Error + 'static), _marker: &str) -> bool {
        error.downcast_ref::<AISDKError>().is_some()
    }
}

impl fmt::Display for AISDKError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.name, self.message)
    }
}

impl Error for AISDKError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.cause.as_ref().map(|e| &**e as &(dyn Error + 'static))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io;

    #[test]
    fn test_create_constructor() {
        let error = AISDKError::new("TestError", "An error occurred", None);
        assert_eq!(error.name, "TestError");
        assert_eq!(error.message, "An error occurred");
        assert!(error.cause.is_none());
    }
    #[test]
    fn test_new_without_cause() {
        let error = AISDKError::new("TestError", "Something went wrong", None);
        assert_eq!(error.name, "TestError");
        assert_eq!(error.message, "Something went wrong");
        assert!(error.cause.is_none());
    }

    #[test]
    fn test_new_with_cause() {
        let cause = io::Error::new(io::ErrorKind::Other, "io error");
        let error = AISDKError::new("TestError", "Something went wrong", Some(Box::new(cause)));
        assert_eq!(error.name, "TestError");
        assert_eq!(error.message, "Something went wrong");
        assert!(error.cause.is_some());
    }

    #[test]
    fn test_display() {
        let error = AISDKError::new("TestError", "Something went wrong", None);
        assert_eq!(format!("{}", error), "TestError: Something went wrong");
    }

    #[test]
    fn test_source() {
        let cause = io::Error::new(io::ErrorKind::Other, "io error");
        let error = AISDKError::new("TestError", "Something went wrong", Some(Box::new(cause)));

        let source = error.source();
        assert!(source.is_some());
        assert_eq!(source.unwrap().to_string(), "io error");
    }

    #[test]
    fn test_is_instance() {
        let sdk_error = AISDKError::new("TestError", "Something went wrong", None);
        assert!(AISDKError::is_instance(&sdk_error));

        let io_error = io::Error::new(io::ErrorKind::Other, "io error");
        assert_eq!(AISDKError::is_instance(&io_error), false);
    }
}
