use std::collections::HashMap;
use lazy_static::lazy_static;

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

type ReplEnv = HashMap<&'static str, Box<dyn Fn(i64, i64) -> i64>>;

pub fn init() -> ReplEnv {
    let mut repl_env: ReplEnv = HashMap::new();
    repl_env.insert("+", Box::new(add));
    repl_env.insert("-", Box::new(subtract));
    repl_env.insert("*", Box::new(multiply));
    repl_env.insert("/", Box::new(divide));

    repl_env
}

fn add(a: i64, b: i64) -> i64 {
    a + b
}
fn subtract(a: i64, b: i64) -> i64 {
    a - b
}
fn multiply(a: i64, b: i64) -> i64 {
    a * b
}
fn divide(a: i64, b: i64) -> i64 {
    a / b
}
