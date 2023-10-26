use std::slice::Iter;
use std::collections::VecDeque;

use crate::abstract_syntax_tree::{
    Conditional, Domain, Enum, EnumEntry, Expr, ForStatement, Initialization, Line, Literal, Name,
    Statement, Struct, Tag, TypeAnnotation, TypeExpr, Typeclass, Value,
};
use crate::operator::{ExprOperator, BP};
use crate::tokens::{Position, Token, TokenType};

#[derive(Debug)]
pub struct Parser {
    token_stream: VecDeque<Token>,
    error_stream: VecDeque<Token>,
    expected_domain: Option<Domain>,
    indent: Position,
}

impl Parser {
    fn next(&mut self) -> Option<Token> {
        return match self
            .indent
            .includes(self.token_stream.front().expect("Early End of file"))
        {
            Ok(_) => self.token_stream.pop_front(),
            Err(_) => {
                self.expected_domain = None;
                Some(Token::new(TokenType::Newline, self.indent.0, 0))
            }
        };
    }

    fn peek(&mut self) -> Token {
        return match self
            .indent
            .includes(self.token_stream.front().expect("Early End of file"))
        {
            Ok(_) => self
                .token_stream
                .front()
                .expect("Early end of File")
                .clone(),
            Err(_) => {
                self.expected_domain = None;
                Token::new(TokenType::Newline, self.indent.0, 0)
            }
        };
    }

    fn yank(&mut self) -> Token {
        self.next().expect("Early end of file")
    }

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

    fn parse_list<T: Parse>(
        &mut self,
        separator: TokenType,
        left: TokenType,
        right: TokenType,
    ) -> Result<Vec<T>, ParseError> {
        let first = self.yank();
        if left != first.token_type && left != TokenType::KwFor {
            return Err(ParseError::new(
                format!(
                    "Unexpected first Grouping '{:?}', expected {:?}",
                    &first, &left
                ),
                first.position
            ));
        }

        let mut list = Vec::new();
        loop {
            let element = self.yank();
            if element.token_type == right {
                return Ok(list);
            }
            list.push(T::parse(self)?);
            if separator == TokenType::Newline {
                self.indent = Position(self.indent.0 + 1, 0);
            }
            let next = self.yank();
            match next.token_type {
                separator => {}
                right => return Ok(list),
                _ => {
                    return Err(ParseError::new(
                        format!(
                            "Unexpected token '{:?}', expected '{:?}' | '{:?}'",
                            &first, &separator, &right
                        ),
                        element.position
                    ))
                }
            }
        }
    }

    fn parse_tag<T: Parse, U: Parse>(
        &mut self,
        separator: TokenType,
    ) -> Result<(T, Option<U>), ParseError> {
        let first = T::parse(self)?;
        let next = self.yank();
        if next.token_type == separator {
            return Ok((first, Some(U::parse(self)?)));
        } else {
            return Ok((first, None));
        }
    }

    fn ast_build(&mut self) -> Vec<Statement> {
        todo!()
    }
}

fn yank(next_token: Option<&Token>) -> &Token {
    next_token.expect("Early end of file")
}

fn yank_owned(next_token: Option<Token>) -> Token {
    next_token.expect("Early end of file")
}

pub trait Parse {
    fn parse(p: &mut Parser) -> Result<Self, ParseError>
    where
        Self: Sized;
}

pub trait ParseBP {
    fn parse_bp(p: &mut Parser, min_bp: u8) -> Result<Self, ParseError>
    where
        Self: Sized;
}

pub struct ParseError {
    message: String,
    position: Position,
}

impl ParseError {
    pub fn new(message: String, position: Position) -> Self {
        ParseError { message, position }
    }
}

//
//  Parse implementations for AST types
//

