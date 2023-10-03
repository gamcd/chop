use crate::tokens::{Token, TokenType};

pub struct TypeOperator;
impl bp for TypeOperator {
    fn bp(token: Token) -> Result<(u8, u8), String> {
        todo!()
    }

    fn as_operator(token: Token) -> Option<Token> {
        todo!()
    }
}

pub struct ExprOperator;
impl bp for ExprOperator {
    fn bp(token: Token) -> Result<(u8, u8), String> {
        todo!()
    }

    fn as_operator(token: Token) -> Option<Token> {
        match &token.token_type {
            TokenType::KwIf
            | TokenType::KwIn
            | TokenType::KwFor
            | TokenType::KwAnd
            | TokenType::KwOr
            | TokenType::Dot
            | TokenType::LParen
            | TokenType::LBrace
            | TokenType::LBracket
            | TokenType::LBracket
            | TokenType::Question
            | TokenType::LT
            | TokenType::GT
            | TokenType::EqualsEq
            | TokenType::BangEq
            | TokenType::Bang
            | TokenType::Percent
            | TokenType::Star
            | TokenType::Slash
            | TokenType::Plus
            | TokenType::Minus
            | TokenType::Negate
            | TokenType::Arrow
            => Some(token),
            _ => None
        }
    }
}

pub struct LineOperator;
impl bp for LineOperator {
    fn bp(token: Token) -> Result<(u8, u8), String> {
        todo!()
    }

    fn as_operator(token: Token) -> Option<Token> {
        match &token.token_type {
              TokenType::KwIf
            | TokenType::KwElse
            | TokenType::KwFor
            | TokenType::KwReturn
            | TokenType::KwWhile
            | TokenType::KwBreak
            | TokenType::Equals
            | TokenType::PercentEq
            | TokenType::StarEq
            | TokenType::SlashEq
            | TokenType::PlusEq
            | TokenType::MinusEq
            => Some(token),
            _ => None,
        }
    }
}

pub trait bp {
    fn bp(token: Token) -> Result<(u8, u8), String>;
    fn as_operator(token: Token) -> Option<Token>;
}