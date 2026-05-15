#[cfg(test)]
mod tests {
    use super::super::lexer::{Lexer, Token};

    fn lex(input: &str) -> Vec<Token> {
        Lexer::new(input).tokenize().expect("lex failed")
    }

    #[test]
    fn test_empty_input() {
        let tokens = lex("");
        assert_eq!(tokens, vec![Token::Eof]);
    }

    #[test]
    fn test_simple_eq() {
        let tokens = lex("level == \"error\"");
        assert_eq!(
            tokens,
            vec![
                Token::Ident("level".into()),
                Token::Eq,
                Token::StringLit("error".into()),
                Token::Eof,
            ]
        );
    }

    #[test]
    fn test_not_eq() {
        let tokens = lex("status != 200");
        assert_eq!(
            tokens,
            vec![
                Token::Ident("status".into()),
                Token::NotEq,
                Token::NumberLit(200.0),
                Token::Eof,
            ]
        );
    }

    #[test]
    fn test_logical_and_or() {
        let tokens = lex("a and b or c");
        assert_eq!(
            tokens,
            vec![
                Token::Ident("a".into()),
                Token::And,
                Token::Ident("b".into()),
                Token::Or,
                Token::Ident("c".into()),
                Token::Eof,
            ]
        );
    }

    #[test]
    fn test_contains_keyword() {
        let tokens = lex("message contains \"timeout\"");
        assert_eq!(
            tokens,
            vec![
                Token::Ident("message".into()),
                Token::Contains,
                Token::StringLit("timeout".into()),
                Token::Eof,
            ]
        );
    }

    #[test]
    fn test_nested_parens() {
        let tokens = lex("(a and b)");
        assert_eq!(
            tokens,
            vec![
                Token::LParen,
                Token::Ident("a".into()),
                Token::And,
                Token::Ident("b".into()),
                Token::RParen,
                Token::Eof,
            ]
        );
    }

    #[test]
    fn test_dotted_field() {
        let tokens = lex("http.status == 404");
        assert_eq!(
            tokens,
            vec![
                Token::Ident("http.status".into()),
                Token::Eq,
                Token::NumberLit(404.0),
                Token::Eof,
            ]
        );
    }

    #[test]
    fn test_escaped_string() {
        let tokens = lex(r#""he said \"hi\""");
        assert_eq!(
            tokens,
            vec![
                Token::StringLit("he said \"hi\"".into()),
                Token::Eof,
            ]
        );
    }

    #[test]
    fn test_unexpected_char_errors() {
        let result = Lexer::new("@bad").tokenize();
        assert!(result.is_err());
    }
}
