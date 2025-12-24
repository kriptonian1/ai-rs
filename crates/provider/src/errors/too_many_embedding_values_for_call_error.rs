use std::any::Any;

use crate::errors::sdk_errors::{AISDKError, MARKER, extended_marker};

const NAME: &str = "AI_TooManyEmbeddingValuesForCallError";

#[derive(Debug)]
struct TooManyEmbeddingValuesForCallError {
    provider: String,
    model_id: String,
    max_embedding_per_call: u64,
    values: Vec<Box<dyn Any>>,
    base: AISDKError,
}

impl TooManyEmbeddingValuesForCallError {
    fn new(
        provider: String,
        model_id: String,
        max_embedding_per_call: u64,
        values: Vec<Box<dyn Any>>,
    ) -> Self {
        let values_len = values.len();
        let message = format!(
            "Too many values for a single embedding call. The {provider} model {model_id} can only embed up to {max_embedding_per_call} values per call, but {values_len} values were provided."
        );
        Self {
            provider,
            model_id,
            max_embedding_per_call,
            values,
            base: AISDKError::new(NAME, message, None),
        }
    }

    pub fn is_instance(error: &TooManyEmbeddingValuesForCallError) -> bool {
        AISDKError::has_marker(&error.base, extended_marker(MARKER, NAME).as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constructor() {
        let provider = "OpenAI";
        let model_id = "OpenAI:5.1";
        let max_embedding_per_call = 24;
        let values: Vec<Box<dyn Any>> = vec![Box::new(1), Box::new(2), Box::new(3)];

        let aisdk_error_msg = "Too many values for a single embedding call. The OpenAI model OpenAI:5.1 can only embed up to 24 values per call, but 3 values were provided.";

        let too_many_embedding_values_for_call_error = TooManyEmbeddingValuesForCallError::new(
            provider.to_string(),
            model_id.to_string(),
            max_embedding_per_call,
            values,
        );

        assert_eq!(too_many_embedding_values_for_call_error.provider, provider);
        assert_eq!(too_many_embedding_values_for_call_error.model_id, model_id);
        assert_eq!(
            too_many_embedding_values_for_call_error.max_embedding_per_call,
            max_embedding_per_call
        );

        assert_eq!(
            too_many_embedding_values_for_call_error.base.message,
            aisdk_error_msg
        );

        assert_eq!(too_many_embedding_values_for_call_error.base.name, NAME)
    }

    #[test]
    fn test_is_instance() {
        let provider = "OpenAI";
        let model_id = "OpenAI:5.1";
        let max_embedding_per_call = 24;
        let values: Vec<Box<dyn Any>> = vec![Box::new(1), Box::new(2), Box::new(3)];

        let too_many_embedding_values_for_call_error = TooManyEmbeddingValuesForCallError::new(
            provider.to_string(),
            model_id.to_string(),
            max_embedding_per_call,
            values,
        );

        assert!(TooManyEmbeddingValuesForCallError::is_instance(
            &too_many_embedding_values_for_call_error
        ))
    }
}
