use std::error::Error;

#[derive(Debug)]
pub struct Lexeme {
    pub value: String,
}

impl Lexeme {
    pub fn new() -> Lexeme {
        Lexeme {
            value: String::new(),
        }
    }
}

impl Iterator for Expression {
    type Item = Lexeme;

    fn next(&mut self) -> Option<Self::Item> {
        let mut lexeme: Lexeme = Lexeme::new();

        if lexeme.is_empty() {
            return None;
        }
        return Some(lexeme);
    }
}
