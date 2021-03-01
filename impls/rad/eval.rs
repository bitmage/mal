use std::collections::HashMap;
use std::io;

use types::{RadVal, RadNode, error_invalid_data, make_node, rtype_as_str};

pub type ReplFn = Box<dyn Fn(&Vec<&RadNode>) -> io::Result<RadNode>>;
pub type ReplEnv = HashMap<&'static str, ReplFn>;

pub fn init() -> ReplEnv {
    let mut repl_env: ReplEnv = HashMap::new();
    repl_env.insert("+", fn_float(Box::new(|a, b| a + b)));
    repl_env.insert("-", fn_float(Box::new(|a, b| a - b)));
    repl_env.insert("*", fn_float(Box::new(|a, b| a * b)));
    repl_env.insert("/", fn_float(Box::new(|a, b| a / b)));
    repl_env
}

// helper for constructing a function that works with floats
pub fn fn_float(proc: Box<dyn Fn(f64, f64) -> f64>) -> ReplFn {
    Box::new(move |args| {
        // convert args to nums, complaining if type conversion fails
        let mut num: f64 = 0.0;
        for (i, a) in args.iter().enumerate() {
            match a.rval {
                RadVal::Number(n) => {
                    if i == 0 {
                        num = n;
                    } else {
                        num = proc(num, n);
                    }
                },
                _ => {
                    let msg = format!(
                        "{} is not a Number, it's a {}.",
                        a, rtype_as_str(&a)
                    );
                    Err(error_invalid_data(msg))?;
                }
            }
        }
        // apply proc to any number of args
        let num_str = num.to_string();
        Ok(make_node(num_str.as_str(), RadVal::Number(num)))
    })
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
                        Err(error_invalid_data(txt))
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
