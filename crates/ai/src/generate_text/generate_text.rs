use provider::{
    errors::generate_text_error::GenerateTextError,
    language_models::Models,
    traits::generate_text::{GenerateText, GenerateTextRequest},
};

/// Generate text for a given model using the provided prompt, this dose not stream output
///
/// # Example
/// ```
/// let api_key = std::env::var("OPENAI_API_KEY").unwrap();
/// if let Ok(openai) = OpenAiClient::new(api_key) {
///   let text = generate_text(
///        &openai,
///        Models::OpenAi(provider::language_models::OpenAiModel::Gpt4_1),
///        "Hey, How are you ?",
///    )
///    .await;
///
///    match text {
///        Ok(val) => println!("AI: {}", val),
///        Err(err) => println!("Error: {}", err),
///    }
/// }
/// ```
pub async fn generate_text(
    provider: &dyn GenerateText,
    models: Models,
    prompt: &str,
) -> Result<String, GenerateTextError> {
    let resp = provider
        .generate_text(GenerateTextRequest {
            model: models.to_model_version(),
            prompt: prompt.to_string(),
        })
        .await?;
    Ok(resp.text)
}

#[cfg(test)]
mod tests {
    use openai::clients::OpenAiClient;

    use super::*;

    #[tokio::test]
    async fn test_generate_text_openai() {
        dotenvy::dotenv().ok();
        let api_key = std::env::var("OPEN_API_KEY").unwrap();

        if let Ok(openai) = OpenAiClient::new(api_key) {
            let text = generate_text(
                &openai,
                Models::OpenAi(provider::language_models::OpenAiModel::Gpt4_1),
                "Hey, How are you ?",
            )
            .await;

            match text {
                Ok(val) => println!("{}", val),
                Err(err) => println!("{}", err),
            }
        }
    }
    #[test]
    fn test_generate_text_empty_error() {
        let api_key = "   ".to_string();
        let client = OpenAiClient::new(api_key);
        match client {
            Err(err) => assert_eq!("API key is missing", err.to_string()),
            _ => panic!("Should return error"),
        }
    }

    #[test]
    fn test_generate_text_invalid_error() {
        let api_key = "abcd".to_string();
        let client = OpenAiClient::new(api_key);
        match client {
            Err(err) => assert_eq!("API key format is invalid", err.to_string()),
            _ => panic!("Should return error"),
        }
    }
}
