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

    #[test]
    fn lex_quote() {
        let text = "| This is a quote";
        let mut l = lexer::Lexer::new(text);
        let k = l.parse_quote().unwrap();
        assert_eq!(k, lexer::Kind::Quote("This is a quote"))
    }

    #[test]
    fn lex_link() {
        // No whitespace
        let text = "[http://nb.kar.wtf]";
        let mut l = lexer::Lexer::new(text);
        let k = l.parse_simple_link().unwrap();
        assert_eq!(k, lexer::Kind::Link(("http://nb.kar.wtf", None)));

        // Whitespace
        let text = "[ http://nb.kar.wtf ]";
        let mut l = lexer::Lexer::new(text);
        let k = l.parse_simple_link().unwrap();
        assert_eq!(k, lexer::Kind::Link(("http://nb.kar.wtf", None)));

        // Error
        let text = "http://nb.kar.wtf";
        let mut l = lexer::Lexer::new(text);
        let k = l.parse_simple_link().unwrap_err();
        assert_eq!(
            k,
            lexer::ParseError::new("Invalid link format", lexer::Position::new(1, 1))
        );
    }

    #[test]
    fn lex_table() {
        let result = lexer::Kind::Table(vec![
            vec!["heading 1", "heading 2", "heading 3"],
            vec!["sub 1", "sub 2", "sub 3"],
        ]);

        // With a comma as the separator
        let text = r###"#,{
        heading 1, heading 2, heading 3
        sub 1, sub 2, sub 3
        }
        "###;
        let mut l = lexer::Lexer::new(text);
        let k = l.parse_table().unwrap();
        assert_eq!(k, result);

        // With 2 pipes
        let text = r###"#||{
        heading 1|| heading 2  || heading 3
        sub 1 || sub 2 || sub 3
        }
        "###;

        let mut l = lexer::Lexer::new(text);
        let k = l.parse_table().unwrap();
        assert_eq!(k, result);
    }
}
