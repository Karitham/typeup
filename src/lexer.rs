use crate::syntax::{self, Kind};

#[derive(Debug)]
pub struct Lexer<'a> {
    doc: std::str::Chars<'a>,
    root: Vec<Kind<'a>>,
    curr: Option<Kind<'a>>,
    coll: String,
}

impl<'a> Lexer<'a> {
    pub fn new(s: &'a str) -> Self {
        Self {
            doc: s.chars(),
            root: vec![],
            coll: String::new(),
            curr: None,
        }
    }

    pub fn parse(mut self) -> Kind<'a> {
        let c = self.doc.next().unwrap().clone();
        self.parse_char(c);

        self.curr.unwrap()
    }

    fn parse_char(&mut self, ch: char) {
        match ch {
            syntax::symbols::STAR => self.bold(),
            syntax::symbols::UNDERSCORE => self.italic(),
            _ => self.coll.push(ch),
        }
    }

    fn bold(&mut self) {
        self.parse_until_char(syntax::symbols::STAR);
        self.curr = Some(match self.curr.take() {
            Some(k) => Kind::Bold(Box::new(k)),
            None => Kind::Bold(Box::new(Kind::Str(self.coll.clone().into_boxed_str()))),
        });
        self.coll.clear();
    }

    fn italic(&mut self) {
        self.parse_until_char(syntax::symbols::UNDERSCORE);
        self.curr = Some(match self.curr.take() {
            Some(k) => Kind::Italic(Box::new(k)),
            None => Kind::Italic(Box::new(Kind::Str(self.coll.clone().into_boxed_str()))),
        });
        self.coll.clear()
    }

    fn parse_until_char(&mut self, symbol: char) {
        while let Some(ch) = self.doc.next() {
            if ch == symbol {
                break;
            }
            self.parse_char(ch);
        }
    }
}
