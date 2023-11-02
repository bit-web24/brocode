pub mod data;
pub mod error;
pub mod expr;
pub mod token;

use expr::Expression;
use std::error::Error;
use token::{kind::Kind, Token};

pub fn tokenize(expression: Expression) -> Result<Vec<Token>, Box<dyn Error>> {
    let mut tokens: Vec<Token> = Vec::new();
    for (lexeme, location) in expression {
        let kind: Kind = Kind::get(&lexeme);
        let token = Token {
            lexeme,
            location,
            kind,
        };
        tokens.push(token);
    }

    Ok(tokens)
}
