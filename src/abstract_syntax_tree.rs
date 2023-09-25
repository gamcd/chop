use crate::tokens::{Token};

pub enum Domain {
    Const,
    Var,
    Proc,
    Fn,
    Struct,
    Enum,
    Typeclass,
    Type,
}

pub struct Arg(Name, Option<TypeExpr>);

pub struct Signature {args: Vec<Arg>, return_type: Option<TypeExpr> }


pub struct Literal {
    grouping: Option<Token>,
}
pub enum ExprLiteral {
    Int(i64),
    Float(f64),
    List(Vec<Expr>),
    Set(Vec<Expr>),
    Map(Vec<(Expr, Expr)>),
    Tuple(Vec<Expr>),
    StructInitialization(Vec<(Field, Expr)>),
    Null
}

pub struct Fn(Signature, Box<Expr>);
pub struct Proc(Signature, Vec<Statement>);

pub enum Expr {
    FunctionCall(Fn, Vec<Arg>),
    ProcCall(Proc, Vec<Arg>),
    Literal(Literal),
    Reference(Name),
    Grouping,
}

pub enum TypeExpr {
    Operator(Token, Vec<TypeExpr>),
    Literal(Name),
    Grouping,
}

pub struct Name(pub String);
pub struct Field(pub String);
pub struct TypeAnnotation(pub Option<TypeExpr>);
pub enum Value {
    F(Fn),
    P(Proc),
    E(Expr),
    T(TypeExpr),
}


pub enum Statement {
    Initialization(Domain, Name, TypeAnnotation, Value),
    ProcCall(LineStatement),
}

pub struct LineStatement {
    proc_name: Name,
    args: Vec<Arg>,
}