use std::str::FromStr;

#[derive(Debug, Clone)]
pub enum Token {
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
    SingleQuote,
    DoubleQuote,
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
    IntLit(i64),
    FloatLit(f64),
    StringLit(String),
}

impl FromStr for Token {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "fn" => Ok(Token::KwFn),
            "proc" => Ok(Token::KwProc),
            "struct" => Ok(Token::KwStruct),
            "union" => Ok(Token::KwUnion),
            "typeclass" => Ok(Token::KwTypeclass),
            "if" => Ok(Token::KwIf),
            "is" => Ok(Token::KwIs),
            "in" => Ok(Token::KwIn),
            "with" => Ok(Token::KwWith),
            "for" => Ok(Token::KwFor),
            "while" => Ok(Token::KwWhile),
            "null" => Ok(Token::KwNull),
            "const" => Ok(Token::KwConst),
            "var" => Ok(Token::KwVar),
            _ => Err(s.to_string())
        }
    }
}