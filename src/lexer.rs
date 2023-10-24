use crate::stdin::Expression;
use std::error::Error;
use std::fmt;

pub mod token;
use token::{Kind, Lexeme, Location, Token};

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

impl fmt::Display for TokenizationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Tokenization Error: {}", self.message)
    }
}

pub fn tokenize(expression: Expression) -> Result<Vec<Token>, Box<dyn Error>> {
    let mut tokens: Vec<Token> = Vec::new();
    tokens.push(Token {
        lexeme: Lexeme {
            value: String::from("lambda"),
        },
        location: Location {
            line: 2,
            column: 5,
            len: 6,
        },
        kind: Kind::Name,
    });

    Ok(tokens)
}
