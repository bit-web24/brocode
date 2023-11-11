pub mod error;
pub mod expr;
pub mod token;

use error::{ErrorType, TokenizationError};
use expr::Expression;
use std::error::Error;
use token::{kind::Kind, Token};

pub fn tokenize(expression: Expression) -> Result<Vec<Token>, Box<dyn Error>> {
    let mut tokens: Vec<Token> = Vec::new();
    for lexeme in expression {
        match Kind::get(&lexeme) {
            Some(kind) => {
                let token = Token { lexeme, kind };
                tokens.push(token);
            }
            None => {
                return Err(Box::new(TokenizationError::new(
                    lexeme,
                    ErrorType::InvalidToken,
                )));
            }
        }
    }

    Ok(tokens)
}
