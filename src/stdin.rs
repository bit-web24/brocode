use std::fs::File;
use std::io::{self, Read};

pub struct ExpressionReader {
    file: File,
    buffer: Vec<u8>,
}

impl ExpressionReader {
    pub fn new(file_path: &str) -> io::Result<Self> {
        let file = File::open(file_path)?;
        Ok(ExpressionReader {
            file,
            buffer: Vec::new(),
        })
    }

    pub fn process_expression(expression: String) -> String {
        let mut result = String::new();
        let mut space_count = 0;

        for c in expression.chars() {
            if c == ' ' {
                space_count += 1;
                if space_count <= 1 {
                    result.push(c);
                }
            } else {
                result.push(c);
                space_count = 0;
            }
        }

        result
    }
}

impl Iterator for ExpressionReader {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        self.buffer.clear();

        loop {
            let mut byte = [0; 1];
            match self.file.read_exact(&mut byte) {
                Ok(_) => {
                    if byte[0] == b';' {
                        if !self.buffer.is_empty() {
                            return Some(String::from_utf8_lossy(&self.buffer).to_string());
                        }
                    } else if byte[0] == b'\n' || byte[0] == b'\t' || byte[0] == b'\r' {
                        continue;
                    } else {
                        self.buffer.push(byte[0]);
                    }
                }
                Err(_) => {
                    if !self.buffer.is_empty() {
                        return Some(String::from_utf8_lossy(&self.buffer).to_string());
                    } else {
                        return None;
                    }
                }
            }
        }
    }
}
