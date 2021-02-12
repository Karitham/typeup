use crate::lexer;

#[derive(Debug)]
pub struct ParseError {
    message: String,
    location: lexer::Position,
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "error: {} at: {}", self.message, self.location)
    }
}

pub struct Reader<'a> {
    doc: &'a Vec<(lexer::Token, lexer::Position)>,
    off: usize,
}

impl<'a> Reader<'a> {
    /// count_following_token returns the number of tokens that pass the filter
    fn count_following_token(&mut self, filter: fn(&lexer::Token) -> bool) -> usize {
        self.doc[self.off..]
            .iter()
            .take_while(|(t, _)| filter(t))
            .count()
    }

    /// peek_next_token returns the next token
    fn peek_next_token(&mut self) -> &lexer::Token {
        match self.doc.get(self.off + 1) {
            Some((t, _)) => t,
            None => &lexer::Token::Eof,
        }
    }
    /// advance_by advances the reader
    fn advance_by(&mut self, amount: usize) {
        self.off += amount;
    }

    /// get_while calls the conditional while it returns something and appends it to the currend string
    /// returning the string once it doesn't fit
    fn get_while(&mut self, cond: fn(&lexer::Token) -> Option<char>) -> String {
        let mut string = String::new();

        while let Some(c) = cond(match self.doc.get(self.off) {
            Some((t, _)) => t,
            None => return string,
        }) {
            string.push(c);
            self.advance_by(1);
        }

        string
    }
}
