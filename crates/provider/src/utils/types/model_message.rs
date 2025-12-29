use super::{
    assistant_model_message::AssistantModelMessage,
    system_model_message::SystemModelMessage,
    tool_model_message::ToolModelMessage,
    user_model_message::UserModelMessage,
};

// Enum representing different types of model messages
#[derive(Debug)]
pub enum ModelMessage {
    System(SystemModelMessage),
    User(UserModelMessage),
    Assistant(AssistantModelMessage),
    Tool(ToolModelMessage),
}

// Expose as a type alias for easier usage
pub type ModelMessages = Vec<ModelMessage>;