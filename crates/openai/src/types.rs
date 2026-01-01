use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct OpenAiRequest {
    pub model: String,
    pub input: String,
}

// #[derive(Deserialize)]
// struct OpenAIResponse {
//     output: Vec<OpenAIOutput>,
// }
