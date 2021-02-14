#[derive(Debug, PartialEq)]
pub enum Token {
    NumberSign,
    Equal,
    Star,
    LSquareBracket,
    RSquareBracket,
    LCurlyBracket,
    RCurlyBracket,
    Pipe,
    Dash,
    Colon,
    Slash,
    Underscore,
    Exclamation,
    Backtick,
    Newline,
    Eof,
    Whitespace(char),
    Char(char),
}

impl From<char> for Token {
    fn from(c: char) -> Self {
        match c {
            '#' => Token::NumberSign,
            '*' => Token::Star,
            '=' => Token::Equal,
            '[' => Token::LSquareBracket,
            ']' => Token::RSquareBracket,
            '{' => Token::LCurlyBracket,
            '}' => Token::RCurlyBracket,
            '|' => Token::Pipe,
            '-' => Token::Dash,
            ':' => Token::Colon,
            '/' => Token::Slash,
            '_' => Token::Underscore,
            '`' => Token::Backtick,
            '\n' => Token::Newline,
            '!' => Token::Exclamation,
            '\r' | '\t' | ' ' => Token::Whitespace(c),
            _ => Token::Char(c),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    pub line: u32,
    pub index: u32,
}

impl Position {
    pub fn new(line: u32, index: u32) -> Self {
        Self { line, index }
    }
}

impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[{}, {}]", self.line, self.index)
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

pub fn tokenize(s: &String) -> Vec<(Token, Position)> {
    let mut index = 1;
    let mut line = 1;

    let mut tokens: Vec<(Token, Position)> = s
        .chars()
        .map(|c| {
            let tok = Token::from(c);
            match tok {
                Token::Newline => {
                    let new_lined = (tok, Position::new(line, index));
                    index = 1;
                    line += 1;

                    new_lined
                }
                _ => {
                    let token = (tok, Position::new(line, index));
                    index += 1;

                    token
                }
            }
        })
        .collect();

    tokens.push((Token::Eof, Position::new(line, index)));

    tokens
}
