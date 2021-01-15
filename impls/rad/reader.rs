use lazy_static::lazy_static;
use regex::Regex;
use std::iter::Peekable;

use types::{RadNode, RadList, RadType};

#[cfg(test)]
mod test {
    use super::tokenize;

    #[test]
    fn tokenize_test() {
        let tests = [
            ("(print 'hello')",
                vec!["(", "print", "'", "hello", "'", ")"]),
            ("(list + 2 (list * 3 4))",
                vec!["(", "list", "+", "2", "(",
                "list", "*", "3", "4", ")", ")"]),
        ];
        for (i, o) in tests.iter() {
            let res = tokenize(i);
            assert_eq!(&res[..], &o[..]);
            println!("result: {} -> {:?}", i, res);
        }
    }
}

type Tokens = Vec<String>;

fn tokenize(input: &str) -> Tokens {
    lazy_static! {static ref RE: Regex =
        Regex::new(
            r#"[\s,]*(~@|[\[\]{}()'`~^@]|"(?:\\.|[^\\"])*"?|;.*|[^\s\[\]{}('"`,;)]*)"#).unwrap();}
    let matches = RE.find_iter(input).map(|mat| {
        mat.as_str().trim().to_string()
    }).collect();
    matches
}

pub fn read_str(input: &str) -> RadList {
    let tokens = tokenize(input);
    read_form(&tokens)
}

fn read_form(tokens: &Tokens) -> RadList {
    let reader = tokens.iter().peekable();
    let token = reader.peek();
    match token.map(|t| t.as_str()) {
        Some("(") => read_list(reader),
        Some(_) => read_atom(reader),
        None => RadList::new()
    }
}

fn read_list<I: AsRef<str>>(reader: impl Iterator<Item = I>) -> RadList
{
    // this should be safe because we peek before getting here
    let src = reader.next().unwrap();
    vec!(RadNode {
        src: src.as_ref().to_string(),
        rtype: RadType::List,
        args: RadList::new(),
    })
}

fn read_atom<I: AsRef<str>>(reader: impl Iterator<Item = I>) -> RadList
{
    // this should be safe because we peek before getting here
    let src = reader.next().unwrap();
    vec!(RadNode {
        src: src.as_ref().to_string(),
        rtype: RadType::RString,
        args: RadList::new(),
    })
}
