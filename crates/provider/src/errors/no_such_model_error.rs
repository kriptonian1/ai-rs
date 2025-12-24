use std::fmt;

use serde::{Deserialize, Serialize};

use crate::errors::sdk_errors::{AISDKError, MARKER, extended_marker};

const NAME: &str = "AI_NoSuchModelError";

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub enum ModelType {
    LanguageModel,
    EmbeddingModel,
    ImageModel,
    TranscriptionModel,
    SpeechModel,
    RerankingModel,
}

impl fmt::Display for ModelType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ModelType::LanguageModel => write!(f, "languageModel"),
            ModelType::EmbeddingModel => write!(f, "embeddingModel"),
            ModelType::ImageModel => write!(f, "imageModel"),
            ModelType::TranscriptionModel => write!(f, "transcriptionModel"),
            ModelType::SpeechModel => write!(f, "speechModel"),
            ModelType::RerankingModel => write!(f, "rerankingModel"),
        }
    }
}

struct NoSuchModelError {
    model_id: String,
    model_type: ModelType,
    base: AISDKError,
}

impl NoSuchModelError {
    fn new(
        error: Option<String>,
        model_id: String,
        model_type: ModelType,
        message: Option<String>,
    ) -> Self {
        let error = match error {
            Some(e) => e,
            None => String::from(NAME),
        };

        let message = match message {
            Some(m) => m,
            None => String::from(format!("No such ${model_type}: ${model_id}")),
        };

        Self {
            model_id,
            model_type,
            base: AISDKError::new(error, message, None),
        }
    }

    pub fn is_instance(error: &NoSuchModelError) -> bool {
        AISDKError::has_marker(&error.base, extended_marker(MARKER, NAME).as_str())
    }
}

#[cfg(test)]
mod tests_no_such_model_error {
    use super::*;

    #[test]
    fn test_constructor() {
        let error = "Custom Error";
        let model_id = "OpenAI:5.1";
        let model_type = ModelType::LanguageModel;
        let message = None;
        let no_such_model_error = NoSuchModelError::new(
            Some(error.to_string()),
            model_id.to_string(),
            model_type,
            message,
        );

        assert_eq!(no_such_model_error.model_id, model_id);
        assert_eq!(
            format!("{}", no_such_model_error.model_type),
            format!("{}", model_type)
        );
        assert_eq!(no_such_model_error.base.name, error);
        assert_eq!(
            no_such_model_error.base.message,
            format!("No such ${model_type}: ${model_id}")
        )
    }

    #[test]
    fn test_is_instance() {
        let error = "Custom Error";
        let model_id = "OpenAI:5.1";
        let model_type = ModelType::LanguageModel;
        let message = None;
        let no_such_model_error = NoSuchModelError::new(
            Some(error.to_string()),
            model_id.to_string(),
            model_type,
            message,
        );

        assert!(NoSuchModelError::is_instance(&no_such_model_error))
    }
}
