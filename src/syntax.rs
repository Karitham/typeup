#[derive(Debug, PartialEq, Eq)]
pub enum Kind<'a> {
    Header(Box<Kind<'a>>, u8),
    Title(Box<Kind<'a>>),
    Code(&'a str, &'a str),
    InlineCode(&'a str),
    UnorderedList(Box<Kind<'a>>),
    OrderedList(Box<Kind<'a>>),
    Quote(Box<Kind<'a>>),
    Italic(Box<Kind<'a>>),
    Bold(Box<Kind<'a>>),
    Image((Option<Box<Kind<'a>>>, &'a str)),
    Link(Option<Box<Kind<'a>>>, &'a str),
    Span(Box<Kind<'a>>),
    Str(Box<str>),
    Table(Vec<Vec<&'a str>>),
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseError<'a> {
    message: &'a str,
}

impl<'a> ParseError<'a> {
    pub fn new(message: &'a str) -> Self {
        Self { message }
    }
}

impl<'a> std::fmt::Display for ParseError<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "error: {}", self.message)
    }
}

pub(crate) mod symbols {
    pub const STAR: char = '*';
    pub const UNDERSCORE: char = '_';
}
