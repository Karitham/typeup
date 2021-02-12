use crate::parser;

pub trait Node {
    fn content(&self) -> Content;
    fn kind(&self) -> Kind;
    fn parse(&self, r: parser::Reader) -> Self;
}

impl Kind {}

pub enum Kind {
    H1(Content),
    H2(Content),
    H3(Content),
    H4(Content),
    H5(Content),
    H6(Content),
    Code(Content),
    InlineCode(Content),
    UnorderedList(Content),
    OrderedList(Content),
    Quote(Content),
    Italic(Content),
    Bold(Content),
    Image(Content),
    Link(Content),
    Embed(Content),
}

pub enum Content {
    Text(String),
    Table(Vec<Vec<String>>),
    Image((String, String)),
    Link((String, String)),
    Embed((String, String)),
    List(Vec<String>),
}
