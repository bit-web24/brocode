use super::token::lexeme::Lexeme;

#[derive(Debug)]
pub struct Expression {
    pub buffer: Vec<u8>,
    pub cursor: usize,
    line: usize,
    column: usize,
}

impl Expression {
    pub fn new() -> Self {
        Self {
            buffer: Vec::new(),
            cursor: 0,
            line: 1,
            column: 1,
        }
    }

    pub fn unwrap(self) -> Vec<u8> {
        return self.buffer;
    }
}

impl Iterator for Expression {
    type Item = Lexeme;

    fn next(&mut self) -> Option<Self::Item> {
        let mut lexeme = Lexeme::new();

        while self.cursor < self.buffer.len() {
            let ch = self.buffer[self.cursor] as char;
            self.cursor += 1;

            match ch {
                ' ' => {
                    if !lexeme.value.is_empty() {
                        lexeme.location.column = self.column;
                        lexeme.location.line = self.line;
                        lexeme.location.len = lexeme.value.len();
                        return Some(lexeme);
                    }
                }
                '\t' | '\n' => {
                    if ch == '\n' {
                        self.line += 1;
                        self.column = 1;
                    }
                }
                ch if "[{(<|,>)}]".contains(ch) => {
                    if lexeme.value.is_empty() {
                        lexeme.value.push(ch);
                        lexeme.location.column = self.column;
                        lexeme.location.line = self.line;
                        lexeme.location.len = 1;
                        return Some(lexeme);
                    }
                }
                _ => {
                    lexeme.value.push(ch);
                    self.column += 1;
                }
            }
        }

        if !lexeme.value.is_empty() {
            lexeme.location.column = self.column;
            lexeme.location.line = self.line;
            lexeme.location.len = lexeme.value.len();
            Some(lexeme)
        } else {
            None
        }
    }
}
