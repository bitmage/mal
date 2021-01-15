pub type RadLink = Option<Box<RadNode>>;
pub type RadList = Vec<RadNode>;

pub struct RadNode {
    pub src: String,
    pub rtype: RadType,
    //src_line: usize,
    //src_col: usize,
    pub args: RadList,
}

pub enum RadType {
    List,
    RString,
}
