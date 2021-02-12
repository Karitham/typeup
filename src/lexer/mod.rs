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
    DoubleDot,
    Slash,
    Underscore,
    Exclamation,
    Backtick,
    Newline,
    Eof,
    Whitespace(char),
    Char(char),
}

fn tok(c: char) -> Token {
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
        ':' => Token::DoubleDot,
        '/' => Token::Slash,
        '_' => Token::Underscore,
        '`' => Token::Backtick,
        '\n' => Token::Newline,
        '!' => Token::Exclamation,
        '\r' | '\t' | ' ' => Token::Whitespace(c),
        _ => Token::Char(c),
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Position {
    line: u32,
    index: u32,
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

pub fn tokenize(s: String) -> Vec<(Token, Position)> {
    let mut index: u32 = 0;
    let mut line: u32 = 0;

    s.chars()
        .map(|c| {
            index += 1;
            let t = tok(c);
            match t {
                Token::Newline => {
                    index = 0;
                    line += 1;
                    (t, Position { index, line })
                }
                _ => (t, Position { index, line }),
            }
        })
        .collect()
}
