use std::fmt;

#[derive(Debug)]
pub enum ClientError {
    EmptyAPIKey,
    InvalidAPIKey,
}

impl fmt::Display for ClientError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ClientError::EmptyAPIKey => write!(f, "API key is missing"),
            ClientError::InvalidAPIKey => write!(f, "API key format is invalid"),
        }
    }
}
