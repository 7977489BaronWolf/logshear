#[cfg(test)]
mod tests {
    use crate::query::ast::{BinaryOp, Expr, UnaryOp, Value};
    use crate::query::optimizer::optimize;

    fn lit_bool(b: bool) -> Expr {
        Expr::Literal(Value::Bool(b))
    }

    fn field(name: &str) -> Expr {
        Expr::Field(name.to_string())
    }

    fn and(l: Expr, r: Expr) -> Expr {
        Expr::BinaryOp {
            op: BinaryOp::And,
            left: Box::new(l),
            right: Box::new(r),
        }
    }

    fn or(l: Expr, r: Expr) -> Expr {
        Expr::BinaryOp {
            op: BinaryOp::Or,
            left: Box::new(l),
            right: Box::new(r),
        }
    }

    fn not(e: Expr) -> Expr {
        Expr::UnaryOp {
            op: UnaryOp::Not,
            operand: Box::new(e),
        }
    }

    #[test]
    fn and_false_short_circuits() {
        let expr = and(lit_bool(false), field("level"));
        assert_eq!(optimize(expr), lit_bool(false));
    }

    #[test]
    fn and_true_identity_left() {
        let expr = and(lit_bool(true), field("level"));
        assert_eq!(optimize(expr), field("level"));
    }

    #[test]
    fn and_true_identity_right() {
        let expr = and(field("level"), lit_bool(true));
        assert_eq!(optimize(expr), field("level"));
    }

    #[test]
    fn or_true_short_circuits() {
        let expr = or(lit_bool(true), field("level"));
        assert_eq!(optimize(expr), lit_bool(true));
    }

    #[test]
    fn or_false_identity_left() {
        let expr = or(lit_bool(false), field("level"));
        assert_eq!(optimize(expr), field("level"));
    }

    #[test]
    fn or_false_identity_right() {
        let expr = or(field("level"), lit_bool(false));
        assert_eq!(optimize(expr), field("level"));
    }

    #[test]
    fn not_true_folds_to_false() {
        assert_eq!(optimize(not(lit_bool(true))), lit_bool(false));
    }

    #[test]
    fn not_false_folds_to_true() {
        assert_eq!(optimize(not(lit_bool(false))), lit_bool(true));
    }

    #[test]
    fn nested_optimization_propagates() {
        // (true AND false) OR field  =>  false OR field  =>  field
        let expr = or(and(lit_bool(true), lit_bool(false)), field("msg"));
        assert_eq!(optimize(expr), field("msg"));
    }

    #[test]
    fn non_constant_expr_unchanged() {
        let expr = and(field("a"), field("b"));
        let expected = and(field("a"), field("b"));
        assert_eq!(optimize(expr), expected);
    }
}
