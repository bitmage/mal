use lazy_static::lazy_static;
use regex::Regex;
use std::iter::Peekable;

use types::{RadNode, RadList, RadLink, RadType};

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

pub fn read_str(input: &str) -> Option<RadNode> {
    let tokens = tokenize(input);
    let (list, _) = read_form(&tokens, 0);
    list
}

fn read_form(tokens: &Tokens, pos: usize) -> (Option<RadNode>, usize) {
    let token = &tokens.get(pos);
    match token.map(|t| t.as_str()) {
        Some("(") => read_list(tokens, pos),
        Some(_) => read_atom(tokens, pos),
        None => (None, pos)
    }
}

fn read_list(tokens: &Tokens, mut pos: usize) -> (Option<RadNode>, usize)
{
    let mut args = RadList::new();
    //let src = Vec<String>::new();

    let mut _t: Option<&str> = None;
    loop {

        // get the next token
        pos += 1;
        _t = tokens.get(pos).map(|s| s.as_str());

        match _t {
            // end the list
            Some(")") => break,
            None => panic!("Unterminated list!"),

            // process a list item
            Some(_) => {
                let (form, _pos) = read_form(tokens, pos);
                pos = _pos;
                args.push(form.unwrap());
            },
        }
    }
    let node = RadNode {
        src: "".to_string(),
        rtype: RadType::List,
        args: RadList::new(),
    };
    (Some(node), pos)
}

fn read_atom(tokens: &Tokens, pos: usize) -> (Option<RadNode>, usize)
{
    // this should be safe because we peek before getting here
    let src = tokens.get(pos).unwrap().to_string();
    let node = RadNode {
        src: src,
        rtype: RadType::RString,
        args: RadList::new(),
    };
    (Some(node), pos + 1)
}
