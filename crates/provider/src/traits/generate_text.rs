use async_trait::async_trait;

use crate::errors::generate_text_error::GenerateTextError;

pub struct GenerateTextRequest {
    pub model: String,
    pub prompt: String,
}

pub struct GenerateTextResponse {
    pub text: String,
}

#[async_trait]
pub trait GenerateText {
    /// request AI model's API and returns the answer to the prompt
    async fn generate_text(
        &self,
        req: GenerateTextRequest,
    ) -> Result<GenerateTextResponse, GenerateTextError>;
}
