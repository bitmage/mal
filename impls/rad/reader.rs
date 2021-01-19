use lazy_static::lazy_static;
use regex::Regex;

use types::{RadNode, RadList, RadType};

lazy_static! {static ref TOKEN: Regex = Regex::new(
    r#"[\s,]*(~@|[\[\]{}()'`~^@]|"(?:\\.|[^\\"])*"?|;.*|[^\s\[\]{}('"`,;)]*)"#
).unwrap();}

lazy_static! {static ref NUMBER: Regex = Regex::new(
    r#"^-?[0-9][0-9\.]+$"#
).unwrap();}

#[cfg(test)]
mod test {
    use super::*;

    //#[test]
    //fn tokenize_test() {
        //let tests = [
            //("(print 'hello')",
                //vec!["(", "print", "'", "hello", "'", ")"]),
            //("(list + 2 (list * 3 4))",
                //vec!["(", "list", "+", "2", "(",
                //"list", "*", "3", "4", ")", ")"]),
        //];
        //for (i, o) in tests.iter() {
            //let res = tokenize(i);
            //assert_eq!(&res[..], &o[..]);
            //println!("tokenize result: {} -> {:?}", i, res);
        //}
    //}

    #[test]
    fn read_str_test() {
        let tests = [
            "(print \"hello\")",
            "(print \"hello world\")",
            "123",
            "abc",
            "(123 456)",
            "(+ 2 (* 3 4))",
        ];
        for (input) in tests.iter() {
            let res = read_str(input).unwrap();
            //println!("read_str result: {} -> {}", input, res);

            let output = format!("{}", res);
            assert_eq!(*input, output.as_str());
        }
    }
}

type Tokens = Vec<String>;

fn tokenize(input: &str) -> Tokens {
    let tokens = TOKEN.captures_iter(input)
        .filter_map(|caps| { caps.get(1) })
        .map(|t| { t.as_str().trim().to_string() })
        .collect();
    tokens
}

pub fn read_str(input: &str) -> Option<RadNode> {
    let tokens = tokenize(input);
    //println!("tokens: {:?}", tokens);
    let (list, _) = read_form(&tokens, 0);
    list
}

fn read_form(tokens: &Tokens, pos: usize) -> (Option<RadNode>, usize) {
    let token = &tokens.get(pos).map(|t| t.as_str());
    match token {
        Some("(") => read_list(tokens, pos),
        Some(_) => read_atom(tokens, pos),
        None => (None, pos)
    }
}

fn read_list(tokens: &Tokens, mut pos: usize) -> (Option<RadNode>, usize)
{
    // skip the opening '('
    pos += 1;
    let mut args = RadList::new();

    let mut _t: Option<&str> = None;
    loop {

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
                //println!("adding to args: {:?}", args)
            },
        }
    }
    let node = RadNode {
        text: "()".to_string(),
        rtype: RadType::List,
        args: args,
    };
    (Some(node), pos)
}


fn read_atom(tokens: &Tokens, pos: usize) -> (Option<RadNode>, usize)
{
    // this should be safe because we peek before getting here
    let mut text = tokens[pos].as_str();
    //println!("read_atom: {}", text);

    let rtype;

    // string
    if text.starts_with('"') {
        rtype = RadType::String;
        let end = text.len()-1;
        text = &text[1..end];

    } else if NUMBER.is_match(text) {
        rtype = RadType::Number;

    // symbol
    } else {
        rtype = RadType::Symbol;
    }

    let node = RadNode {
        text: text.to_string(),
        rtype: rtype,
        args: RadList::new(),
    };
    (Some(node), pos + 1)
}
