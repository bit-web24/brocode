pub mod kind;
pub mod lexeme;
pub mod location;

use kind::Kind;
use lexeme::Lexeme;
use location::Location;

#[derive(Debug)]
pub struct Token {
    pub lexeme: Lexeme,
    pub location: Location,
    pub kind: Kind,
}
