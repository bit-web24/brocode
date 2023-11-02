use std::error::Error;
use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
struct TokenizationError {
    message: String,
}

impl TokenizationError {
    fn new(message: &str) -> Self {
        TokenizationError {
            message: message.to_string(),
        }
    }
}

impl Error for TokenizationError {}

impl Display for TokenizationError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "Tokenization Error: {}", self.message)
    }
}
