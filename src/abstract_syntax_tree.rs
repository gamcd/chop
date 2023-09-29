use crate::tokens::{Token, TokenType};

pub enum Value {
    Expr(Expr),
    Proc(ProcDec),
    Fn(FnDec),
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

pub struct ProcDec(Signature, Vec<Line>);
pub struct FnDec(Signature, Box<Expr>);
pub struct Struct(pub Vec<Line>);
pub struct Enum(pub Vec<EnumEntry>);
pub struct EnumEntry(Field, Option<Vec<TypeExpr>>);

pub struct Typeclass(Vec<Line>);
pub struct Name(pub String);
pub struct Arg(Name, TypeAnnotation);

pub struct Signature {args: Vec<Arg>, return_type: TypeAnnotation }


pub enum Literal {
    Int(i64),
    Float(f64),
    List(Vec<Expr>),
    Set(Vec<Expr>),
    Map(Vec<(Expr, Expr)>),
    Tuple(Vec<Expr>),
    StructInitialization(Vec<Expr>),
    Closure(Box<Value>),
    Null
}


pub enum Expr {
    Call(Name, Vec<Arg>),
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
    Statement(Statement)
}

pub struct Statement {
    proc_name: Name,
    args: Vec<Arg>,
}