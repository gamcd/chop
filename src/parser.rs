use std::collections::VecDeque;
use crate::tokens::Token;

pub struct Parser {
    token_stream: VecDeque<Token>,
}

trait Parse {
    fn parse<T>(p: &mut Parser) -> T;
}

impl Parser {
    pub fn new(token_stream: VecDeque<Token>) -> Self {
        Parser {
            token_stream
        }
    }
}