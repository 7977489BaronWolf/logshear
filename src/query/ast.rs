/// Represents a parsed query expression tree
#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    /// Match a literal string anywhere in the log line
    Contains(String),
    /// Match a field value: `field:value`
    FieldEq(String, Value),
    /// Match a field with a comparison operator: `field>5`
    FieldCmp(String, CmpOp, Value),
    /// Logical AND of two expressions
    And(Box<Expr>, Box<Expr>),
    /// Logical OR of two expressions
    Or(Box<Expr>, Box<Expr>),
    /// Logical NOT of an expression
    Not(Box<Expr>),
    /// Wildcard — match everything
    Wildcard,
}

impl Expr {
    /// Returns `true` if this expression is a terminal (leaf) node,
    /// i.e. it does not contain any sub-expressions.
    pub fn is_leaf(&self) -> bool {
        matches!(self, Expr::Contains(_) | Expr::FieldEq(_, _) | Expr::FieldCmp(_, _, _) | Expr::Wildcard)
    }

    /// Wraps this expression in a `Not`, unless it is already a `Not`,
    /// in which case the inner expression is returned (double-negation elimination).
    pub fn negate(self) -> Expr {
        match self {
            Expr::Not(inner) => *inner,
            other => Expr::Not(Box::new(other)),
        }
    }
}

/// A value used in field comparisons
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Str(String),
    Int(i64),
    Float(f64),
    Bool(bool),
}

/// Comparison operators for numeric/string field filtering
#[derive(Debug, Clone, PartialEq)]
pub enum CmpOp {
    Gt,
    Lt,
    Gte,
    Lte,
    Neq,
}

impl std::fmt::Display for CmpOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CmpOp::Gt  => write!(f, ">"),
            CmpOp::Lt  => write!(f, "<"),
            CmpOp::Gte => write!(f, ">="),
            CmpOp::Lte => write!(f, "<="),
            CmpOp::Neq => write!(f, "!="),
        }
    }
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Str(s)   => write!(f, "{}", s),
            Value::Int(i)   => write!(f, "{}", i),
            Value::Float(v) => write!(f, "{}", v),
            Value::Bool(b)  => write!(f, "{}", b),
        }
    }
}

impl std::fmt::Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Contains(s)         => write!(f, "\"{}\"" , s),
            Expr::FieldEq(k, v)       => write!(f, "{}:{}", k, v),
            Expr::FieldCmp(k, op, v)  => write!(f, "{}{}{}", k, op, v),
            Expr::And(l, r)           => write!(f, "({} AND {})", l, r),
            Expr::Or(l, r)            => write!(f, "({} OR {})", l, r),
            Expr::Not(e)              => write!(f, "NOT {}", e),
            Expr::Wildcard            => write!(f, "*"),
        }
    }
}
