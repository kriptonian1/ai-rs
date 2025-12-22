use std::error::Error;
use std::fmt;

/// Symbol used for identifying AI SDK Error instances.
/// Enables checking if an error is an instance of AISDKError across package versions.
const MARKER: &str = "aisdk.error";

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

    fn has_marker(error: &(dyn Error + 'static), _marker: &str) -> bool {
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
        self.cause
            .as_ref()
            .map(|e| &**e as &(dyn Error + 'static))
    }
}
