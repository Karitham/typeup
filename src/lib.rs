pub mod lexer;
pub mod syntax;

#[cfg(test)]
mod tests {
    use super::syntax::Kind;
    use crate::lexer::Lexer;

    #[test]
    fn italic_and_bold() {
        let s = "*_THIS IS BOLD AND ITALIC TEXT_*";
        let s2 = String::from("THIS IS BOLD AND ITALIC TEXT");
        let l = Lexer::new(s);

        assert_eq!(
            l.parse(),
            Kind::Bold(Box::new(Kind::Italic(Box::new(Kind::Str(
                s2.clone().into_boxed_str()
            )))))
        );
    }
}
