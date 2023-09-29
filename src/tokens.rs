use std::str::FromStr;
use crate::abstract_syntax_tree::{Name, Domain};

#[repr(u8)]
#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    KwFn,
    KwProc,
    KwStruct,
    KwEnum,
    KwType,
    KwTypeclass,
    KwIf,
    KwElse,
    KwIs,
    KwIn,
    KwFor,
    KwReturn,
    KwWith,
    KwWhile,
    KwNull,
    KwConst,
    KwVar,
    KwAnd,
    KwOr,
    KwTrue,
    KwFalse,

    Newline,
    Comma,
    Dot,
    LT,
    GT,
    At,
    LBrace,
    RBrace,
    LBracket,
    RBracket,
    LParen,
    RParen,
    SemiColon,
    Colon,
    Underscore,
    Question,
    SingleQuote,
    Delim,

    Equals,     EqualsEq,
    Bang,       BangEq,
    Percent,    PercentEq,
    Star,       StarEq,
    Slash,      SlashEq,
    Plus,       PlusEq,
    Minus,      MinusEq,
    Negate,
    Arrow,

    EOF,
    Whitespace,
    Ident(String),
    IntLit(Box<i64>),
    FloatLit(Box<f64>),
    StringLit(String),
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub position: Position,
}


#[derive(Copy, Clone, Debug)]
pub struct Position(pub u32, pub u16);

impl Position {
    pub fn part_of_line(&self, t: &Token) -> Result<(), ()> {
        return match (*t).token_type {
            TokenType::RParen | TokenType::RBracket | TokenType::RBrace => {
                if self.1 <= t.position.1 {
                    Ok(())
                } else {
                    Err(())
                }
            },
            _ => {
                if self.1 < t.position.1 {
                    Ok(())
                } else {
                    Err(())
                }
            }
        }
    }
}

impl Token {
    pub fn new(token_type: TokenType, line: u32, col: u16) -> Self {
        return Token {
            token_type,
            position: Position(line, col),
        }
    }

    pub fn is_domain(&self) -> bool {
        match self.token_type {
            TokenType::KwConst |
            TokenType::KwVar |
            TokenType::KwFn |
            TokenType::KwProc |
            TokenType::KwStruct |
            TokenType::KwEnum |
            TokenType::KwType |
            TokenType::KwTypeclass => true,
            _ => false
        }
    }

    pub fn as_domain(&self) -> Option<Domain> {
        match self.token_type {
            TokenType::KwConst => Some(Domain::Const),
            TokenType::KwVar => Some(Domain::Var),
            TokenType::KwFn => Some(Domain::Fn),
            TokenType::KwProc => Some(Domain::Proc),
            TokenType::KwStruct => Some(Domain::Struct),
            TokenType::KwEnum => Some(Domain:: Enum),
            TokenType::KwType => Some(Domain::Type),
            TokenType::KwTypeclass => Some(Domain::Typeclass),
            _ => None
        }
    }

    pub fn as_name(&self) -> Option<Name> {
        if let TokenType::Ident(s ) = &self.token_type {
            Some(Name(s.to_string()))
        } else {
            None
        }
    }


    pub fn as_keyword(&self) -> Option<TokenType> {
        match self.token_type {
            TokenType::KwFn => Some(TokenType::KwFn),
            TokenType::KwProc => Some(TokenType::KwProc),
            TokenType::KwStruct => Some(TokenType::KwStruct),
            TokenType::KwEnum => Some(TokenType::KwEnum),
            TokenType::KwType => Some(TokenType::KwType),
            TokenType::KwTypeclass => Some(TokenType::KwTypeclass),
            TokenType::KwIf => Some(TokenType::KwIf),
            TokenType::KwIs => Some(TokenType::KwIs),
            TokenType::KwWith => Some(TokenType::KwWith),
            TokenType::KwIn => Some(TokenType::KwIn),
            TokenType::KwFor => Some(TokenType::KwFor),
            TokenType::KwWhile => Some(TokenType::KwWhile),
            TokenType::KwNull => Some(TokenType::KwNull),
            TokenType::KwConst => Some(TokenType::KwConst),
            TokenType::KwVar => Some(TokenType::KwVar),
            TokenType::KwAnd => Some(TokenType::KwAnd),
            TokenType::KwOr => Some(TokenType::KwOr),
            TokenType::KwTrue => Some(TokenType::KwTrue),
            TokenType::KwFalse => Some(TokenType::KwFalse),
            _ => None
        }
    }

    pub fn bp(&self, expr_type: ExprType) -> Option<(u8, u8)> {
        todo!();
        /*
        match expr_type {
            ExprType::TypeExpr => {
                match self.token_type {
                }
            },
            ExprType::PureExpr => {

            },
            ExprType::LineExpr => {

            },
        }
        */
    }
}

pub enum ExprType {
    TypeExpr,
    PureExpr,
    LineExpr,
}

impl FromStr for TokenType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, ()> {
        match s {
            "fn" => Ok(TokenType::KwFn),
            "proc" => Ok(TokenType::KwProc),
            "struct" => Ok(TokenType::KwStruct),
            "union" => Ok(TokenType::KwEnum),
            "type" => Ok(TokenType::KwType),
            "typeclass" => Ok(TokenType::KwTypeclass),
            "if" => Ok(TokenType::KwIf),
            "is" => Ok(TokenType::KwIs),
            "in" => Ok(TokenType::KwIn),
            "with" => Ok(TokenType::KwWith),
            "for" => Ok(TokenType::KwFor),
            "while" => Ok(TokenType::KwWhile),
            "null" => Ok(TokenType::KwNull),
            "const" => Ok(TokenType::KwConst),
            "var" => Ok(TokenType::KwVar),
            "and" => Ok(TokenType::KwAnd),
            "or" => Ok(TokenType::KwOr),
            "true" => Ok(TokenType::KwTrue),
            "false" => Ok(TokenType::KwFalse),
            _ => Ok(TokenType::Ident(s.to_string()))
        }
    }
}
