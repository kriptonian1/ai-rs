use serde::{Deserialize, Serialize};
use super::{
    content_part::{
        DataOrURL,
        FilePart,
        ImagePart,
        TextPart,
    },
    provider_options::ProviderOptions,
};


#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum UserContent {
    String(String),
    Parts(Vec<UserContentPart>),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum UserContentPart {
    TextPart(TextPart),
    ImagePart(ImagePart),
    FilePart(FilePart),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserModelMessage {
    pub role: String,
    pub content: UserContent,
    #[serde(rename = "providerOptions", skip_serializing_if = "Option::is_none")]
    pub provider_options: Option<ProviderOptions>,
}
