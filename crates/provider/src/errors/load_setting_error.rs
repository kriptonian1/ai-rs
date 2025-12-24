use super::sdk_errors::{
    AISDKError,
    MARKER,
    extended_marker
};
use std::error::Error;
use std::fmt;

const NAME: &str = "AI_LoadSettingError";

#[derive(Debug)]
pub struct LoadSettingError {
    message: String,
    base: AISDKError,
}

impl LoadSettingError {
    pub fn new(
        message: &String,
    ) -> Self {
        Self {
            message: message.clone(),
            base: AISDKError::new(NAME, message.clone(), None),
        }
    }

    pub fn is_instance(error: &LoadSettingError) -> bool {
        AISDKError::has_marker(&error.base, extended_marker(MARKER, NAME).as_str())
    }
}

impl fmt::Display for LoadSettingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", NAME, self.message)
    }
}
impl Error for LoadSettingError {}

impl From<LoadSettingError> for AISDKError {
    fn from(error: LoadSettingError) -> Self {
        AISDKError::new(
            NAME,
            error.message.clone(),
            Some(Box::new(error)),
        )
    }
}

#[cfg(test)]
mod tests_load_setting_error {
    use super::*;

    #[test]
    fn test_constructor() {
        let message = String::from("API key not found");
        let load_setting_error = LoadSettingError::new(&message);

        assert_eq!(load_setting_error.message, message);
        assert_eq!(
            format!("{}", load_setting_error.base),
            format!("{}: {}", NAME, message)
        );
    }

    #[test]
    fn test_is_instance() {
        let message = String::from("API key not found");
        let load_setting_error = LoadSettingError::new(&message);

        assert!(LoadSettingError::is_instance(&load_setting_error));
    }

    #[test]
    fn test_display() {
        let message = String::from("API key not found");
        let load_setting_error = LoadSettingError::new(&message);

        assert_eq!(
            format!("{}", load_setting_error),
            format!("{}: {}", NAME, message)
        );
    }

    #[test]
    fn test_load_setting_error_to_aisdk_error() {
        let message = String::from("API key not found");
        let load_setting_error = LoadSettingError::new(&message);
        let aisdk_error: AISDKError = load_setting_error.into();

        assert_eq!(
            format!("{}", aisdk_error),
            format!("{}: {}", NAME, message)
        );
    }
}