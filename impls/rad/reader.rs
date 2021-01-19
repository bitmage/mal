use lazy_static::lazy_static;
use regex::Regex;
use io;

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
    fn tokenize_test() {
        let tests = [
            ("(print \"hello\")",
                vec!["(", "print", "\"hello\"", ")"]),
            ("(list + 2 (list * 3 4))",
                vec!["(", "list", "+", "2", "(",
                "list", "*", "3", "4", ")", ")"]),
        ];
        for (i, o) in tests.iter() {
            let res = tokenize(i);
            assert_eq!(&res[..], &o[..]);
            println!("tokenize result: {} -> {:?}", i, res);
        }
    }

    #[test]
    fn read_str_identity() {
        let tests = [
            "(print \"hello\")",
            "(print \"hello world\")",
            "123",
            "abc",
            "(123 456)",
            "(+ 2 (* 3 4))",
            "(() ())",
        ];
        for input in tests.iter() {
            let res = read_str(input).unwrap();
            //println!("read_str result: {} -> {}", input, res);

            let output = format!("{}", res);
            assert_eq!(*input, output.as_str());
        }
    }

    #[test]
    // test for explicit outputs or errors
    fn read_str_differ() {
        let tests: Vec<(&str, Result<&str, &str>)> = vec![
            ("(   + 1   3 )", Ok("(+ 1 3)")),
            ("\"abc", Err("EOF; No terminating quote on this string.")),
            ("\\", Err("EOF; Expected escaped char.")),
            ("\n", Err("EOF")),
        ];
        for (input, expected) in tests.iter() {
            let res = read_str(input);
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

type Tokens = Vec<String>;

fn tokenize(input: &str) -> Tokens {
    let tokens = TOKEN.captures_iter(input)
        .filter_map(|caps| { caps.get(1) })
        .map(|t| { t.as_str().trim().to_string() })
        .collect();
    tokens
}

pub fn read_str(input: &str) -> io::Result<RadNode> {
    let tokens = tokenize(input);
    //println!("tokens: {:?}", tokens);
    let (list, _) = read_form(&tokens, 0);
    list
}

fn read_form(tokens: &Tokens, pos: usize) -> (io::Result<RadNode>, usize) {
    let token = &tokens.get(pos).map(|t| t.as_str());
    match token {
        Some("(") | Some("[") | Some("{") => read_list(tokens, pos),
        Some(_) => read_atom(tokens, pos),
        None => {
            let e = io::Error::new(io::ErrorKind::UnexpectedEof, "Unexpected EOF.");
            (Err(e), pos)
        }
    }
}

fn read_list(tokens: &Tokens, mut pos: usize) -> (io::Result<RadNode>, usize)
{
    // get starting token to determine list type
    let end_token = match tokens[pos].as_str() {
        "(" => ")",
        "[" => "]",
        "{" => "}",
        _ => ")",
    };

    // skip the opening '('
    pos += 1;
    let mut args = RadList::new();

    let mut _t: Option<&str> = None;
    loop {

        _t = tokens.get(pos).map(|s| s.as_str());
        //_t.map(|t| println!("next list item: {:?}, pos: {}", t, pos));

        match _t {
            // end the list
            Some(end) if end == end_token => {
                pos += 1;
                break;
            },
            None => {
                let e = io::Error::new(
                    io::ErrorKind::UnexpectedEof, "EOF; Unterminated list!"
                );
                return (Err(e), 0);
            },

            // process a list item
            Some(_) => {
                let (form, _pos) = read_form(tokens, pos);
                pos = _pos;
                match form {
                    Ok(f) => args.push(f),
                    Err(e) => return (Err(e), pos),
                }
                //println!("adding to args: {:?}", args)
            },
        }
    }
    let node = RadNode {
        text: "()".to_string(),
        rtype: RadType::List,
        args: args,
    };
    (Ok(node), pos)
}


fn read_atom(tokens: &Tokens, pos: usize) -> (io::Result<RadNode>, usize)
{
    // this should be safe because we peek before getting here
    let mut text = tokens[pos].as_str();
    //println!("read_atom: {}", text);

    let rtype;

    // string
    if text.starts_with('"') {
        if text.len() == 1 || !text.ends_with('"') {
            let e = io::Error::new(io::ErrorKind::InvalidInput, "EOF; No terminating quote on this string.");
            return (Err(e), 0);
        }
        rtype = RadType::String;
        let end = text.len()-1;
        text = &text[1..end];

    } else if NUMBER.is_match(text) {
        rtype = RadType::Number;

    } else if text.starts_with('\\') {
        if text.len() == 1 {
            let e = io::Error::new(io::ErrorKind::InvalidInput, "EOF; Expected escaped char.");
            return (Err(e), 0);
        }
        rtype = RadType::Char;

    // symbol
    } else {
        rtype = RadType::Symbol;
    }

    let node = RadNode {
        text: text.to_string(),
        rtype: rtype,
        args: RadList::new(),
    };
    (Ok(node), pos + 1)
}
