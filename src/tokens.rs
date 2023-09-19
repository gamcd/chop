use std::str::FromStr;

#[derive(Debug, Clone)]
pub enum TokenType {
    KwFn,
    KwProc,
    KwStruct,
    KwUnion,
    KwTypeclass,
    KwIf,
    KwIs,
    KwWith,
    KwIn,
    KwFor,
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

    Bang,       BangEq,
    Percent,    PercentEq,
    Plus,       PlusEq,
    Minus,      MinusEq,
    Arrow,
    Star,       StarEq,
    Slash,      SlashEq,
    Equals,     EqualsEq,

    EOF,
    WhiteSpace,
    Ident(String),
    IntLit(Box<i64>),
    FloatLit(Box<f64>),
    StringLit(String),
}

#[derive(Debug, Clone)]
pub(crate) struct Token {
    token_type: TokenType,
    line: u32,
    col: u32,
}

impl Token {
    pub fn new(token_type: TokenType, line: u32, col: u32) -> Self {
        return Token {
            token_type,
            line,
            col
        }
    }
}

impl FromStr for TokenType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, ()> {
        match s {
            "fn" => Ok(TokenType::KwFn),
            "proc" => Ok(TokenType::KwProc),
            "struct" => Ok(TokenType::KwStruct),
            "union" => Ok(TokenType::KwUnion),
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
