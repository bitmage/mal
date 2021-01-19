extern crate lazy_static;
extern crate regex;

mod reader;
mod types;

use std::io::{self, Write};
use types::RadNode;

fn read(text: &str) -> io::Result<RadNode> {
    reader::read_str(text)
}

fn eval(tree: io::Result<RadNode>) -> io::Result<RadNode> {
    tree
}

fn print(tree: io::Result<RadNode>) -> io::Result<String> {
    tree.map(|t| format!("{}", t))
}

fn rep(text: &str) -> io::Result<String> {
    let tree = read(&text);
    let results = eval(tree);
    let output = print(results);
    output
}

fn main() -> io::Result<()> {
    loop {
        print!("user> ");
        io::stdout().flush()?;
        let mut input_buffer = String::new();
        let result = io::stdin().read_line(&mut input_buffer)?;
        if result == 0 {
            break
        }
        match rep(&input_buffer) {
            Err(e) => println!("{}", e),
            Ok(result) => println!("{}", result),
        }
    }
    Ok(())
}
