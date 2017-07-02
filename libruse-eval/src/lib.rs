//! Evaluates Ruse expressions

extern crate libruse_read as read;

use read::parse::expr::Expr;

/// Evaluates an expression into a result for printing.
pub fn eval(e: Expr) -> Result<Expr, ()> {
    unimplemented!()
}

