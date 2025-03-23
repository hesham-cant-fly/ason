use std::path::Path;
use std::io::Read;
use std::fs::File;

use ast::AsonValue;

mod lexer;
mod parser;
mod token;
mod ast;
mod environment;

fn main() -> Result<(), String> {
    let content = read_file(&Path::new("./test.json"));
    let value = AsonValue::from_ason_string(&content)?;
    println!("{}", value.to_json());
    Ok(())
}

fn read_file(path: &Path) -> String {
    let mut f = File::open(path).unwrap();
    let mut text = String::new();
    _ = f.read_to_string(&mut text).unwrap();
    text
}
