use std::any::Any;
use super::sdk_errors::{
    AISDKError,
    MARKER,
    extended_marker
};
use std::error::Error;
use std::fmt;

const NAME: &str = "AI_InvalidPromptError";

#[derive(Debug)]
pub struct InvalidPromptError<'a> {
    #[allow(dead_code)]
    pub prompt: Option<&'a dyn Any>,
    pub message: String,
    pub cause: Option<Box<dyn Error + Send + Sync + 'static>>,
}

#[allow(dead_code)]
impl<'a> InvalidPromptError<'a> {
    pub fn new(
        prompt: Option<&'a dyn Any>,
        message: String,
        cause: Option<Box<dyn Error + Send + Sync + 'static>>,
    ) -> Self {
        Self {
            prompt,
            message,
            cause,
        }
    }

    pub fn is_instance(error: &(dyn Error + 'static)) -> bool {
        Self::has_marker(error, extended_marker(MARKER, NAME).as_str())
    }

    fn has_marker(error: &(dyn Error + 'static), marker: &str) -> bool {
        error.downcast_ref::<InvalidPromptError>().is_some()
            || AISDKError::has_marker(error, marker)
    }
}

impl<'a> fmt::Display for InvalidPromptError<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Invalid prompt: {}", self.message)
    }
}

impl<'a> Error for InvalidPromptError<'a> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.cause.as_ref().map(|e| &**e as &(dyn Error + 'static))
    }
}

impl<'a> From<InvalidPromptError<'a>> for AISDKError {
    fn from(error: InvalidPromptError<'a>) -> Self {
        AISDKError::new(
            NAME,
            format!("Invalid prompt: {}", error.message),
            error.cause,
        )
    }
}

#[cfg(test)]
mod tests_invalid_prompt_error {
    use super::*;
    pub(super) use super::super::sdk_errors::AISDKError;

    #[test]
    fn test_constructor() {
        let prompt: Option<&dyn Any> = None;
        let message = String::from("Invalid Prompt Error");
        let invalid_prompt_error = InvalidPromptError::new(prompt, message.clone(), None);

        assert!(invalid_prompt_error.prompt.is_none());
        assert_eq!(invalid_prompt_error.message, message);
    }

    #[test]
    fn test_invalid_prompt_error_creation_with_aisdkerror_couples(){
        let prompt: Option<&dyn Any> = None;
        let message = String::from("Invalid Prompt Error");
        let invalid_prompt_error = InvalidPromptError::new(prompt, message.clone(), None);
        let aisdk_error: AISDKError = invalid_prompt_error.into();

        assert_eq!(aisdk_error.name, NAME);
        assert_eq!(aisdk_error.message, format!("Invalid prompt: {}", message));
    }

    #[test]
    fn test_invalid_prompt_error_display() {
        let prompt: Option<&dyn Any> = None;
        let message = String::from("Invalid Prompt Error");
        let invalid_prompt_error = InvalidPromptError::new(prompt, message.clone(), None);

        assert_eq!(
            format!("{}", invalid_prompt_error),
            format!("Invalid prompt: {}", message)
        );
    }

    #[test]
    fn test_invalid_prompt_error_source() {
        let prompt: Option<&dyn Any> = None;
        let message = String::from("Invalid Prompt Error");
        let cause = std::io::Error::new(std::io::ErrorKind::Other, "io error");
        let invalid_prompt_error = InvalidPromptError::new(
            prompt,
            message,
            Some(Box::new(cause)),
        );

        assert!(invalid_prompt_error.source().is_some());
    }

    #[test]
    fn test_invalid_prompt_error_source_none() {
        let prompt: Option<&dyn Any> = None;
        let message = String::from("Invalid Prompt Error");
        let invalid_prompt_error = InvalidPromptError::new(prompt, message, None);

        assert!(invalid_prompt_error.source().is_none());
    }

    #[test]
    fn test_invalid_prompt_error_with_cause() {
        let prompt: Option<&dyn Any> = None;
        let message = String::from("Invalid Prompt Error");
        let cause = std::io::Error::new(std::io::ErrorKind::Other, "io error");
        let invalid_prompt_error = InvalidPromptError::new(
            prompt,
            message,
            Some(Box::new(cause)),
        );

        assert!(invalid_prompt_error.cause.is_some());
    }

    #[test]
    fn test_invalid_prompt_error_without_cause() {
        let prompt: Option<&dyn Any> = None;
        let message = String::from("Invalid Prompt Error");
        let invalid_prompt_error = InvalidPromptError::new(prompt, message, None);

        assert!(invalid_prompt_error.cause.is_none());
    }

    #[test]
    fn test_invalid_prompt_error_some_prompt_value() {
        let sample_prompt = "Sample Prompt";
        let prompt: Option<&dyn Any> = Some(&sample_prompt);
        let message = String::from("Invalid Prompt Error");
        let invalid_prompt_error = InvalidPromptError::new(prompt, message, None);

        assert!(invalid_prompt_error.prompt.is_some());
        // print!("{}", invalid_prompt_error.prompt.unwrap().downcast_ref::<&str>().unwrap()); // for debugging
        assert_eq!(
            invalid_prompt_error.prompt.unwrap().downcast_ref::<&str>(),
            Some(&sample_prompt)
        );
    }

    #[test]
    fn test_is_instance() {
        let prompt: Option<&dyn Any> = None;
        let message = String::from("Invalid Prompt Error");
        let invalid_prompt_error = InvalidPromptError::new(prompt, message, None);

        assert!(InvalidPromptError::is_instance(&invalid_prompt_error));
    }
}