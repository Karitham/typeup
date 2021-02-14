use crate::lexer::{self, Token};
use crate::reader;

#[derive(Debug)]
pub enum Block {
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
    Text(Content),
    Root,
}

#[derive(Debug)]
pub enum Content {
    Text(String),
    Table(Vec<Vec<String>>),
    TextAndLink((String, String)),
    List(Vec<String>),
}

#[derive(Debug)]
pub struct ParseError {
    message: String,
    location: lexer::Position,
}

impl ParseError {
    pub fn new(message: String, location: lexer::Position) -> Self {
        Self { message, location }
    }
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "error: {} at: {}", self.message, self.location)
    }
}

/// Node is a node in an AST
#[derive(Debug)]
pub struct Node {
    node: Vec<Node>,
    block: Block,
}

impl Node {
    pub(crate) fn new(b: Block) -> Self {
        Self {
            node: Vec::new(),
            block: b,
        }
    }
}

impl reader::Reader {
    pub fn parse(&mut self) -> (Node, Vec<ParseError>) {
        let mut root_node = Node::new(Block::Root);

        let mut errors: Vec<ParseError> = Vec::new();
        let curr_node = &mut root_node;
        loop {
            match self.token() {
                Token::NumberSign => {}
                Token::Equal => {}
                Token::Star => {}
                Token::LSquareBracket => {}
                Token::RSquareBracket => {}
                Token::LCurlyBracket => {}
                Token::RCurlyBracket => {}
                Token::Pipe => {}
                Token::Dash => {}
                Token::Colon => {}
                Token::Slash => {}
                Token::Underscore => {}
                Token::Exclamation => {}
                Token::Backtick => {}
                Token::Newline => {
                    let b = self.block_on_newline();
                    curr_node.node.push(Node::new(b.0));
                    match b.1 {
                        Some(e) => errors.push(e),
                        None => {}
                    }
                }
                Token::Eof => {
                    break;
                }
                Token::Whitespace(_) => {}
                Token::Char(_) => {}
            }
            self.advance_by(1)
        }

        (root_node, errors)
    }

    fn header(&mut self) -> Result<Block, ParseError> {
        match self.get_while(|t| &Token::NumberSign == t).count() {
            1 => Ok(Block::H1(self.get_same_line_text())),
            2 => Ok(Block::H2(self.get_same_line_text())),
            3 => Ok(Block::H3(self.get_same_line_text())),
            4 => Ok(Block::H4(self.get_same_line_text())),
            5 => Ok(Block::H5(self.get_same_line_text())),
            6 => Ok(Block::H6(self.get_same_line_text())),
            _ => Err(ParseError::new(
                String::from("Invalid header format"),
                self.position(),
            )),
        }
    }

    fn code_block(&mut self) -> Result<Block, ParseError> {
        Err(ParseError::new(
            String::from("Not Implemented"),
            lexer::Position { index: 0, line: 0 },
        ))
    }

    fn get_same_line_text(&mut self) -> Content {
        Content::Text(
            self.get_while(|t| -> bool {
                match t {
                    Token::Newline | Token::Eof => false,
                    _ => true,
                }
            })
            .map(|(t, _)| t.to_string())
            .collect(),
        )
    }
    fn block_on_newline(&mut self) -> (Block, Option<ParseError>) {
        self.advance_by(1);
        match self.token() {
            &Token::NumberSign => match self.header() {
                Ok(b) => (b, None),
                Err(e) => (Block::H6(self.get_same_line_text()), Some(e)),
            },
            &Token::Backtick => match self.code_block() {
                Ok(b) => (b, None),
                Err(e) => (Block::H6(self.get_same_line_text()), Some(e)),
            },
            _ => (
                Block::Text(self.get_same_line_text()),
                Some(ParseError::new(
                    "Unimplemented".to_string(),
                    self.position(),
                )),
            ),
        }
    }
}
