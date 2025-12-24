use super::sdk_errors::{
    AISDKError,
    MARKER,
    extended_marker
};
use std::error::Error;
use std::fmt;

const NAME: &str = "AI_UnsupportedFunctionalityError";

#[derive(Debug)]
pub struct UnsupportedFunctionalityError {
    message: String,
    base: AISDKError,
}

impl UnsupportedFunctionalityError {
    pub fn new(
        message: &String,
    ) -> Self {
        Self {
            message: message.clone(),
            base: AISDKError::new(NAME, format!("{} functionality not supported", message), None),
        }
    }

    pub fn is_instance(error: &UnsupportedFunctionalityError) -> bool {
        AISDKError::has_marker(&error.base, extended_marker(MARKER, NAME).as_str())
    }
}

impl fmt::Display for UnsupportedFunctionalityError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", NAME, self.message)
    }
}
impl Error for UnsupportedFunctionalityError {}

impl From<UnsupportedFunctionalityError> for AISDKError {
    fn from(error: UnsupportedFunctionalityError) -> Self {
        AISDKError::new(
            NAME,
            format!("{} functionality not supported", error.message),
            Some(Box::new(error)),
        )
    }
}

#[cfg(test)]
mod tests_unsupported_functionality_error {
    use super::*;

    #[test]
    fn test_constructor() {
        let message = String::from("This");
        let unsupported_functionality_error = UnsupportedFunctionalityError::new(&message);

        assert_eq!(unsupported_functionality_error.message, message);
        assert_eq!(
            format!("{}", unsupported_functionality_error.base),
            format!("{}: {} functionality not supported", NAME, message)
        );
    }

    #[test]
    fn test_is_instance() {
        let message = String::from("This");
        let unsupported_functionality_error = UnsupportedFunctionalityError::new(&message);

        assert!(UnsupportedFunctionalityError::is_instance(&unsupported_functionality_error));
    }

    #[test]
    fn test_display() {
        let message = String::from("This");
        let unsupported_functionality_error = UnsupportedFunctionalityError::new(&message);

        assert_eq!(
            format!("{}", unsupported_functionality_error),
            format!("{}: {}", NAME, message)
        );
    }

    #[test]
    fn test_unsupported_functionality_to_aisdk_error() {
        let message = String::from("This");
        let unsupported_functionality_error = UnsupportedFunctionalityError::new(&message);
        let aisdk_error: AISDKError = unsupported_functionality_error.into();

        assert_eq!(
            format!("{}", aisdk_error),
            format!("{}: {} functionality not supported", NAME, message)
        );
    }
}