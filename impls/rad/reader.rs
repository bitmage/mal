use lazy_static::lazy_static;
use regex::Regex;
use io;

use types::{
    RadNode,
    make_node,
    RadList,
    RadVal,
    ending_token,
    make_list_val,
    make_map_val,
    make_meta_val,
    make_quote_val,
    is_map,
    is_ending_token,
    error_eof,
    convert_error,
};

lazy_static! {static ref TOKEN: Regex = Regex::new(
    r#"[\s,]*(~@|[\[\]{}()'`~^@]|"(?:\\.|[^\\"])*"?|;.*|[^\s\[\]{}('"`,;)]*)"#
).unwrap();}

lazy_static! {static ref NUMBER: Regex = Regex::new(
    r#"^-?[0-9][0-9\.]+$"#
).unwrap();}

lazy_static! {static ref ALL_FORWARD_SLASH: Regex = Regex::new(
    r#"^\\+$"#
).unwrap();}

lazy_static! {static ref ODD_FORWARD_SLASH: Regex = Regex::new(
    r#"^(?:\\\\)*\\$"#
).unwrap();}

#[allow(dead_code)]
#[cfg(test)]
mod test {
    use super::*;

    #[test]
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
            ("\\\\", Err("EOF; Expected escaped char.")),
            ("\n", Err("EOF; read_form")),
            (r#" "\" "#, Err("EOF; I don't like the cut of your jib.")),
            (r#"'"#, Err("EOF; Quote what?")),
            ("'1", Ok("(quote 1)")),
            ("`1", Ok("(quasiquote 1)")),
            ("^{a 1} [1 2 3]", Ok("(with-meta [1 2 3] {a 1})")),
            ("^{a 1}", Err("EOF; Meta for what?")),
            ("((^) (baz foo))", Err("EOF; Meta for what?")),
        ];
        for (input, expected) in tests.iter() {
            let res = read_str(input);
            //println!("result radtree: {:?}", res);
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
        .filter(|t| !t.is_empty())
        .collect();
    tokens
}

pub fn read_str(input: &str) -> io::Result<RadNode> {
    let tokens = tokenize(input);
    //println!("tokens: {:?}", tokens);
    read_form(&tokens, 0).map(|f| f.0)
}

fn read_form(tokens: &Tokens, pos: usize) -> io::Result<(RadNode, usize)> {
    let token = &tokens.get(pos).map(|t| t.as_str());
    match token {
        Some("(") | Some("[") | Some("{") => read_list(tokens, pos),
        Some("'") | Some("`") | Some("~") | Some("~@") | Some("@") =>
            read_quote(tokens, pos),
        Some("^") => read_meta(tokens, pos),
        Some(_) => read_atom(tokens, pos),
        None => {
            let e = io::Error::new(io::ErrorKind::UnexpectedEof, "EOF; read_form");
            Err(e)
        }
    }
}

fn read_list(tokens: &Tokens, mut pos: usize) -> io::Result<(RadNode, usize)>
{
    // get starting token to determine list type
    // unwraps are safe as long as called by read_form
    let start_token = tokens[pos].as_str();
    let end_token = ending_token(start_token).expect(
        format!("read_list got invalid start_token: {}", start_token).as_str()
    );

    // skip the start_token
    pos += 1;
    let mut items = RadList::new();

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
                return Err(error_eof("EOF; Unterminated list!".to_string()));
            },

            // process a list item
            Some(_) => {
                let (f, _pos) = read_form(tokens, pos)?;
                items.push(f);
                pos = _pos;
                //println!("adding to args: {:?}", args)
            },
        }
    }
    let lval = if is_map(start_token) {
        make_map_val(start_token, items)?
    } else {
        make_list_val(start_token, items)?
    };
    let node = make_node(
        format!("{}{}", start_token, end_token).as_str(),
        lval,
    );
    Ok((node, pos))
}

fn read_quote(tokens: &Tokens, mut pos: usize) -> io::Result<(RadNode, usize)>
{
    // this should be safe because we peek before getting here
    let text = tokens[pos].as_str();

    // skip quote char
    pos += 1;

    next_token_or_err(tokens, pos, "EOF; Quote what?")?;

    // read what is to be quoted
    let (q, new_pos) = read_form(tokens, pos)?;

    // construct quote node
    let node = make_node(text, make_quote_val(text, q)?);
    Ok((node, new_pos))
}

fn next_token_or_err<'a>(tokens: &'a Tokens, pos: usize, message: &str) -> io::Result<&'a str> {
    let token = tokens.get(pos).map(|t| t.as_str());
    match token {
        Some(t) if is_ending_token(t) => {
            let e = io::Error::new(io::ErrorKind::InvalidInput, message);
            return Err(e)
        },
        None => {
            let e = io::Error::new(io::ErrorKind::InvalidInput, message);
            return Err(e)
        },
        Some(t) => Ok(t)
    }
}

fn read_meta(tokens: &Tokens, mut pos: usize) -> io::Result<(RadNode, usize)>
{
    // skip quote char
    pos += 1;
    next_token_or_err(tokens, pos, "EOF; Meta for what?")?;


    // add meta to node
    let (meta, new_pos) = read_form(tokens, pos)?;
    pos = new_pos;

    // add target form to node
    // assemble result and return
    next_token_or_err(tokens, pos, "EOF; Meta for what?")?;
    let (form, new_pos) = read_form(tokens, pos)?;

    // construct quote node
    let node = make_node("^", make_meta_val(meta, form));
    Ok((node, new_pos))
}

fn read_atom(tokens: &Tokens, pos: usize) -> io::Result<(RadNode, usize)>
{
    // this should be safe because we peek before getting here
    let mut text = tokens[pos].as_str();
    //println!("read_atom: {}", text);

    let rval;

    // string
    if text.starts_with('"') {
        if text.len() == 1 || !text.ends_with('"') {
            let e = io::Error::new(io::ErrorKind::InvalidInput, "EOF; No terminating quote on this string.");
            return Err(e)
        }

        // get rid of enclosing quotes
        rval = RadVal::String;
        let end = text.len()-1;
        text = &text[1..end];

        // check to see if someone's playing with us
        if ODD_FORWARD_SLASH.is_match(text) {
            let e = io::Error::new(io::ErrorKind::InvalidInput, "EOF; I don't like the cut of your jib.");
            return Err(e)
        }

    } else if NUMBER.is_match(text) {
        let num = text.parse::<f64>().map_err(convert_error)?;
        rval = RadVal::Number(num);

    } else if text.starts_with('\\') {
        if text.len() == 1 || ALL_FORWARD_SLASH.is_match(text) {
            let e = io::Error::new(io::ErrorKind::InvalidInput, "EOF; Expected escaped char.");
            return Err(e)
        }
        rval = RadVal::Char;

    // symbol
    } else {
        rval = RadVal::Symbol;
    }

    let node = make_node(text, rval);
    Ok((node, pos + 1))
}
