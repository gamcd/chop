use std::collections::VecDeque;
use crate::tokens::Token;

pub struct Parser {
    token_stream: VecDeque<Token>,
}

impl Parser {
    pub fn new(token_stream: VecDeque<Token>) -> Self {
        Parser {
            token_stream
        }
    }
}