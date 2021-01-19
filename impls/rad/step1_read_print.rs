extern crate lazy_static;
extern crate regex;

mod reader;
mod types;

use std::io::{self, Write};
use types::RadNode;

fn read(text: &str) -> Option<RadNode> {
    reader::read_str(text)
}

fn eval(tree: Option<RadNode>) -> Option<RadNode> {
    tree
}

fn print(tree: Option<RadNode>) -> String {
    tree.map_or("EOF".to_string(), |t| format!("{}", t))
}

fn rep(text: &str) -> String {
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
        let evaluated = rep(&input_buffer);
        println!("{}", evaluated);
    }
    Ok(())
}
