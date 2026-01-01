use async_trait::async_trait;

use crate::{errors::generate_text_error::GenerateTextError, language_models::Models};

pub struct GenerateTextRequest {
    pub model: String,
    pub prompt: String,
}

pub struct GenerateTextResponse {
    pub text: String,
}

#[async_trait]
pub trait GenerateText {
    async fn generate_text(
        &self,
        req: GenerateTextRequest,
    ) -> Result<GenerateTextResponse, GenerateTextError>;
}
