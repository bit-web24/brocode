use crate::lexer::expr::Expression;
use std::fs::File;
use std::io::{self, Read};

pub struct SourceCode {
    file: File,
}

impl SourceCode {
    pub fn new(file_path: &str) -> io::Result<Self> {
        let file = File::open(file_path)?;
        Ok(Self { file })
    }
}

impl Iterator for SourceCode {
    type Item = Expression;

    fn next(&mut self) -> Option<Self::Item> {
        let mut expression: Expression = Expression::new();
        let mut stack: Vec<char> = Vec::new();

        loop {
            let mut byte = [0; 1];
            match self.file.read_exact(&mut byte) {
                Ok(_) => {
                    if "\t\n".contains(byte[0] as char) {
                        continue;
                    }

                    expression.buffer.push(byte[0]);
                    if byte[0] == b'(' {
                        stack.push('(');
                    } else if byte[0] == b')' {
                        stack.pop();
                    }

                    if stack.is_empty() {
                        return Some(expression);
                    }
                }

                Err(_) => return None,
            }
        }
    }
}
