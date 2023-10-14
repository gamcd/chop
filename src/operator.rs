use crate::tokens::TokenType;

pub struct TypeOperator;
impl BP for TypeOperator {
    fn prefix_bp(token_type: TokenType) -> Option<u8> {
        todo!()
    }

    fn infix_bp(token_type: TokenType) -> Option<(u8, u8)> {
        todo!()
    }

    fn postfix_bp(token_type: TokenType) -> Option<u8> {
        todo!()
    }
}

pub struct ExprOperator;
impl BP for ExprOperator {
    fn prefix_bp(token_type: TokenType) -> Option<u8> {
        match token_type {
            TokenType::Bang => Some(15),
            TokenType::Negate => Some(15),
            _ => None,
        }
    }

    fn infix_bp(token_type: TokenType) -> Option<(u8, u8)> {
        match token_type {
            TokenType::Dot => Some((15, 16)),
            TokenType::Percent | TokenType::Slash | TokenType::Star => Some((13, 14)),
            TokenType::Plus | TokenType::Minus => Some((11, 12)),
            TokenType::KwAnd | TokenType::KwOr => Some((9, 10)),
            TokenType::EqualsEq | TokenType::BangEq => Some((7, 8)),
            TokenType::KwIf => Some((6, 5)),
            TokenType::KwFor => Some((5, 6)),
            TokenType::Arrow => Some((4, 3)),
            _ => None,
        }
    }

    fn postfix_bp(token_type: TokenType) -> Option<u8> {
        match token_type {
            TokenType::RBrace | TokenType::RBracket | TokenType::RParen => Some(17),
            TokenType::Comma => Some(1),
            _ => None,
        }
    }
}

pub struct LineOperator;
impl BP for LineOperator {
    fn prefix_bp(token_type: TokenType) -> Option<u8> {
        todo!();
    }

    fn infix_bp(token_type: TokenType) -> Option<(u8, u8)> {
        todo!();
    }

    fn postfix_bp(token_type: TokenType) -> Option<u8> {
        todo!();
    }
}

pub trait BP {
    fn prefix_bp(token_type: TokenType) -> Option<u8>;
    fn infix_bp(token_type: TokenType) -> Option<(u8, u8)>;
    fn postfix_bp(token_type: TokenType) -> Option<u8>;
}
