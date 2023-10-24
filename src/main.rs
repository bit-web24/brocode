use std::env;
use std::error::Error;
use std::fs;

mod stdin;
use stdin::Expression;
use stdin::SourceCode;

mod lexer;
use lexer::{token::Token, tokenize};

fn main() -> Result<(), Box<dyn Error>> {
    let argv: Vec<String> = env::args().collect();

    if argv.len() <= 1 {
        return Err("Source file required".into());
    }

    let filename = &argv[1];

    let current_dir = env::current_dir()?;
    let path = current_dir
        .to_str()
        .ok_or("Failed to convert current directory to a string")?;
    let absolute_file_path = format!("{}/{}", path, filename);

    if !fs::metadata(&absolute_file_path).is_ok() {
        return Err(format!("File '{}' does not exist", filename).into());
    }

    let source_code = SourceCode::new(&absolute_file_path)?;

    for expression in source_code {
        let tokens: Vec<Token> = tokenize(Expression::trim(expression))?;
		println!("{:#?}", tokens);
    }

    Ok(())
}
