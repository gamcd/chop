use std::collections::vec_deque::Iter;
use std::collections::VecDeque;
use crate::abstract_syntax_tree::{Field, LineStatement, Name, Statement, TypeAnnotation, TypeExpr, Value};
use crate::tokens::{Position, Token, TokenType};

#[derive(Debug)]
pub struct Parser {
    token_stream: VecDeque<Token>,
    error_stream: VecDeque<Token>,
}

impl Parser {
    pub fn view(&self) -> Iter<&Token> {
        self.into()
    }

    fn next(&mut self) -> Option<&Token> {
        self.token_stream.front()
    }

    pub fn new(token_stream: VecDeque<Token>) -> Self {
        Parser {
            token_stream,
            error_stream: VecDeque::new(),
        }
    }

    fn skip(&mut self, n: u8) {
        for _ in 0..n {
            self.next()
        }
    }

    fn skip_every(&mut self, token_type: TokenType) {
        for &&a in self.view() {
            if a.token_type != token_type {break}
            self.next()
        }
    }
}

pub trait Parse {
    fn parse<T>(p: &mut Parser) -> Result<T, ParseError>;
}

struct ParseError {
    message: String,
    err_range: [Position; 2]
}

impl ParseError {
    fn new(message: String, err_range: [Position; 2]) -> Self {
        ParseError {
            message,
            err_range,
        }
    }
}



//
//  Parse implementations for AST types
//

fn yank(next_token: Option<Token>) -> Token {
    next_token.expect("Early end of file")
}


impl Parse for Statement {
    fn parse<Statement>(p: &mut Parser) -> Result<Statement, ParseError> {
        let mut view = p.view();
        let tok = yank(**view.next());

        if let Some(domain) = tok.as_domain() {
            Statement::Initialization(domain, Name::parse(p), Field::parse(p), Value::parse(p))
        }

        Statement::ProcCall(LineStatement::parse(p))
    }
}



impl Parse for LineStatement {
    fn parse<LineStatement>(p: &mut Parser) -> Result<LineStatement, ParseError> {
        todo!()
    }
}

impl Parse for Name {
    fn parse<Name>(p: &mut Parser) -> Result<Name, ParseError>{
        let mut view = p.view();
        let tok = yank(**view.next());

        tok.as_name().ok_or(ParseError::new(
            format!("Expected Identifier, found \'{:?}\'", tok.token_type),
            [tok.position, view.next().expect("Early end of file").position]
        ))
    }

}

impl Parse for TypeAnnotation {
    fn parse<TypeAnnotation>(p: &mut Parser) -> Result<TypeAnnotation, ParseError> {

        let mut view = p.view();
        let first = yank(**view.nth(0));
        let second = yank(**view.nth(1));

        return match first.token_type {
            TokenType::WhiteSpace => {
                return match second.token_type {
                    TokenType::Comma => {
                        p.skip(1);
                        Ok(TypeAnnotation(None))
                    },
                    TokenType::Equals | TokenType::WhiteSpace => {
                        p.skip(2);
                        Ok(TypeAnnotation(None))
                    },
                    _ => {Err(ParseError::new(format!("Unexpected token '{:?}'", second.token_type), [second.position, second.position]))},
                }
            },
            TokenType::Colon => {
                p.skip(1);
                Ok(TypeExpr::parse(p)?)
            },
            _ => {
                Err(ParseError::new(format!("Unexpected token '{:?}'", second.token_type), [second.position, second.position]))
            },
        }

    }
}

impl Parse for Value {
    fn parse<Value>(p: &mut Parser) -> Result<Value, ParseError> {
        let mut view = p.view();

        loop {
            let tok = yank(**view.next());
        }

        todo!()
    }
}