use std::fmt;

use reqwest::StatusCode;

#[derive(Debug)]
pub enum GenerateTextError {
    Transport(reqwest::Error),
    Api {
        status_code: StatusCode,
        message: String,
    },
}

impl fmt::Display for GenerateTextError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GenerateTextError::Transport(err) => write!(f, "network error: {}", err),
            GenerateTextError::Api {
                status_code,
                message,
            } => {
                write!(f, "API error [{}]: {}", status_code, message)
            }
        }
    }
}
