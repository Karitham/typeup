pub mod lexer;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lex_h1() {
        let text = "# This is a h1 title\n this is not part of it";
        let mut l = lexer::Lexer::new(text);
        let k = l.parse_header().unwrap();
        assert_eq!(k, lexer::Kind::H1("This is a h1 title"))
    }
    #[test]
    fn lex_h2() {
        let text = "## This is a H2\n Usually, it is under a H1";
        let mut l = lexer::Lexer::new(text);
        let k = l.parse_header().unwrap();
        assert_eq!(k, lexer::Kind::H2("This is a H2"))
    }

    #[test]
    fn lex_h7() {
        let text = "####### This is a H7\n It's not supposed to exist";
        let mut l = lexer::Lexer::new(text);
        let k = l.parse_header().unwrap_err();
        assert_eq!(
            k,
            lexer::ParseError::new("invalid header format", lexer::Position::new(1, 7))
        )
    }

    #[test]
    fn lex_title() {
        let text = "=# This is a document title\nThis is not part of the title anymore";
        let mut l = lexer::Lexer::new(text);
        let k = l.parse_title().unwrap();
        assert_eq!(k, lexer::Kind::Title("This is a document title"))
    }
}
