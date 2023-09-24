use crate::parser::{Parse, Parser};
use crate::tokens::{Token, TokenType};

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
    StructInitialization(Vec<Field, Expr>),
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
impl Parse for Name {
    fn parse<T>(p: &mut Parser) -> T {
        let &&tok = p.view().next().expect("Early end of file");
        tok.as_name()?
    }

}
pub struct Field(String);
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
