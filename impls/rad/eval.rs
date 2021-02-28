use std::collections::HashMap;
use std::io;

use types::{RadVal, RadNode, error_invalid_data};

#[allow(dead_code)]
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn repl_env_test() {
        let env = init();
        //assert_eq!(env["+"](1, 2), 3);
    }
}

pub type ReplFn = Box<dyn Fn(&Vec<&RadNode>) -> io::Result<RadNode>>;
pub type ReplEnv = HashMap<&'static str, ReplFn>;

pub fn init() -> ReplEnv {
    let repl_env: ReplEnv = HashMap::new();
    // TODO: refactor types and then do a proper implementation
    //repl_env.insert("+", Box::new(|a, b| a + b));
    //repl_env.insert("-", Box::new(|a, b| a - b));
    //repl_env.insert("*", Box::new(|a, b| a * b));
    //repl_env.insert("/", Box::new(|a, b| a / b));

    repl_env
}

pub fn eval_ast(tree: &RadNode, ns: &ReplEnv) -> io::Result<RadNode> {
    match &tree.rval {
        RadVal::Symbol => Ok(tree.clone()),
        RadVal::List(items) => {
            for i in 0..items.len() {
                eval_ast(&items[i], ns)?;
            }
            // if we have a form at the beginning of the list
            // then run it as a function
            if let Some(form) = items.get(0) {
                match ns.get(form.text.as_str()) {
                    Some(fun) => fun(&items.iter().skip(1).collect()),
                    None => {
                        let txt = format!("{} was not found in namespace!", form.text);
                        Err(error_invalid_data(txt.as_str()))
                    }
                }
            // otherwise return a copy of the list
            } else {
                Ok(tree.clone())
            }
        },
        _ => Ok(tree.clone()),
    }
}
