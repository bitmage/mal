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
    String,
    Symbol,
    Number,
    Char,
}

impl fmt::Display for RadNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.rtype {
            RadType::List => {
                let inner: Vec<String> = self.args.iter()
                    .map(|a| format!("{}", a))
                    .collect();
                write!(f, "({})", inner.join(" "))
            },
            RadType::String => write!(f, "\"{}\"", self.text),
            RadType::Symbol => write!(f, "{}", self.text),
            RadType::Number => write!(f, "{}", self.text),
            RadType::Char =>   write!(f, "{}", self.text),
        }
    }
}