impl Parse for Line {
    fn parse(p: &mut Parser) -> Result<Self, ParseError> {
        let tok = yank(p.token_stream.front());

        if tok.is_domain() {
            return Ok(Line::Initialization(Initialization::parse(p)?));
        }

        return match tok.token_type {
            TokenType::KwFor => Ok(Line::For(ForStatement::parse(p)?)),
            TokenType::KwIf => Ok(Line::If(Conditional::parse(p)?)),
            TokenType::KwWhile => Ok(Line::While(Conditional::parse(p)?)),
            TokenType::KwReturn => {p.yank(); Ok(Line::Return(Expr::parse(p)?))},
            TokenType::KwBreak => {p.yank(); Ok(Line::Break)},
            TokenType::KwContinue => {p.yank(); Ok(Line::Continue)},
            _ => Ok(Line::parse(p)?),
        };
    }
}

impl Parse for ForStatement {
    fn parse(p: &mut Parser) -> Result<Self, ParseError> {
        Ok(ForStatement(
            p.parse_list(TokenType::Comma, TokenType::KwFor, TokenType::KwIn)?,
            Expr::parse(p)?,
            p.parse_list(TokenType::Newline, TokenType::LBrace, TokenType::RBrace)?,
        ))
    }
}

impl Parse for Conditional {
    fn parse(p: &mut Parser) -> Result<Self, ParseError> {
        Ok(Conditional(
            Expr::parse(p)?,
            p.parse_list(TokenType::Newline, TokenType::LBrace, TokenType::RBrace)?,
            if p.peek().token_type == TokenType::KwElse {
                p.next();
                Some(p.parse_list(TokenType::Newline, TokenType::LBrace, TokenType::RBrace)?)
            } else {
                None
            },
        ))
    }
}

