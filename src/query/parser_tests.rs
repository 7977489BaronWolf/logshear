#[cfg(test)]
mod tests {
    use crate::query::parser::{Expr, Op, ParseError, Parser, Value};

    fn parse(input: &str) -> Result<Expr, ParseError> {
        Parser::new(input).parse()
    }

    #[test]
    fn test_simple_eq() {
        let expr = parse("level = \"error\"").unwrap();
        assert_eq!(
            expr,
            Expr::Field("level".into(), Op::Eq, Value::Str("error".into()))
        );
    }

    #[test]
    fn test_numeric_comparison() {
        let expr = parse("status >= 500").unwrap();
        assert_eq!(
            expr,
            Expr::Field("status".into(), Op::Gte, Value::Num(500.0))
        );
    }

    #[test]
    fn test_and_expression() {
        let expr = parse("level = \"error\" AND service = \"api\"").unwrap();
        assert_eq!(
            expr,
            Expr::And(
                Box::new(Expr::Field("level".into(), Op::Eq, Value::Str("error".into()))),
                Box::new(Expr::Field("service".into(), Op::Eq, Value::Str("api".into())))
            )
        );
    }

    #[test]
    fn test_or_expression() {
        let expr = parse("level = \"warn\" OR level = \"error\"").unwrap();
        assert!(matches!(expr, Expr::Or(_, _)));
    }

    #[test]
    fn test_not_expression() {
        let expr = parse("NOT level = \"debug\"").unwrap();
        assert!(matches!(expr, Expr::Not(_)));
    }

    #[test]
    fn test_parentheses() {
        let expr = parse("(level = \"error\") AND service = \"api\"").unwrap();
        assert!(matches!(expr, Expr::And(_, _)));
    }

    #[test]
    fn test_free_text() {
        let expr = parse("timeout").unwrap();
        assert_eq!(expr, Expr::FreeText("timeout".into()));
    }

    #[test]
    fn test_contains_op() {
        let expr = parse("message ~ \"error\"").unwrap();
        assert_eq!(
            expr,
            Expr::Field("message".into(), Op::Contains, Value::Str("error".into()))
        );
    }

    #[test]
    fn test_bool_value() {
        let expr = parse("active = true").unwrap();
        assert_eq!(
            expr,
            Expr::Field("active".into(), Op::Eq, Value::Bool(true))
        );
    }

    #[test]
    fn test_nested_logic() {
        let expr =
            parse("(level = \"error\" OR level = \"warn\") AND service = \"api\"").unwrap();
        assert!(matches!(expr, Expr::And(_, _)));
        if let Expr::And(left, _) = expr {
            assert!(matches!(*left, Expr::Or(_, _)));
        }
    }

    #[test]
    fn test_invalid_expression() {
        let result = parse("level =");
        assert!(result.is_err());
    }
}
