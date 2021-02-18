use std::fmt;

//pub type RadLink = Option<Box<RadNode>>;
pub type RadList = Vec<RadNode>;

#[derive(Debug)]
pub struct RadNode {
    pub text: String,
    pub rtype: RadType,
    //src_line: usize,
    //src_col: usize,
    pub args: RadList,
}

#[derive(Debug)]
pub enum RadType {
    List,
    Array,
    Map,
    String,
    Symbol,
    Number,
    Char,
    Quote,
    Quasiquote,
    Unquote,
    SpliceUnquote,
    Deref,
    WithMeta,
}

impl fmt::Display for RadNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.rtype {
            RadType::List | RadType::Array | RadType::Map => {
                let inner: Vec<String> = self.args.iter()
                    .map(|a| format!("{}", a))
                    .collect();
                let start = starting_token(&self.rtype).unwrap();
                let end = ending_token(start).unwrap();
                write!(f, "{}{}{}", start, inner.join(" "), end)
            },
            RadType::String => write!(f, "\"{}\"", self.text),
            RadType::Symbol => write!(f, "{}", self.text),
            RadType::Number => write!(f, "{}", self.text),
            RadType::Char =>   write!(f, "'{}'", self.text),
            RadType::Quote | RadType::Quasiquote | RadType::Unquote | RadType:: SpliceUnquote | RadType::Deref => {
                let word = quote_word(&self.rtype).unwrap();
                write!(f, "({} {})", word, self.args[0])
            },
            RadType::WithMeta => {
                write!(f, "(with-meta {} {})", self.args[1], self.args[0])
            },
        }
    }
}

// helper functions to relate list types and their corresponding tokens
pub fn ending_token(starting: &str) -> Option<&'static str> {
    match starting {
        "(" => Some(")"),
        "[" => Some("]"),
        "{" => Some("}"),
        _ => None,
    }
}

pub fn starting_token(rtype: &RadType) -> Option<&'static str> {
    match rtype {
        RadType::List => Some("("),
        RadType::Array => Some("["),
        RadType::Map => Some("{"),
        _ => None,
    }
}

pub fn list_type(starting: &str) -> Option<RadType> {
    match starting {
        "(" => Some(RadType::List),
        "[" => Some(RadType::Array),
        "{" => Some(RadType::Map),
        _ => None,
    }
}

pub fn is_ending_token(token: &str) -> bool {
    match token {
        ")" | "]" | "}" => true,
        _ => false
    }
}

pub fn quote_type(starting: &str) -> Option<RadType> {
    match starting {
        "`" => Some(RadType::Quasiquote),
        "'" => Some(RadType::Quote),
        "~" => Some(RadType::Unquote),
        "~@" => Some(RadType::SpliceUnquote),
        "@" => Some(RadType::Deref),
        _ => None,
    }
}

pub fn quote_word(starting: &RadType) -> Option<&'static str> {
    match starting {
        RadType::Quasiquote => Some("quasiquote"),
        RadType::Quote => Some("quote"),
        RadType::Unquote => Some("unquote"),
        RadType::SpliceUnquote => Some("splice-unquote"),
        RadType::Deref => Some("deref"),
        _ => None,
    }
}
