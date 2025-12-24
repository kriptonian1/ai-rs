use std::error::Error;

use crate::errors::{
    get_error_message::get_error_message,
    sdk_errors::{AISDKError, MARKER, extended_marker},
};

const NAME: &str = "AI_JSONParseError";

#[allow(dead_code)]
pub struct JSONParseError {
    text: String,
    base: AISDKError,
}

#[allow(dead_code)]
impl JSONParseError {
    pub fn new(text: &String, cause: Option<Box<dyn Error + Send + Sync + 'static>>) -> Self {
        let error_msg = get_error_message(Some(&cause));
        let message = format!("JSON parsing failed: Text: {text} \n Error message: {error_msg}");
        Self {
            text: text.clone(),
            base: AISDKError::new(NAME, message, cause),
        }
    }

    pub fn is_instance(error: &JSONParseError) -> bool {
        AISDKError::has_marker(&error.base, extended_marker(MARKER, NAME).as_str())
    }
}

#[cfg(test)]
mod tests_json_parser_error {
    use super::*;

    #[test]
    fn test_constructor() {
        let text = String::from("New Error");
        let json_parse_error = JSONParseError::new(&text, None);

        let error_msg = get_error_message(None);
        let message = format!("JSON parsing failed: Text: {text} \n Error message: {error_msg}");

        assert_eq!(json_parse_error.text, text);
        assert_eq!(
            format!("{}", json_parse_error.base),
            format!("{NAME}: {message}")
        )
    }

    #[test]
    fn test_is_instance() {
        let text = String::from("New Error");
        let json_parse_error = JSONParseError::new(&text, None);

        assert!(JSONParseError::is_instance(&json_parse_error))
    }
}