impl Parse for Initialization {
    fn parse(p: &mut Parser) -> Result<Self, ParseError> {
        p.indent = p.peek().position;

        let domain = Domain::parse(p)?;

        p.expected_domain = Some(domain);

        let name = Name::parse(p)?;

        if let Domain::Type = domain {
            return Ok(Initialization {
                domain,
                name,
                type_annotation: TypeAnnotation(None),
                value: Value::Type(TypeAnnotation::parse(p)?),
            });
        }

        let type_annotation = TypeAnnotation::parse(p)?;

        let equals = p.yank();

        if let TokenType::Equals = equals.token_type {
        } else {
            return Err(ParseError::new(
                format!("Expected '=', found {:?}", equals.token_type),
                equals.position
            ));
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
        todo!()
    }
}

impl Parse for Domain {
    fn parse(p: &mut Parser) -> Result<Self, ParseError> {
        Ok(p.yank().as_domain().unwrap())
    }
}

impl Parse for Name {
    fn parse(p: &mut Parser) -> Result<Self, ParseError> {
        let tok = yank(p.token_stream.front());

        return match tok.as_name() {
            Some(n) => Ok(n),
            None => Err(ParseError::new(
                format!("Expected Identifier, found '{:?}\'", tok.token_type),
                tok.position,
            )),
        };
    }
}

impl<T: Parse, U: Parse> Parse for Tag<T, U> {
    fn parse(p: &mut Parser) -> Result<Self, ParseError> {
        let (left, right) = p.parse_tag(TokenType::Colon)?;
        Ok(Tag(left, right))
    }
}

impl Parse for TypeAnnotation {
    fn parse(p: &mut Parser) -> Result<Self, ParseError> {
        let first = yank(p.token_stream.front());
        let second = yank(p.token_stream.get(1));

        return match first.token_type {
            TokenType::Equals | TokenType::LBrace | TokenType::Comma | TokenType::Newline => {
                Ok(TypeAnnotation(None))
            }
            TokenType::Colon => {
                p.skip(1);
                Ok(TypeAnnotation(Some(TypeExpr::parse(p)?)))
            }
            _ => Err(ParseError::new(
                format!("Unexpected token '{:?}'", second.token_type),
                second.position
            )),
        };
    }
}

//
// Value Parser implementations
//

impl Parse for Value {
    fn parse(p: &mut Parser) -> Result<Self, ParseError> {
        if let Some(d) = &p.expected_domain {
            match d {
                &Domain::Struct => return Ok(Value::Struct(Struct::parse(p)?)),
                &Domain::Enum => return Ok(Value::Enum(Enum::parse(p)?)),
                &Domain::Typeclass => return Ok(Value::Typeclass(Typeclass::parse(p)?)),
                _ => {}
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

impl ParseBP for Expr {
    fn parse_bp(p: &mut Parser, min_bp: u8) -> Result<Self, ParseError> {
        let first = p.yank();

        let lhs: Expr = if let Some(bp) = ExprOperator::prefix_bp(&first.token_type) {
            match first.token_type {
                TokenType::LParen => {
                    if p.peek().token_type == TokenType::RParen {
                        p.next();
                        Expr::Literal(Literal::Void)
                    } else {
                        let inside = Expr::parse_bp(p, bp)?;
                        match inside {
                            Expr::Sequence(s) => Expr::Literal(Literal::Tuple(s)),
                            e => e,
                        }
                    }
                }

                TokenType::LBrace => {
                    let elements = p.parse_list::<Tag<Expr, Expr>>(
                        TokenType::Comma,
                        TokenType::LBrace,
                        TokenType::RBrace,
                    )?;
                    let mut err = "";
                    if elements[0].1.is_some() {
                        let mut map = Vec::new();
                        for e in elements {
                            if let Some(v) = e.1 {
                                map.push((e.0, v))
                            } else {
                                err = "Expected map entry, found set entry";
                                break;
                            }
                        }

                        if err != "" {
                            return Err(ParseError::new(err.to_string(), first.position));
                        }

                        Expr::Literal(Literal::Map(map))
                    } else {
                        let mut set = Vec::new();
                        for e in elements {
                            if e.1.is_none() {
                                set.push(e.0);
                            } else {
                                err = "Expected set entry, found map entry";
                                break;
                            }
                        }

                        if err != "" {
                            return Err(ParseError::new(err.to_string(), first.position));
                        }

                        Expr::Literal(Literal::Set(set))
                    }
                }

                TokenType::LBracket => {
                    let list =
                        p.parse_list(TokenType::Comma, TokenType::LBracket, TokenType::RBracket)?;
                    Expr::Literal(Literal::List(list))
                }

                TokenType::Bang => {
                    Expr::Call(Name(String::from("not")), vec![Expr::parse_bp(p, bp)?])
                }

                _ => Expr::Call(Name(String::from("negate")), vec![Expr::parse_bp(p, bp)?]),
            }
        } else {
            match &first.token_type {
                TokenType::Ident(s) => Expr::Reference(Name(s.to_string())),
                TokenType::KwNull => Expr::Literal(Literal::Null),
                TokenType::KwTrue => Expr::Literal(Literal::Bool(true)),
                TokenType::KwFalse => Expr::Literal(Literal::Bool(false)),
                TokenType::IntLit(i) => Expr::Literal(Literal::Int(*i)),
                TokenType::FloatLit(f) => Expr::Literal(Literal::Float(*f)),
                _ => {
                    return Err(ParseError::new(
                        format!("Unexpected token {:?}, unsure what happened", first),
                        first.position
                    ))
                }
            }
        };

        loop {}
    }
}

impl Parse for Struct {
    fn parse(p: &mut Parser) -> Result<Self, ParseError> {
        Ok(Struct(p.parse_list(
            TokenType::Newline,
            TokenType::LBrace,
            TokenType::RBrace,
        )?))
    }
}

impl Parse for Enum {
    fn parse(p: &mut Parser) -> Result<Self, ParseError> {
        Ok(Enum(p.parse_list(
            TokenType::Newline,
            TokenType::LBrace,
            TokenType::RBrace,
        )?))
    }
}

impl Parse for EnumEntry {
    fn parse(p: &mut Parser) -> Result<Self, ParseError> {
        todo!()
    }
}

impl Parse for Typeclass {
    fn parse(p: &mut Parser) -> Result<Self, ParseError> {
        Ok(Typeclass(p.parse_list(
            TokenType::Newline,
            TokenType::LBrace,
            TokenType::RBrace,
        )?))
    }
}

impl Parse for TypeExpr {
    fn parse(p: &mut Parser) -> Result<Self, ParseError> {
        todo!()
    }
}
