use std::fmt;
use std::io;
use std::io::ErrorKind::{UnexpectedEof, InvalidData};
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::cell::RefCell;

//pub type RadLink = Option<Box<RadNode>>;
pub type RadList = Vec<RadNode>;
pub type RadMap = HashMap<RadNode, RadNode>;

#[derive(Debug, Clone)]
pub struct RadNode {
    pub text: String,
    pub rval: RadVal,
    pub id: usize,
    //src_line: usize,
    //src_col: usize,
}

impl Hash for RadNode {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl PartialEq for RadNode {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
impl Eq for RadNode {}

#[derive(Debug, Clone)]
pub enum RadVal {
    Nil,
    List(RadList),
    Array(RadList),
    Map(RadMap),
    String,
    Symbol,
    Number,
    Char,
    Quote(Box<RadNode>),
    Quasiquote(Box<RadNode>),
    Unquote(Box<RadNode>),
    SpliceUnquote(Box<RadNode>),
    Deref(Box<RadNode>),
    WithMeta(Box<RadNode>, Box<RadNode>),
}

// can display any list type
// also used to display map with converter map_to_list
fn display_list(f: &mut fmt::Formatter, list: &RadNode, items: &RadList) -> fmt::Result {
    let inner: Vec<String> = items.iter()
        .map(|a| format!("{}", a))
        .collect();
    let start = starting_token(&list.rval).unwrap();
    let end = ending_token(start).unwrap();
    write!(f, "{}{}{}", start, inner.join(" "), end)
}

impl fmt::Display for RadNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.rval {
            RadVal::List(items) | RadVal::Array(items) => {
                display_list(f, self, items)
            },
            RadVal::Map(map) => {
                display_list(f, self, &map_to_list(map))
            },
            RadVal::String => write!(f, "\"{}\"", self.text),
            RadVal::Symbol => write!(f, "{}", self.text),
            RadVal::Number => write!(f, "{}", self.text),
            RadVal::Char =>   write!(f, "'{}'", self.text),
            RadVal::Nil =>   write!(f, "nil"),
            RadVal::Quote(form)
                | RadVal::Quasiquote(form)
                | RadVal::Unquote(form)
                | RadVal::SpliceUnquote(form)
                | RadVal::Deref(form) => {
                let word = quote_word(&self.rval).unwrap();
                write!(f, "({} {})", word, form)
            },
            RadVal::WithMeta(meta, form) => {
                write!(f, "(with-meta {} {})", form, meta)
            },
        }
    }
}

// used to generate a unique ID for each form
thread_local! {
    static NEXT_ID: RefCell<usize> = RefCell::new(0);
}

pub fn gen_id() -> usize {
    NEXT_ID.with(|next| -> usize {
        let mut next = next.borrow_mut();
        let id = *next;
        *next = id + 1;
        id
    })
}

pub fn make_node(text: &str, rval: RadVal) -> RadNode {
    RadNode {
        text: text.to_string(),
        rval,
        id: gen_id(),
    }
}

pub fn make_nil() -> RadNode {
    make_node("", RadVal::Nil)
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

pub fn starting_token(rval: &RadVal) -> Option<&'static str> {
    match rval {
        RadVal::List(_) => Some("("),
        RadVal::Array(_) => Some("["),
        RadVal::Map(_) => Some("{"),
        _ => None,
    }
}

pub fn is_map(listchar: &str) -> bool {
    listchar == "{"
}

pub fn make_list_val(listchar: &str, items: RadList) -> io::Result<RadVal> {
    match listchar {
        "(" => Ok(RadVal::List(items)),
        "[" => Ok(RadVal::Array(items)),
        _ => {
            let txt = format!("Not a valid listchar: {}", listchar);
            Err(error_invalid_data(txt.as_str()))
        },
    }
}

pub fn make_map_val(mapchar: &str, items: RadList) -> io::Result<RadVal> {
    match mapchar {
        "{" => Ok(RadVal::Map(list_to_map(items))),
        _ => {
            let txt = format!("Not a valid mapchar: {}", mapchar);
            Err(error_invalid_data(txt.as_str()))
        },
    }
}

pub fn list_to_map(list: RadList) -> RadMap {
    let mut map = RadMap::new();
    let mut i = 0;
    loop {
        if i >= list.len() {return map};
        let key = list[i].clone();
        if i+1 >= list.len() {
            map.insert(key, make_nil());
        } else {
            let val = list[i+1].clone();
            map.insert(key, val);
        }
        i += 2;
    }
}

pub fn map_to_list(map: &RadMap) -> RadList {
    map.iter().flat_map(|(k, v)| vec![k.clone(), v.clone()]).collect()
}

pub fn is_ending_token(token: &str) -> bool {
    match token {
        ")" | "]" | "}" => true,
        _ => false
    }
}

pub fn make_quote_val(quotechar: &str, form: RadNode) -> io::Result<RadVal> {
    let form = Box::new(form);
    match quotechar {
        "`" => Ok(RadVal::Quasiquote(form)),
        "'" => Ok(RadVal::Quote(form)),
        "~" => Ok(RadVal::Unquote(form)),
        "~@" => Ok(RadVal::SpliceUnquote(form)),
        "@" => Ok(RadVal::Deref(form)),
        _ => {
            let txt = format!("Not a valid quotechar: {}", quotechar);
            Err(error_invalid_data(txt.as_str()))
        },
    }
}

pub fn make_meta_val(meta: RadNode, form: RadNode) -> RadVal {
    RadVal::WithMeta(Box::new(meta), Box::new(form))
}

pub fn quote_word(starting: &RadVal) -> Option<&'static str> {
    match starting {
        RadVal::Quasiquote(_) => Some("quasiquote"),
        RadVal::Quote(_) => Some("quote"),
        RadVal::Unquote(_) => Some("unquote"),
        RadVal::SpliceUnquote(_) => Some("splice-unquote"),
        RadVal::Deref(_) => Some("deref"),
        _ => None,
    }
}

pub fn error_eof(text: &str) -> io::Error {
    error(text, UnexpectedEof)
}

pub fn error_invalid_data(text: &str) -> io::Error {
    error(text, InvalidData)
}

pub fn error(text: &str, kind: io::ErrorKind) -> io::Error {
    io::Error::new(kind, text.to_string())
}
