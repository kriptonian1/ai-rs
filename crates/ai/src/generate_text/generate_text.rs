use provider::{
    errors::generate_text_error::GenerateTextError,
    language_models::Models,
    traits::generate_text::{GenerateText, GenerateTextRequest},
};

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

        let openai = OpenAiClient::new(api_key.to_string());

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
