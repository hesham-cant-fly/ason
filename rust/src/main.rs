use std::path::Path;
use std::io::Read;
use std::fs::File;
use std::process::exit;

use ast::AsonValue;

mod lexer;
mod parser;
mod token;
mod ast;
mod environment;
mod runtime;

fn main() {
    let content = read_file(&Path::new("./test.json"));
    let value = match AsonValue::from_ason_string(&content) {
        Ok(v) => v,
        Err(e) => {
            e.report();
            exit(1);
        }
    };
    println!("{}", value.to_json());
}

fn read_file(path: &Path) -> String {
    // Hello
    let mut f = File::open(path).unwrap();
    let mut text = String::new();
    _ = f.read_to_string(&mut text).unwrap();
    text
}
