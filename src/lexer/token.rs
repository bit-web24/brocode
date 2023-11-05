pub mod kind;
pub mod lexeme;

use kind::Kind;
use lexeme::Lexeme;

#[derive(Debug)]
pub struct Token {
    pub lexeme: Lexeme,
    pub kind: Kind,
}
