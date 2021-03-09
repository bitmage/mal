use std::collections::HashMap;
use std::io;

use types::{
    RadVal,
    RadNode,
    error_invalid_data,
    make_node,
    rtype_as_str,
};

pub type ReplFn = Box<dyn Fn(&Vec<&RadNode>) -> io::Result<RadNode>>;
pub type ReplEnv = HashMap<&'static str, ReplFn>;

pub fn init() -> ReplEnv {
    let mut repl_env: ReplEnv = HashMap::new();
    //TODO: I bet I could remove the Box::new()
    repl_env.insert("+", fn_float(Box::new(|a, b| a + b)));
    repl_env.insert("-", fn_float(Box::new(|a, b| a - b)));
    repl_env.insert("*", fn_float(Box::new(|a, b| a * b)));
    repl_env.insert("/", fn_float(Box::new(|a, b| a / b)));
    repl_env
}

//impl ReplEnv {
    //pub fn set(sym: &RadNode, value: &RadNode) -> io::Result<()> {
        //panic!("not implemented")
    //}
    //pub fn find(sym: &RadNode) -> io::Result<()> {
        //panic!("not implemented")
    //}
    //pub fn get(sym: &RadNode) -> io::Result<()> {
        //panic!("not implemented")
    //}
//}

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
