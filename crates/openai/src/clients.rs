use async_trait::async_trait;
use provider::{
    errors::generate_text_error::GenerateTextError,
    traits::generate_text::{GenerateText, GenerateTextRequest, GenerateTextResponse},
};
use serde_json::Value;

use crate::types::OpenAiRequest;

#[derive(Debug)]
pub struct OpenAiClient {
    api_key: String,
    client: reqwest::Client,
}

impl OpenAiClient {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            client: reqwest::Client::new(),
        }
    }
}

#[async_trait]
impl GenerateText for OpenAiClient {
    async fn generate_text(
        &self,
        req: GenerateTextRequest,
    ) -> Result<GenerateTextResponse, GenerateTextError> {
        let json_req = OpenAiRequest {
            input: req.prompt,
            model: req.model,
        };

        let resp = self
            .client
            .post("https://api.openai.com/v1/responses")
            .bearer_auth(&self.api_key)
            .json(&json_req)
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

        let text = body["output"]
            .get("output")
            .and_then(|v| v.as_array())
            .into_iter()
            .flat_map(|arr| arr.iter())
            .filter_map(|o| o.get("content").and_then(|c| c.as_array()))
            .flat_map(|arr| arr.iter())
            .filter_map(|c| c.get("text").and_then(|t| t.as_str()))
            .collect::<String>();

        Ok(GenerateTextResponse { text })
    }
}
