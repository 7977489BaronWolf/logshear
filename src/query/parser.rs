use crate::query::lexer::{Lexer, Token};

#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    Field(String, Op, Value),
    And(Box<Expr>, Box<Expr>),
    Or(Box<Expr>, Box<Expr>),
    Not(Box<Expr>),
    FreeText(String),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Op {
    Eq,
    NotEq,
    Contains,
    StartsWith,
    Gt,
    Lt,
    Gte,
    Lte,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    Str(String),
    Num(f64),
    Bool(bool),
}

#[derive(Debug)]
pub struct ParseError(pub String);

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Parse error: {}", self.0)
    }
}

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(input: &str) -> Self {
        let tokens = Lexer::new(input).collect();
        Parser { tokens, pos: 0 }
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }

    fn advance(&mut self) -> Option<&Token> {
        let tok = self.tokens.get(self.pos);
        self.pos += 1;
        tok
    }

    pub fn parse(&mut self) -> Result<Expr, ParseError> {
        let expr = self.parse_or()?;
        Ok(expr)
    }

    fn parse_or(&mut self) -> Result<Expr, ParseError> {
        let mut left = self.parse_and()?;
        while self.peek() == Some(&Token::Or) {
            self.advance();
            let right = self.parse_and()?;
            left = Expr::Or(Box::new(left), Box::new(right));
        }
        Ok(left)
    }

    fn parse_and(&mut self) -> Result<Expr, ParseError> {
        let mut left = self.parse_unary()?;
        while self.peek() == Some(&Token::And) {
            self.advance();
            let right = self.parse_unary()?;
            left = Expr::And(Box::new(left), Box::new(right));
        }
        Ok(left)
    }

    fn parse_unary(&mut self) -> Result<Expr, ParseError> {
        if self.peek() == Some(&Token::Not) {
            self.advance();
            let expr = self.parse_primary()?;
            return Ok(Expr::Not(Box::new(expr)));
        }
        self.parse_primary()
    }

    fn parse_primary(&mut self) -> Result<Expr, ParseError> {
        if self.peek() == Some(&Token::LParen) {
            self.advance();
            let expr = self.parse_or()?;
            if self.advance() != Some(&Token::RParen) {
                return Err(ParseError("expected closing parenthesis".into()));
            }
            return Ok(expr);
        }

        match self.advance().cloned() {
            Some(Token::Ident(name)) => {
                if let Some(op_tok) = self.peek().cloned() {
                    if let Some(op) = token_to_op(&op_tok) {
                        self.advance();
                        let val = self.parse_value()?;
                        return Ok(Expr::Field(name, op, val));
                    }
                }
                Ok(Expr::FreeText(name))
            }
            Some(Token::Str(s)) => Ok(Expr::FreeText(s)),
            other => Err(ParseError(format!("unexpected token: {:?}", other))),
        }
    }

    fn parse_value(&mut self) -> Result<Value, ParseError> {
        match self.advance().cloned() {
            Some(Token::Str(s)) => Ok(Value::Str(s)),
            Some(Token::Num(n)) => Ok(Value::Num(n)),
            Some(Token::Bool(b)) => Ok(Value::Bool(b)),
            Some(Token::Ident(s)) => Ok(Value::Str(s)),
            other => Err(ParseError(format!("expected value, got: {:?}", other))),
        }
    }
}

fn token_to_op(tok: &Token) -> Option<Op> {
    match tok {
        Token::Eq => Some(Op::Eq),
        Token::NotEq => Some(Op::NotEq),
        Token::Contains => Some(Op::Contains),
        Token::StartsWith => Some(Op::StartsWith),
        Token::Gt => Some(Op::Gt),
        Token::Lt => Some(Op::Lt),
        Token::Gte => Some(Op::Gte),
        Token::Lte => Some(Op::Lte),
        _ => None,
    }
}
