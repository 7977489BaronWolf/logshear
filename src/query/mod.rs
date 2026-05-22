pub mod ast;
pub mod eval;
pub mod highlight;
pub mod lexer;
pub mod optimizer;
pub mod parser;

#[cfg(test)]
mod eval_tests;
#[cfg(test)]
mod highlight_tests;
#[cfg(test)]
mod lexer_tests;
#[cfg(test)]
mod optimizer_tests;
#[cfg(test)]
mod parser_tests;

pub use ast::Expr;
pub use eval::{eval, EvalContext};
pub use highlight::{highlight_literal, render_highlighted, Highlight, HighlightKind};
pub use optimizer::optimize;
pub use parser::parse;
