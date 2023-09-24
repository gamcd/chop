use std::collections::vec_deque::Iter;
use std::collections::VecDeque;
use crate::abstract_syntax_tree::{Name, Statement};
use crate::tokens::Token;

#[derive(Debug)]
pub struct Parser {
    token_stream: VecDeque<Token>,
}

impl Parser {
    pub fn view(&self) -> Iter<&Token> {
        self.into()
    }

    fn next(&mut self) -> Option<&Token> {
        self.token_stream.front()
    }
}

impl Parser {
    pub fn new(token_stream: VecDeque<Token>) -> Self {
        Parser {
            token_stream
        }
    }
}

pub trait Parse {
    fn parse<T>(p: &mut Parser) -> Result<T, ParseError>;
}

struct ParseError {
    message: String,
    start_line: u32,
    start_col: u16,
    end_line: u32,
    end_col: u16,
}

impl ParseError {
    fn new(message: String, start_line: u32, start_col: u16, end_line: u32, end_col: u16) -> Self {
        ParseError {
            message,
            start_line,
            start_col,
            end_line,
            end_col,
        }
    }
}

impl Parse for Statement {
    fn parse<Statement>(p: &mut Parser) -> Result<Statement, ParseError> {
        let mut view = p.view();
        let &&tok = view.next().expect("Early end of file");

        if let Some(domain) = tok.as_domain() {
            Statement::Initialization(domain, Name::parse(p), )
        }


        todo!()
    }


}