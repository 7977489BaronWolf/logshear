#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::query::ast::{CmpOp, Expr, Value};
    use crate::query::eval::eval;

    fn fields(pairs: &[(&str, &str)]) -> HashMap<String, String> {
        pairs.iter().map(|(k, v)| (k.to_string(), v.to_string())).collect()
    }

    #[test]
    fn wildcard_always_matches() {
        assert!(eval(&Expr::Wildcard, "anything", &HashMap::new()));
    }

    #[test]
    fn contains_match() {
        assert!(eval(&Expr::Contains("error".into()), "fatal error occurred", &HashMap::new()));
        assert!(!eval(&Expr::Contains("warn".into()), "fatal error occurred", &HashMap::new()));
    }

    #[test]
    fn field_eq_string() {
        let f = fields(&[("level", "error")]);
        assert!(eval(&Expr::FieldEq("level".into(), Value::Str("error".into())), "", &f));
        assert!(!eval(&Expr::FieldEq("level".into(), Value::Str("warn".into())), "", &f));
    }

    #[test]
    fn field_eq_missing_key() {
        let f = fields(&[]);
        assert!(!eval(&Expr::FieldEq("level".into(), Value::Str("error".into())), "", &f));
    }

    #[test]
    fn field_cmp_int_gt() {
        let f = fields(&[("status", "500")]);
        let expr = Expr::FieldCmp("status".into(), CmpOp::Gt, Value::Int(400));
        assert!(eval(&expr, "", &f));
        let expr2 = Expr::FieldCmp("status".into(), CmpOp::Gt, Value::Int(500));
        assert!(!eval(&expr2, "", &f));
    }

    #[test]
    fn field_cmp_float_lte() {
        let f = fields(&[("latency", "1.5")]);
        let expr = Expr::FieldCmp("latency".into(), CmpOp::Lte, Value::Float(2.0));
        assert!(eval(&expr, "", &f));
    }

    #[test]
    fn logical_and() {
        let f = fields(&[("level", "error"), ("status", "500")]);
        let expr = Expr::And(
            Box::new(Expr::FieldEq("level".into(), Value::Str("error".into()))),
            Box::new(Expr::FieldCmp("status".into(), CmpOp::Gte, Value::Int(500))),
        );
        assert!(eval(&expr, "", &f));
    }

    #[test]
    fn logical_or_one_false() {
        let f = fields(&[("level", "warn")]);
        let expr = Expr::Or(
            Box::new(Expr::FieldEq("level".into(), Value::Str("error".into()))),
            Box::new(Expr::FieldEq("level".into(), Value::Str("warn".into()))),
        );
        assert!(eval(&expr, "", &f));
    }

    #[test]
    fn logical_not() {
        let f = fields(&[("level", "info")]);
        let expr = Expr::Not(Box::new(Expr::FieldEq("level".into(), Value::Str("error".into()))));
        assert!(eval(&expr, "", &f));
    }
}
