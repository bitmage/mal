use std::collections::HashMap;
use std::io;

use types::{RadType, RadNode};

//thread_local! {
    //static PLACEHOLDER: RadNode = placeholder();
//}

//fn placeholder() -> RadNode {
    //RadNode {
        //text: "".to_string(),
        //rtype: RadType::Placeholder,
        //args: Vec::new(),
    //}
//}

// I tried implementing eval_ast without clone... no joy
// TODO: find out if this is possible/feasible?
//fn swap_and_eval(node: &mut RadNode, ns: &ReplEnv) -> io::Result<()> {
    //let mut result: io::Result<()> = Ok(());
    //PLACEHOLDER.with(|mut arg| {
        //mem::swap(node, &mut arg);
        //match eval_ast(arg, ns) {
            //Ok(a) => mem::swap(node, &mut arg),
            //Err(e) => result = Err(e),
        //}
    //});
    //result
//}


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

pub type ReplEnv = HashMap<&'static str, Box<dyn Fn(i64, i64) -> i64>>;

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
        RadType::Symbol => Ok(*tree.clone()),
        RadType::List => {
            for i in 0..tree.args.len() {
                eval_ast(&tree.args[i], ns)?;
            }
            Ok(*tree.clone())
        },
        _ => Ok(*tree.clone()),
    }
}
