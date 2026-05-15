use crate::query::ast::{CmpOp, Expr, Value};
use std::collections::HashMap;

/// Evaluate a query expression against a log line and its extracted fields.
///
/// `line`   — the raw log line text  
/// `fields` — key/value pairs extracted from the line (e.g. from JSON or kv parsing)
pub fn eval(expr: &Expr, line: &str, fields: &HashMap<String, String>) -> bool {
    match expr {
        Expr::Wildcard => true,

        Expr::Contains(s) => line.contains(s.as_str()),

        Expr::FieldEq(key, val) => {
            match fields.get(key) {
                None => false,
                Some(fv) => match val {
                    Value::Str(s)   => fv == s,
                    Value::Bool(b)  => fv.parse::<bool>().ok().as_ref() == Some(b),
                    Value::Int(i)   => fv.parse::<i64>().ok().as_ref() == Some(i),
                    Value::Float(v) => fv.parse::<f64>().ok().map(|f| f == *v).unwrap_or(false),
                },
            }
        }

        Expr::FieldCmp(key, op, val) => {
            match fields.get(key) {
                None => false,
                Some(fv) => cmp_field(fv, op, val),
            }
        }

        Expr::And(l, r) => eval(l, line, fields) && eval(r, line, fields),
        Expr::Or(l, r)  => eval(l, line, fields) || eval(r, line, fields),
        Expr::Not(e)    => !eval(e, line, fields),
    }
}

fn cmp_field(field_val: &str, op: &CmpOp, val: &Value) -> bool {
    match val {
        Value::Int(i) => {
            if let Ok(fv) = field_val.parse::<i64>() {
                apply_cmp(fv, op, *i)
            } else {
                false
            }
        }
        Value::Float(v) => {
            if let Ok(fv) = field_val.parse::<f64>() {
                apply_cmp_f(fv, op, *v)
            } else {
                false
            }
        }
        Value::Str(s) => match op {
            CmpOp::Neq => field_val != s.as_str(),
            _          => false, // ordering on arbitrary strings not supported
        },
        Value::Bool(_) => false,
    }
}

fn apply_cmp<T: PartialOrd>(a: T, op: &CmpOp, b: T) -> bool {
    match op {
        CmpOp::Gt  => a > b,
        CmpOp::Lt  => a < b,
        CmpOp::Gte => a >= b,
        CmpOp::Lte => a <= b,
        CmpOp::Neq => a != b,
    }
}

fn apply_cmp_f(a: f64, op: &CmpOp, b: f64) -> bool {
    apply_cmp(a, op, b)
}
