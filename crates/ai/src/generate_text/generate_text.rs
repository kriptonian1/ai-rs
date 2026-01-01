use provider::{errors::generate_text_error::GenerateTextError, language_models::Models};
use reqwest::Client;
use serde_json::{Value, json};

async fn generate_text(
    models: Models,
    api_key: &str,
    prompt: &str,
) -> Result<String, GenerateTextError> {
    let get_model_version = models.to_model_version();

    let client = Client::new();

    let resp = client
        .post("https://api.openai.com/v1/responses")
        .bearer_auth(api_key)
        .json(&json!({
            "model": get_model_version,
            "input": prompt
        }))
        .send()
        .await
        .map_err(GenerateTextError::Transport)?;

    let status_code = resp.status();

    let body: Value = resp.json().await.map_err(GenerateTextError::Transport)?;

    if !status_code.is_success() {
        let message = body
            .get("error")
            .and_then(|e| e.get("message"))
            .and_then(|m| m.as_str())
            .unwrap_or("unknown error")
            .to_string();

        return Err(GenerateTextError::Api {
            status_code,
            message,
        });
    }

    println!("body: {:?}", body);

    let text = body["output"]
        .get("output")
        .and_then(|v| v.as_array())
        .into_iter()
        .flat_map(|arr| arr.iter())
        .filter_map(|o| o.get("content").and_then(|c| c.as_array()))
        .flat_map(|arr| arr.iter())
        .filter_map(|c| c.get("text").and_then(|t| t.as_str()))
        .collect::<String>();

    Ok(text)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_name() {
        let api_key = "OPENAI_KEY";

        let text = generate_text(
            Models::OpenAi(provider::language_models::OpenAiModel::Gpt4_1),
            api_key,
            "Hey, how are you ?",
        )
        .await;

        match text {
            Ok(val) => println!("{}", val),
            Err(err) => println!("{}", err),
        }
    }
}
