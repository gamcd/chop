use std::collections::vec_deque::Iter;
use std::collections::VecDeque;
use crate::abstract_syntax_tree::{Statement, Name, Line, TypeAnnotation, TypeExpr, Expr, Value, FnDec, ProcDec, Struct, Enum, EnumEntry, Typeclass, Domain, Initialization};
use crate::tokens::{Position, Token, TokenType};

#[derive(Debug)]
pub struct Parser {
    token_stream: VecDeque<Token>,
    error_stream: VecDeque<Token>,
    expected_domain: Option<Domain>,
    indent: Position,
}

impl Parser {
    pub fn view(&self) -> View {
        View(self.token_stream.iter())
    }

    fn next(&mut self) -> Option<Token> {
        return match self.indent.part_of_line(self.token_stream.front().expect("Early End of file")) {
            Ok(_) => {
                self.token_stream.pop_front()
            },
            Err(_) => {
                self.expected_domain = None;
                Some(Token::new(TokenType::Newline, self.indent.0, 0))
            }
        }
    }

    fn yank(&mut self) -> Token { self.token_stream.pop_front().expect("Early end of file") }
    pub fn new(token_stream: VecDeque<Token>) -> Self {
        Parser {
            token_stream,
            error_stream: VecDeque::new(),
            expected_domain: None,
            indent: Position(0, 0),
        }
    }

    fn skip(&mut self, n: u8) {
        for _ in 0..n {
            self.next();
        }
    }

    fn parse_list<T: Parse>(&mut self, separator: TokenType, left: TokenType, right: TokenType) -> Result<Vec<T>, ParseError> {
        let first = self.yank();
        if let left = first.token_type {
            let mut list = Vec::new();
            loop {
                list.push(T::parse(self)?);
                let next = self.yank();
                if let right = next.token_type { return Ok(list); }
                if let separator = next.token_type {
                    if separator == TokenType::Newline {
                        self.indent = Position(self.indent.0 + 1, 0)
                    }
                }
            }
        }
        return Err(ParseError::new(format!("Unexpected first Grouping '{:?}', expected {:?}", &first,  &left), [first.position, first.position]));
    }
}

pub struct View<'a>(Iter<'a, Token>);
impl View<'_> {
    fn yank(&mut self) -> &Token {
        self.0.next().expect("Early end of file")
    }
}
fn yank(next_token: Option<&Token>) -> &Token {
   next_token.expect("Early end of file")
}

fn yank_owned(next_token: Option<Token>) -> Token {
    next_token.expect("Early end of file")
}



pub trait Parse {
    fn parse(p: &mut Parser) -> Result<Self, ParseError> where Self: Sized;
}

pub struct ParseError {
    message: String,
    err_range: [Position; 2]
}

impl ParseError {
    pub fn new(message: String, err_range: [Position; 2]) -> Self {
        ParseError {
            message,
            err_range,
        }
    }
}



//
//  Parse implementations for AST types
//



impl Parse for Line {
    fn parse(p: &mut Parser) -> Result<Self, ParseError> {
        let mut view = p.view();
        let tok = view.yank();

        return if tok.is_domain() {
            Ok(Line::Initialization(Initialization::parse(p)?))
        } else {
            Ok(Line::Statement(Statement::parse(p)?))
        }
    }
}

impl Parse for Initialization {
    fn parse(p: &mut Parser) -> Result<Self, ParseError> {
        p.indent = yank(p.token_stream.front()).position;
        let domain = Domain::parse(p)?;
        p.expected_domain = Some(domain);
        let name = Name::parse(p)?;
        if let Domain::Type = domain {
            return Ok(Initialization { domain, name, type_annotation: TypeAnnotation(None), value: Value::Type(TypeAnnotation::parse(p)?) });
        }
        let type_annotation = TypeAnnotation::parse(p)?;

        let equals = p.yank();
        if let TokenType::Equals = equals.token_type {}
        else {
            return Err(ParseError::new(format!("Expected '=', found {:?}", equals.token_type), [equals.position, equals.position]))
        }

        Ok(Initialization {
            domain,
            name,
            type_annotation,
            value: Value::parse(p)?,
        })

    }
}

impl Parse for Statement {
    fn parse(p: &mut Parser) -> Result<Statement, ParseError> {
        let mut view = p.view();
        p.indent = view.yank().position;

        todo!();
    }
}


impl Parse for Domain {
    fn parse(p: &mut Parser) -> Result<Self, ParseError> {
        Ok(p.yank().as_domain().unwrap())
    }
}




impl Parse for Name {
    fn parse(p: &mut Parser) -> Result<Self, ParseError>{
        let mut view = p.view();
        let tok = view.yank();

        return match tok.as_name() {
            Some(n) => Ok(n),
            None => Err(ParseError::new(format!("Expected Identifier, found \'{:?}\'", tok.token_type), [tok.position, view.yank().position])),
        };

    }

}

impl Parse for TypeAnnotation {
    fn parse(p: &mut Parser) -> Result<Self, ParseError> {

        let mut view = p.view();
        let first = yank(view.0.nth(0));
        let second = yank(view.0.nth(1));

        return match first.token_type {
            TokenType::Equals | TokenType::LBrace | TokenType::Comma | TokenType::Newline => {
                Ok(TypeAnnotation(None))
            },
            TokenType::Colon => {
                p.skip(1);
                Ok(TypeAnnotation(Some(TypeExpr::parse(p)?)))
            },
            _ => {
                Err(ParseError::new(format!("Unexpected token '{:?}'", second.token_type), [second.position, second.position]))
            },
        }

    }
}


//
// Value Parser implementations
//

impl Parse for Value {
    fn parse(p: &mut Parser) -> Result<Self, ParseError> {
        let mut view = p.view();
        if let Some(d) = &p.expected_domain  {
            match d {
                &Domain::Struct => return Ok(Value::Struct(Struct::parse(p)?)),
                &Domain::Enum => return Ok(Value::Enum(Enum::parse(p)?)),
                &Domain::Typeclass => return Ok(Value::Typeclass(Typeclass::parse(p)?)),
                _ => {},
            }
        }

        return Ok(Value::Expr(Expr::parse(p)?));
    }

}

impl Parse for Expr {
    fn parse(p: &mut Parser) -> Result<Self, ParseError> {
        todo!();
    }
}
impl Parse for ProcDec {
    fn parse(p: &mut Parser) -> Result<Self, ParseError> {
        todo!()
    }
}
impl Parse for FnDec {
    fn parse(p: &mut Parser) -> Result<Self, ParseError> {
        todo!()
    }
}

impl Parse for Struct {
    fn parse(p: &mut Parser) -> Result<Self, ParseError> {
        todo!()
    }
}

impl Parse for Enum {
    fn parse(p: &mut Parser) -> Result<Self, ParseError> {
        Ok(Enum(p.parse_list(TokenType::Newline, TokenType::LBrace, TokenType::RBrace)?))
    }
}

impl Parse for EnumEntry {
    fn parse(p: &mut Parser) -> Result<Self, ParseError> {
        todo!()
    }
}

impl Parse for Typeclass {
    fn parse(p: &mut Parser) -> Result<Self, ParseError> {
        todo!()
    }
}

impl Parse for TypeExpr {
    fn parse(p: &mut Parser) -> Result<Self, ParseError> {
        todo!()
    }
}