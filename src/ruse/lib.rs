#![deny(missing_docs)]

//! Ruse is an embedded Scheme for Rust. It is currently in the very early
//! stages, and should not be used for anything.
//!
//! At the moment, Ruse is little more than a lexer and a work-in-progress
//! parser. Long term, the goal is for Ruse to be an R7RS small-specification
//! compliant version of Scheme for embedding in Rust programs. Ruse code will
//! be able to call Rust code, and Rust code will be able to call Ruse. Their
//! relationship should be akin to the relationship between Lua and C.
//!
//! This is obviously an amibitious project, and I don't know when/if Ruse will
//! reach this point. But it's sure fun to play with!

pub mod engine;
pub mod read;
