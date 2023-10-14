use crate::parser::Parse;
use crate::tokens::{Token, TokenType};

pub enum Value {
    Expr(Expr),
    Struct(Struct),
    Enum(Enum),
    Typeclass(Typeclass),
    Type(TypeAnnotation),
}

#[derive(Clone, Copy, Debug)]
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

pub struct Struct(pub Vec<Line>);
pub struct Enum(pub Vec<EnumEntry>);
pub struct EnumEntry(Field, Option<Vec<TypeExpr>>);

pub struct Typeclass(pub Vec<Line>);
pub struct Name(pub String);

pub struct Signature {
    args: Vec<Tag<Name, TypeExpr>>,
    return_type: TypeAnnotation,
}

pub struct Tag<T: Parse, U: Parse>(pub T, pub Option<U>);

pub enum Literal {
    Null,
    Void,
    Int(i64),
    Float(f64),
    Bool(bool),
    List(Vec<Expr>),
    Set(Vec<Expr>),
    Map(Vec<(Expr, Expr)>),
    Tuple(Vec<Expr>),
    StructInitialization(Name, Vec<Expr>),
    Closure(Box<Value>),
}

pub enum Expr {
    Sequence(Vec<Expr>),
    Call(Name, Vec<Expr>),
    Literal(Literal),
    Reference(Name),
    FieldAccess(Box<Expr>, Field),
    Grouping(TokenType, Box<Expr>),
}

pub enum TypeExpr {
    Operator(Token, Vec<TypeExpr>),
    Literal(Name),
    Grouping,
}

pub struct Field {
    pub field_name: String,
}
pub struct TypeAnnotation(pub Option<TypeExpr>);

pub struct Initialization {
    pub domain: Domain,
    pub name: Name,
    pub type_annotation: TypeAnnotation,
    pub value: Value,
}

pub enum Line {
    Initialization(Initialization),
    Statement(Statement),
    Return(Expr),
    For(ForStatement),
    While(Conditional),
    If(Conditional),
    Break,
    Continue,
}

pub struct Conditional(pub Expr, pub Vec<Line>, pub Option<Vec<Line>>);
pub struct ForStatement(pub Vec<Name>, pub Expr, pub Vec<Line>);

pub struct Statement {
    proc_name: Name,
    args: Vec<Expr>,
}
