use lazy_static::lazy_static;
use regex::Regex;

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

struct Reader {
    pos: i64,
    tokens: Vec<String>,
}

fn read_str(input: &str) -> () {
}

fn tokenize(input: &str) -> Vec<String> {
    lazy_static! {static ref RE: Regex =
        Regex::new(
            r#"[\s,]*(~@|[\[\]{}()'`~^@]|"(?:\\.|[^\\"])*"?|;.*|[^\s\[\]{}('"`,;)]*)"#).unwrap();}
    let matches = RE.find_iter(input).map(|mat| {
        mat.as_str().trim().to_string()
    }).collect();
    matches
}

impl<'a> Reader {
    fn next() -> &'a str {
        unimplemented!()
    }
    fn peek() -> &'a str {
        unimplemented!()
    }
}
