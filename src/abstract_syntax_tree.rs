use crate::tokens::Token;

pub enum Domain {
    Const,
    Var,
    Proc,
    Fn,
    Struct,
    Union,
    Typeclass,
    Type
}


pub struct Arg(Name, Option<TypeExpr>);

pub struct Signature {args: Vec<Arg>, return_type: Option<TypeExpr> }
impl Signature {
    fn type_signature(&self) -> Vec<Option<TypeExpr>> {
        self.args.iter().map(|&s| s.1).collect()
    }
}


pub struct Fn(Signature, Expr);
pub struct Proc(Signature, Vec<Statement>);

pub enum Expr {
    Function(Fn, Vec<Expr>),
    Value(Value),
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
    R(Name)
}


pub enum Statement {
    Initialization(Domain, Name, Option<TypeExpr>, Value),
    ProcCall(Name, Vec<Value>),
}
