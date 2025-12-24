use std::error::Error;
use std::fmt;

use crate::errors::{
    get_error_message::get_error_message,
    sdk_errors::{AISDKError, MARKER, extended_marker},
};

/// Marker for TypeValidationError
const NAME: &str = "AI_TypeValidationError";


/// Error type for type validation failures
/// Holds the invalid value and the underlying cause
/// if any.
#[allow(dead_code)]
#[derive(Debug)]
pub struct TypeValidationError {
    value: String,
    base: AISDKError,
}


/// Implementation of TypeValidationError
/// Includes constructor, instance checker, and error wrapping functionality.
/// Also implements Display and Error traits.
/// # Arguments
/// * `value` - The value that failed type validation
/// * `cause` - The original error that caused the type validation to fail
/// # Returns
/// A TypeValidationError instance
#[allow(dead_code)]
impl TypeValidationError {
    pub fn new(value: &String, cause: Option<Box<dyn Error + Send + Sync + 'static>>) -> Self {
        let error_msg = get_error_message(Some(&cause));
        let message = format!("Type validation failed: Value: {value} \n Error message: {error_msg}");
        Self {
            value: value.clone(),
            base: AISDKError::new(NAME, message, cause),
        }
    }

    pub fn is_instance(error: &TypeValidationError) -> bool {
        AISDKError::has_marker(&error.base, extended_marker(MARKER, NAME).as_str())
    }

    /// Wraps an error into a TypeValidationError.
    /// If the cause is already a TypeValidationError with the same value, it returns the cause.
    /// Otherwise, it creates a new TypeValidationError.
    /// # Arguments
    /// * `value` - The value that failed type validation
    /// * `cause` - The original error that caused the type validation to fail
    /// # Returns
    /// A TypeValidationError instance
    pub fn wrap(
        value: &String,
        cause: Option<Box<dyn Error + Send + Sync + 'static>>,
    ) -> TypeValidationError {
        if let Some(boxed_cause) = cause {
            match boxed_cause.downcast::<TypeValidationError>() {
                Ok(type_val_err) => {
                    if &type_val_err.value == value {
                        *type_val_err
                    } else {
                        TypeValidationError::new(value, Some(type_val_err))
                    }
                }
                Err(original_cause) => TypeValidationError::new(value, Some(original_cause)),
            }
        } else {
            TypeValidationError::new(value, None)
        }
    }
}

impl fmt::Display for TypeValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.base)
    }
}

impl Error for TypeValidationError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.base.source()
    }
}


#[cfg(test)]
mod tests_type_validation_error {
    use super::*;

    #[test]
    fn test_constructor() {
        let value = String::from("Invalid Value");
        let type_validation_error = TypeValidationError::new(&value, None);

        let error_msg = get_error_message(None);
        let message = format!("Type validation failed: Value: {value} \n Error message: {error_msg}");

        assert_eq!(type_validation_error.value, value);
        assert_eq!(
            format!("{}", type_validation_error.base),
            format!("{NAME}: {message}")
        )
    }

    #[test]
    fn test_is_instance() {
        let value = String::from("Invalid Value");
        let type_validation_error = TypeValidationError::new(&value, None);

        assert!(TypeValidationError::is_instance(&type_validation_error));
    }

    #[test]
    fn test_wrap_existing_error() {
        let value = String::from("Invalid Value");
        let original_error = TypeValidationError::new(&value, None);

        let wrapped_error = TypeValidationError::wrap(&value, Some(Box::new(original_error)));

        assert_eq!(wrapped_error.value, value);
    }

    #[test]
    fn test_wrap_new_error() {
        let value = String::from("Invalid Value");
        let different_value = String::from("Different Invalid Value");
        let original_error = TypeValidationError::new(&different_value, None);

        let wrapped_error = TypeValidationError::wrap(&value, Some(Box::new(original_error)));

        assert_eq!(wrapped_error.value, value);
    }

    #[test]
    fn test_wrap_no_cause() {
        let value = String::from("Invalid Value");

        let wrapped_error = TypeValidationError::wrap(&value, None);

        assert_eq!(wrapped_error.value, value);
    }

    #[test]
    fn test_display_trait() {
        let value = String::from("Invalid Value");
        let type_validation_error = TypeValidationError::new(&value, None);

        let error_msg = get_error_message(None);
        let message = format!("Type validation failed: Value: {value} \n Error message: {error_msg}");

        assert_eq!(
            format!("{}", type_validation_error),
            format!("{NAME}: {message}")
        );
    }

    #[test]
    fn test_error_trait_source() {
        let value = String::from("Invalid Value");
        let type_validation_error = TypeValidationError::new(&value, None);

        assert!(type_validation_error.source().is_none());
    }
}