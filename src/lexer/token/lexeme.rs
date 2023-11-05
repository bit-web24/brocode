mod location;
use location::Location;

#[derive(Debug)]
pub struct Lexeme {
    pub value: String,
    pub location: Location,
}

impl Lexeme {
    pub fn new() -> Lexeme {
        Lexeme {
            value: String::new(),
            location: Location::new(),
        }
    }
}
