use crate::tokens::Token;

pub enum Domain {
    Const,
    Var,
    Proc,
    Fn,
    Struct,
    Union,
    Typeclass,
    Type,
}


pub struct Arg(Name, Option<TypeExpr>);

pub struct Signature {args: Vec<Arg>, return_type: Option<TypeExpr> }

pub enum Literal {
    Int(i64),
    Float(f64),
    List(Vec<Expr>),
    Set(Vec<Expr>),
    Map(Vec<(Expr, Expr)>),
    Null
}

pub struct Fn(Signature, Box<Expr>);
pub struct Proc(Signature, Vec<Statement>);

pub enum Expr {
    FunctionCall(Fn, Vec<Expr>),
    Literal(Literal),
    Reference(Name),
    Grouping,
}

pub enum TypeExpr {
    Operator(Token, Vec<TypeExpr>),
    Literal(Name),
    Grouping,
}

pub struct Name(String);
pub enum Value {
    F(Fn),
    P(Proc),
    E(Expr),
    T(TypeExpr),
}


pub enum Statement {
    Initialization(Domain, Name, Option<TypeExpr>, Value),
    ProcCall(Name, Vec<Value>),
}
