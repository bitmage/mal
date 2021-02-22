extern crate lazy_static;
extern crate regex;

mod reader;
mod types;
mod eval;

use std::io::{self, Write};
use types::RadNode;
use eval::{eval_ast};

#[allow(dead_code)]
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    // test for explicit outputs or errors
    fn rep_test() {
        let tests: Vec<(&str, Result<&str, &str>)> = vec![
            ("(+ 1 3)", Ok("4")),
        ];
        let ns = eval::init();
        for (input, expected) in tests.iter() {
            let res = rep(input, &ns);
            println!("result: {:?}", res);
            match expected {
                Ok(out) => {
                    let output = format!("{}", res.unwrap());
                    assert_eq!(*out, output.as_str());
                },
                Err(err) => {
                    match &res {
                        Err(e) => {
                            let e_str = &format!("{}", e);
                            assert_eq!(e_str, err);
                        },
                        Ok(nodes) => {
                            panic!("Expected error: {:?}, got RadNodes: {:?}.", err, nodes)
                        }
                    }
                },
            }

        }
    }
}

fn read(text: &str) -> io::Result<RadNode> {
    reader::read_str(text)
}

fn eval(tree: RadNode, ns: &eval::ReplEnv) -> io::Result<RadNode> {
    eval_ast(&tree, ns)
}

fn print(tree: RadNode) -> String {
    format!("{}", tree)
}

fn rep(text: &str, ns: &eval::ReplEnv) -> io::Result<String> {
    let tree = read(&text)?;
    let results = eval(tree, ns)?;
    Ok(print(results))
}

fn main() -> io::Result<()> {
    let ns = eval::init();
    loop {
        print!("user> ");
        io::stdout().flush()?;
        let mut input_buffer = String::new();
        let result = io::stdin().read_line(&mut input_buffer)?;
        if result == 0 {
            break
        }
        match rep(&input_buffer, &ns) {
            Err(e) => println!("{}", e),
            Ok(result) => println!("{}", result),
        }
    }
    Ok(())
}
