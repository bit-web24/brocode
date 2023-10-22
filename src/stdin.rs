use std::fs::File;
use std::io::{self, Read};

pub struct Expression {
    buffer: Vec<u8>,
}

impl Expression {
    pub fn new() -> Self {
        Self { buffer: Vec::new() }
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

        return Self { buffer: new_buffer };
    }

	pub fn unwrap(self) -> Vec<u8> {
		return self.buffer
	}
}

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
