use std::{any::Any, io::Error};

use serde_json::Value;

pub fn get_error_message(error: Option<&dyn Any>) -> String {
    match error {
        Some(err) => {
            if let Some(s) = err.downcast_ref::<String>() {
                s.clone()
            } else if let Some(v) = err.downcast_ref::<Value>() {
                v.to_string()
            } else if let Some(e) = err.downcast_ref::<Error>() {
                e.to_string()
            } else {
                "unknown error".to_string()
            }
        }
        None => "unknown error".to_string(),
    }
}

#[cfg(test)]
mod tests_get_error_message {
    use serde_json::json;

    use super::*;

    #[test]
    fn test_none() {
        let error = get_error_message(None);
        assert_eq!(error, "unknown error")
    }

    #[test]
    fn test_string() {
        const CUSTOM_ERROR: &str = "Custom Error";
        let error = get_error_message(Some(&String::from(CUSTOM_ERROR)));

        assert_eq!(error, CUSTOM_ERROR)
    }

    #[test]
    fn test_serd_json() {
        let custome_error = json!({ "an": "object" });
        let error = get_error_message(Some(&custome_error));

        assert_eq!(error, custome_error.to_string())
    }

    #[test]
    fn test_error() {
        let custom_error_str = "Custom Error";
        let custom_error = std::io::Error::new(std::io::ErrorKind::Other, custom_error_str);
        let error = get_error_message(Some(&custom_error));

        assert_eq!(error, custom_error_str)
    }
}
