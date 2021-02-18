#[derive(Debug, PartialEq, Eq)]
pub enum Kind<'a> {
    H1(&'a str),
    H2(&'a str),
    H3(&'a str),
    H4(&'a str),
    H5(&'a str),
    H6(&'a str),
    Title(&'a str),
    Code(&'a str),
    InlineCode(&'a str),
    UnorderedList(Vec<&'a str>),
    OrderedList(Vec<&'a str>),
    Quote((&'a str, &'a str)),
    Italic(&'a str),
    Bold(&'a str),
    Image((&'a str, &'a str)),
    Link((&'a str, &'a str)),
    Text(&'a str),
}

#[derive(Debug)]
pub struct Lexer<'a> {
    s: &'a str,
    pos: usize,
    off: usize,
    cursor: Position,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseError<'a> {
    message: &'a str,
    location: Position,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    pub line: usize,
    pub index: usize,
}

impl Position {
    pub fn new(line: usize, index: usize) -> Self {
        Self { line, index }
    }
}

impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[{}, {}]", self.line, self.index)
    }
}

impl<'a> ParseError<'a> {
    pub fn new(message: &'a str, location: Position) -> Self {
        Self { message, location }
    }
}

impl<'a> std::fmt::Display for ParseError<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "error: {} at: {}", self.message, self.location)
    }
}

impl<'a> Lexer<'a> {
    pub fn new(s: &'a str) -> Self {
        Self {
            s,
            pos: 0,
            off: 0,
            cursor: Position { line: 1, index: 1 },
        }
    }
    pub fn parse_header(&'a mut self) -> Result<Kind<'a>, ParseError> {
        let count = self.eat_while(|b| *b == b'#');
        self.eat_whitespace();

        match count {
            1 => Ok(Kind::H1(self.get_same_line())),
            2 => Ok(Kind::H2(self.get_same_line())),
            3 => Ok(Kind::H3(self.get_same_line())),
            4 => Ok(Kind::H4(self.get_same_line())),
            5 => Ok(Kind::H5(self.get_same_line())),
            6 => Ok(Kind::H6(self.get_same_line())),
            _ => Err(ParseError::new(
                "invalid header format",
                Position {
                    line: self.cursor.line,
                    index: 7,
                },
            )),
        }
    }

    pub fn parse_title(&'a mut self) -> Result<Kind<'a>, ParseError> {
        if self.s[self.pos..].starts_with(syntax::TITLE) {
            self.off += syntax::TITLE.len();
            self.move_up_to_offset();
            self.eat_whitespace();
            Ok(Kind::Title(self.get_same_line()))
        } else {
            Err(ParseError::new("invalid title format", self.cursor))
        }
    }

    /// move_up_to_offset moves the position to the offset.
    fn move_up_to_offset(&mut self) {
        self.pos += self.off;
        self.off = 0;
    }

    /// keep cursor in sync moves the cursor
    fn keep_cursor_in_sync(&mut self) {
        if util::is_not_newline(self.peek().unwrap()) {
            self.cursor.index += 1
        } else {
            self.newline_cursor()
        }
    }

    /// newline_cursor moves the cursor to the newline
    fn newline_cursor(&mut self) {
        self.cursor.line += 1;
        self.cursor.index = 1;
    }

    /// get_while returns a &str containing the elements it iterated over
    /// it also moves the cursor with an index.
    fn get_while<F>(&mut self, cond: F) -> &'a str
    where
        F: Fn(&u8) -> bool,
    {
        while cond(&self.s.as_bytes()[self.pos + self.off]) {
            self.off += 1;
            self.keep_cursor_in_sync();
        }
        &self.s[self.slice_and_move()]
    }

    /// eat_while advances while it matches the condition.
    /// Once the condition is not matched, it returns how much it
    /// has eaten.
    fn eat_while<F>(&mut self, cond: F) -> usize
    where
        F: Fn(&u8) -> bool,
    {
        while cond(&self.s.as_bytes()[self.pos + self.off]) {
            self.off += 1;
            self.keep_cursor_in_sync();
        }
        self.slice_and_move().count()
    }

    /// peek returns the byte you're currently looking at
    /// it counting the offset
    fn peek(&self) -> Option<&u8> {
        self.s.as_bytes().get(self.pos + self.off)
    }

    /// slice_and_move return a range to slice the string for.
    /// it moves the pos & offset
    fn slice_and_move(&mut self) -> std::ops::Range<usize> {
        let r = self.pos..self.pos + self.off;
        self.move_up_to_offset();
        r
    }

    /// eat_whitespace is a wrapper around eat_while for whitespace
    fn eat_whitespace(&mut self) -> usize {
        self.eat_while(util::is_whitespace)
    }

    /// get_same_line returns the text until it meets a newline
    fn get_same_line(&mut self) -> &'a str {
        self.newline_cursor();
        self.get_while(util::is_not_newline)
    }
}

mod util {
    pub(crate) fn is_not_newline(b: &u8) -> bool {
        *b != b'\n'
    }
    pub(crate) fn is_whitespace(b: &u8) -> bool {
        *b == b'\r' || *b == b' ' || *b == b'\t'
    }
}

mod syntax {
    pub(crate) const TITLE: &str = "=#";
}
