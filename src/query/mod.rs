pub mod ast;
pub mod eval;
pub mod lexer;
pub mod optimizer;
pub mod parser;

#[cfg(test)]
mod eval_tests;
#[cfg(test)]
mod lexer_tests;
#[cfg(test)]
mod optimizer_tests;
#[cfg(test)]
mod parser_tests;

use ast::Expr;

/// Parse a query string and return an optimized AST ready for evaluation.
pub fn compile(input: &str) -> Result<Expr, String> {
    let tokens = lexer::tokenize(input)?;
    let expr = parser::parse(tokens)?;
    Ok(optimizer::optimize(expr))
}
