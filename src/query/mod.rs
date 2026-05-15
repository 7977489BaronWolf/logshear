pub mod lexer;
#[cfg(test)]
mod lexer_tests;

pub mod parser;
#[cfg(test)]
mod parser_tests;

pub use parser::{Expr, Op, ParseError, Parser, Value};

/// Parse a query string into an AST expression.
///
/// # Example
/// ```
/// let expr = logshear::query::parse("level = \"error\" AND service = \"api\"").unwrap();
/// ```
pub fn parse(input: &str) -> Result<Expr, ParseError> {
    Parser::new(input).parse()
}
