use crate::lexer;

#[derive(Debug)]
pub struct Reader {
    doc: Vec<(lexer::Token, lexer::Position)>,
    off: usize,
}

/// Easy usage, just call Reader::from(your_text) to build a reader
impl From<String> for Reader {
    fn from(s: String) -> Self {
        Self::new(lexer::tokenize(&s))
    }
}

impl Reader {
    pub(crate) fn new(doc: Vec<(lexer::Token, lexer::Position)>) -> Self {
        Self { doc, off: 0 }
    }

    /// peek returns the next token
    pub(crate) fn peek(&mut self) -> &lexer::Token {
        match self.doc.get(self.off + 1) {
            Some((t, _)) => t,
            None => &lexer::Token::Eof,
        }
    }

    /// token returns the current token
    pub(crate) fn token(&self) -> &lexer::Token {
        match self.doc.get(self.off) {
            Some((t, _)) => t,
            None => &lexer::Token::Eof,
        }
    }

    /// position returns the current position
    pub(crate) fn position(&self) -> lexer::Position {
        match self.doc.get(self.off) {
            Some(&(_, p)) => p,
            None => lexer::Position { line: 0, index: 0 },
        }
    }

    /// advance_by advances the reader
    pub(crate) fn advance_by(&mut self, amount: usize) {
        self.off += amount;
    }

    /// get_while returns an iterator over the elements that match the conditional
    pub(crate) fn get_while<F>(
        &mut self,
        cond: F,
    ) -> std::slice::Iter<(lexer::Token, lexer::Position)>
    where
        F: Fn(&lexer::Token) -> bool,
    {
        let start = self.off;

        while cond(&self.doc[self.off].0) {
            self.advance_by(1);
        }

        self.doc[start..self.off].iter()
    }
}
