use std::fs::File;
use std::io::{self, Read};

pub struct Expression {
    buffer: Vec<u8>,
}

impl Expression {
    pub fn new() -> Self {
        Expression { buffer: Vec::new() }
    }

    pub fn trim(expression: Expression) -> Vec<u8> {
        let mut new_expr: Vec<u8> = Vec::new();
        let mut space_count = 0;

        for c in expression.buffer {
            if c == b' ' || c == b'\n' || c == b'\t' {
                space_count += 1;
                if space_count <= 1 {
                    new_expr.push(b' ');
                }
            } else {
                new_expr.push(c);
                space_count = 0;
            }
        }

        if let Some(b' ') = new_expr.first() {
            new_expr.remove(0);
        }

        return new_expr;
    }
}

pub struct SourceCode {
    file: File,
}

impl SourceCode {
    pub fn new(file_path: &str) -> io::Result<Self> {
        let file = File::open(file_path)?;
        Ok(SourceCode { file })
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
