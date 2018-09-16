//! The environment type.

use parse::expr::Expr;
use std::collections::HashMap;
use std::default::Default;

/// A single environment
pub struct Env {
    /// The mapping of names to expressions
    symbols: HashMap<String, Expr>,
}

impl Default for Env {
    fn default() -> Env {
        Env {
            symbols: HashMap::new(),
        }
    }
}

