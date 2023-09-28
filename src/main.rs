use std::env;
use std::error::Error;
use std::fs::{self, File};
use std::io::Read;

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

    let mut file = File::open(&absolute_file_path)?;
    let mut contents = String::new();

    if let Err(e) = file.read_to_string(&mut contents) {
        return Err(format!("Error reading file: {}", e).into());
    }

    println!("{}:\n{}", filename, contents);

    Ok(())
}
