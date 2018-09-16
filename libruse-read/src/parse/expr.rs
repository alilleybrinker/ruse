use std::fs::File;
use std::collections::HashMap;
use std::default::Default;

pub struct Expr {
    pub kind: ExprKind,
    pub marked: bool,
    pub mutable: bool,
}

impl Expr {
    pub fn new(kind: ExprKind) -> Expr {
        let marked = false;
        let mutable = false;

        Expr { kind, marked, mutable }
    }
}

pub enum ExprKind {
    Bool(bool),
    Char(char),
    Num(Number),
    Pair(Pair),
    Closure(Closure),
    Syntax(Syntax),
    Symbol(Symbol),
    Str(String),
    Vector(Vector),
    ByteVector(ByteVector),
    Continuation(Continuation),
    Port(Port),
    Env(Env),
}

pub struct Env {
    symbols: HashMap<String, Expr>,
}

impl Default for Env {
    fn default() -> Env {
        Env {
            symbols: HashMap::new(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Number {
    pub exact: bool,
    pub kind: NumberKind,
}

#[derive(Debug, PartialEq)]
pub enum NumberKind {
    Int(i64),
    Real(f64),
    Rational {
        numerator: i64,
        denominator: i64,
    }
}

pub struct Pair {
    pub car: Box<Expr>,
    pub cdr: Box<Expr>,
}

pub struct Closure {
    pub lambda: Box<Lambda>,
    pub env: Env,
    pub syntactic: bool,
    pub body: Box<Expr>,
    pub args: Box<Expr>,
}

pub type Lambda = Fn(Box<Expr>, Box<Env>) -> Box<Expr>;
pub type Symbol = String;
pub type Vector = Vec<Box<Expr>>;
pub type ByteVector = Vec<u8>;

pub struct Syntax {
    pub transformer: Box<Expr>,
    pub env: Box<Env>,
}

// TODO: Implement continuations.
#[derive(Debug, PartialEq)]
pub struct Continuation {}

#[derive(Debug)]
pub struct Port {
    pub file: File,
    pub string: String,
    pub kind: PortKind,
    pub writable: bool,
    pub readable: bool,
}

#[derive(Debug, PartialEq, Eq)]
pub enum PortKind {
    Textual,
    Binary,
}

