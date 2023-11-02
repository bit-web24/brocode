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

        loop {
            let mut byte = [0; 1];
            match self.file.read_exact(&mut byte) {
                Ok(_) => {
                    if byte[0] == b';' {
                        if !expression.buffer.is_empty() {
                            return Some(expression);
                        }
                    } else {
                        expression.buffer.push(byte[0]);
                    }
                }
                Err(_) => {
                    if !expression.buffer.is_empty() {
                        return Some(expression);
                    } else {
                        return None;
                    }
                }
            }
        }
    }
}
