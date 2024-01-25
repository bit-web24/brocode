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

    pub fn unwrap(self) -> String {
        return String::from_utf8_lossy(&self.buffer).to_string();
    }
}

impl Iterator for Expression {
    type Item = Lexeme;

    fn next(&mut self) -> Option<Self::Item> {
        let mut lexeme = Lexeme::new();
        let mut is_str: bool = false;

        while self.cursor < self.buffer.len() {
            let ch = self.buffer[self.cursor] as char;

            match ch {
                '\"' => {
                    lexeme.value.push(ch);
                    if is_str {
                        lexeme.location.line = self.line;
                        lexeme.location.len = lexeme.value.len();
                        lexeme.location.column = self.column - lexeme.location.len;
                        self.cursor += 1;
                        return Some(lexeme);
                    } else {
                        is_str = true;
                    }
                    self.column += 1;
                    self.cursor += 1;
                }
                ' ' => {
                    if is_str {
                        lexeme.value.push(ch);
                        self.column += 1;
                        self.cursor += 1;
                        continue;
                    }
                    if lexeme.value.is_empty() {
                        self.column += 1;
                        self.cursor += 1;
                    } else {
                        lexeme.location.line = self.line;
                        lexeme.location.len = lexeme.value.len();
                        lexeme.location.column = self.column - lexeme.location.len;
                        return Some(lexeme);
                    }
                }
                '\t' => {
                    if is_str {
                        lexeme.value.push(ch);
                    }
                    self.column += 1;
                    self.cursor += 1;
                }
                '\n' => {
                    if is_str {
                        lexeme.value.push(ch);
                        self.column += 1;
                        self.cursor += 1;
                        continue;
                    }
                    self.line += 1;
                    self.column = 1;
                    self.cursor += 1;
                }
                _ if "[{(<|,>)}]".contains(ch) => {
                    if is_str {
                        lexeme.value.push(ch);
                        self.column += 1;
                        self.cursor += 1;
                        continue;
                    }
                    if lexeme.value.is_empty() {
                        lexeme.value.push(ch);
                        self.cursor += 1;
                        self.column += 1;
                    }
                    lexeme.location.line = self.line;
                    lexeme.location.len = lexeme.value.len();
                    lexeme.location.column = self.column - lexeme.location.len;
                    return Some(lexeme);
                }
                _ => {
                    lexeme.value.push(ch);
                    self.cursor += 1;
                    self.column += 1;
                }
            }
        }

        None
    }
}
