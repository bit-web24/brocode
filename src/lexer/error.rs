use std::error::Error;
use std::fmt::{Display, Formatter, Result};

use super::token::lexeme::Lexeme;

#[derive(Debug)]
pub enum ErrorType {
    InvalidToken,
}

#[derive(Debug)]
pub struct TokenizationError {
    message: String,
}

impl TokenizationError {
    pub fn new(lexeme: Lexeme, error_t: ErrorType) -> Self {
        TokenizationError {
            message: message(error_t, lexeme).to_string(),
        }
    }
}

impl Error for TokenizationError {}

impl Display for TokenizationError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "Error::Tokenization::{}", self.message)
    }
}

fn message(error_t: ErrorType, lexeme: Lexeme) -> String {
    format!("{:?}(\"{}\")", error_t, lexeme.value)
}
