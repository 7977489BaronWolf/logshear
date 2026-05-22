//! Query AST optimizer — performs constant folding and predicate simplification
//! before evaluation to reduce redundant work on large log streams.

use crate::query::ast::{Expr, BinaryOp, UnaryOp};

/// Optimize an expression tree, returning a semantically equivalent but
/// potentially simpler tree.
pub fn optimize(expr: Expr) -> Expr {
    match expr {
        // Recurse into binary expressions first, then apply rules.
        Expr::BinaryOp { op, left, right } => {
            let left = optimize(*left);
            let right = optimize(*right);
            fold_binary(op, left, right)
        }

        // Recurse into unary expressions.
        Expr::UnaryOp { op, operand } => {
            let operand = optimize(*operand);
            fold_unary(op, operand)
        }

        // Leaf nodes are already in their simplest form.
        other => other,
    }
}

fn fold_binary(op: BinaryOp, left: Expr, right: Expr) -> Expr {
    match (&op, &left, &right) {
        // AND short-circuits: false AND x => false
        (BinaryOp::And, Expr::Literal(crate::query::ast::Value::Bool(false)), _) => {
            Expr::Literal(crate::query::ast::Value::Bool(false))
        }
        // AND identity: true AND x => x
        (BinaryOp::And, Expr::Literal(crate::query::ast::Value::Bool(true)), _) => right,
        // AND identity: x AND true => x
        (BinaryOp::And, _, Expr::Literal(crate::query::ast::Value::Bool(true))) => left,

        // OR short-circuits: true OR x => true
        (BinaryOp::Or, Expr::Literal(crate::query::ast::Value::Bool(true)), _) => {
            Expr::Literal(crate::query::ast::Value::Bool(true))
        }
        // OR identity: false OR x => x
        (BinaryOp::Or, Expr::Literal(crate::query::ast::Value::Bool(false)), _) => right,
        // OR identity: x OR false => x
        (BinaryOp::Or, _, Expr::Literal(crate::query::ast::Value::Bool(false))) => left,

        // No simplification possible — rebuild the node.
        _ => Expr::BinaryOp {
            op,
            left: Box::new(left),
            right: Box::new(right),
        },
    }
}

fn fold_unary(op: UnaryOp, operand: Expr) -> Expr {
    match (&op, &operand) {
        // NOT true => false
        (UnaryOp::Not, Expr::Literal(crate::query::ast::Value::Bool(true))) => {
            Expr::Literal(crate::query::ast::Value::Bool(false))
        }
        // NOT false => true
        (UnaryOp::Not, Expr::Literal(crate::query::ast::Value::Bool(false))) => {
            Expr::Literal(crate::query::ast::Value::Bool(true))
        }
        _ => Expr::UnaryOp {
            op,
            operand: Box::new(operand),
        },
    }
}
