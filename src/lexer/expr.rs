use super::token::lexeme::Lexeme;

pub struct Expression {
    pub buffer: Vec<u8>,
    pub cursor: usize,
}

impl Expression {
    pub fn new() -> Self {
        Self {
            buffer: Vec::new(),
            cursor: 0,
        }
    }

    pub fn trim(expression: Self) -> Self {
        let mut new_buffer: Vec<u8> = Vec::new();
        let mut space_count = 0;

        for c in expression.buffer {
            if c == b' ' || c == b'\n' || c == b'\t' {
                space_count += 1;
                if space_count <= 1 {
                    new_buffer.push(b' ');
                }
            } else {
                new_buffer.push(c);
                space_count = 0;
            }
        }

        if let Some(b' ') = new_buffer.first() {
            new_buffer.remove(0);
        }

        return Self {
            buffer: new_buffer,
            cursor: 0,
        };
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

            match ch {
                _ if ch.is_whitespace() => {
                    self.cursor += 1;
                    if !lexeme.value.is_empty() {
                        return Some(lexeme);
                    }
                }
                _ if "[{(<|,>)}]".contains(ch) => {
                    if lexeme.value.is_empty() {
                        lexeme.value.push(ch);
                        self.cursor += 1;
                    }
                    return Some(lexeme);
                }
                _ => {
                    lexeme.value.push(ch);
                    self.cursor += 1;
                }
            }
        }

        None
    }
}
