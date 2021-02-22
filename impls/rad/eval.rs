use std::collections::HashMap;
use std::io;

use types::{RadType, RadNode, RadList, error};

#[allow(dead_code)]
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn repl_env_test() {
        let env = init();
        assert_eq!(env["+"](1, 2), 3);
    }
}

pub type ReplFn = Box<dyn Fn(&Vec<&RadNode>) -> io::Result<RadNode>>;
pub type ReplEnv = HashMap<&'static str, ReplFn>;

pub fn init() -> ReplEnv {
    let mut repl_env: ReplEnv = HashMap::new();
    repl_env.insert("+", Box::new(|a, b| a + b));
    repl_env.insert("-", Box::new(|a, b| a - b));
    repl_env.insert("*", Box::new(|a, b| a * b));
    repl_env.insert("/", Box::new(|a, b| a / b));

    repl_env
}

pub fn eval_ast(tree: &RadNode, ns: &ReplEnv) -> io::Result<RadNode> {
    match tree.rtype {
        RadType::Symbol => Ok(tree.clone()),
        RadType::List => {
            for i in 0..tree.args.len() {
                eval_ast(&tree.args[i], ns)?;
            }
            // if we have a form at the beginning of the list
            // then run it as a function
            if let Some(form) = tree.args.get(1) {
                match ns.get(form.text.as_str()) {
                    Some(fun) => fun(&tree.args.iter().skip(1).collect()),
                    None => error(format!("{} was not found in namespace!", form.text).as_str())
                }
            // otherwise return a copy of the list
            } else {
                Ok(tree.clone())
            }
        },
        _ => Ok(tree.clone()),
    }
}
