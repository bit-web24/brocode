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
