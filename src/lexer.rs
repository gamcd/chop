use std::collections::VecDeque;
use std::io::{BufRead, BufReader};
use std::fs::File;
use std::str::FromStr;
use crate::tokens::{TokenType, Token};

pub struct Lexer {
    pub stream: Vec<Vec<u8>>,
    line: usize,
    column: usize,
}


impl Lexer {
    pub(crate) fn new(file: File) -> Lexer {
        let lines = BufReader::new(file)
            .lines()
            .map(|line| line.expect("Line failed").into_bytes())
            .collect();

        Lexer {
            stream: lines,
            line: 0,
            column: 0,
        }
    }

    fn next(&mut self) -> Option<char> {
        if self.stream.len() == self.line as usize {
            return None;
        }

        if self.stream[self.line].len() == self.column {
            self.line += 1;
            self.column = 0;
            return Some('\n');
        }


        let c: char = self.stream[self.line][self.column] as char;
        // println!("Char:  {} , Line:  {} , Column:  {} , Depth:  {} ", c, self.line, self.column, self.depth);
        self.column += 1;
        Some(c)
    }

    fn peek(&self) -> Option<char> {
        if self.stream.len() == self.line as usize || self.stream[self.line].len() == self.column {
            None
        } else {
            Some(self.stream[self.line][self.column] as char)
        }
    }

    pub(crate) fn lex(mut self) -> (VecDeque<Token>, Vec<String>) {
        let mut error_list: Vec<String> = Vec::new();
        let mut token_list: Vec<Token> = Vec::new();

        while let Some(c) = self.next() {

            let col = self.column as u16;
            let line = self.line as u32;
            match self.match_chars(c) {
                Ok(t) => {
                    if t != TokenType::Whitespace {
                        token_list.push(Token::new(t, line, col))
                    }
                },
                Err(e)  =>
                    error_list.push(e.to_owned())
            }
        }
        token_list.push(Token::new(TokenType::EOF, self.line as u32, self.column as u16));

        return (token_list.into(), error_list)
    }

    fn match_chars(&mut self, c: char) -> Result<TokenType, String> {

        match c {
            '\n' => return Ok(TokenType::Newline),
            '\'' => return Ok(TokenType::SingleQuote),
            '\"' => {
                let mut s = c.to_string();
                while let Some(new_char) = self.next() {
                    match new_char {
                        '\"' => break,
                        '\n' => return Err("Multiline strings not supported".to_string()),
                        _ => s.push(new_char)
                    }
                }

                return Ok(TokenType::StringLit(s))
            },
            '\\' => return Ok(TokenType::Delim),
            '(' => return Ok(TokenType::LParen),
            ')' => return Ok(TokenType::RParen),
            '[' => return Ok(TokenType::RBracket),
            ']' => return Ok(TokenType::LBracket),
            ':' => return Ok(TokenType::Colon),
            ';' => return Ok(TokenType::Newline),
            ',' => return Ok(TokenType::Comma),
            '.' => return Ok(TokenType::Dot),
            '<' => return Ok(TokenType::LT),
            '>' => return Ok(TokenType::GT),
            '@' => return Ok(TokenType::At),
            '_' => return Ok(TokenType::Underscore),
            '?' => return Ok(TokenType::Question),
            '{' => return Ok(TokenType::LBrace),
            '}' => return Ok(TokenType::RBrace),
            '!' => if let Some('=') = self.peek() {self.column += 1; Ok(TokenType::BangEq)} else {Ok(TokenType::Bang)},
            '%' => if let Some('=') = self.peek() {self.column += 1; Ok(TokenType::PercentEq)} else {Ok(TokenType::Percent)},
            '+' => if let Some('=') = self.peek() {self.column += 1; Ok(TokenType::PlusEq)} else {Ok(TokenType::Plus)},
            '*' => if let Some('=') = self.peek() {self.column += 1; Ok(TokenType::StarEq)} else {Ok(TokenType::Star)},
            '=' => if let Some('=') = self.peek() {self.column += 1; Ok(TokenType::EqualsEq)} else {Ok(TokenType::Equals)},
            '/' => { match self.peek() {
                Some('/') => {self.line += 1; self.column = 0; Ok(TokenType::Newline)},
                Some('=') => {self.next(); Ok(TokenType::SlashEq)}
                _ => {Ok(TokenType::Slash)}
            }},
            '-' => {match self.peek().expect("Early end of File") {
                'a'..='z' | 'A'..='Z' | '0'..='9' => Ok(TokenType::Negate),
                '>' => {self.next(); Ok(TokenType::Arrow)},
                '=' => {self.next(); Ok(TokenType::MinusEq)},
                ' ' => {Ok(TokenType::Minus)},
                _ => Err(format!("Unexpected character {}:{}", self.line, self.column)),
            }},
            'a'..='z' | 'A'..='Z' => {
                let mut ident = c.to_string();

                while let Some(new_char) = self.next() {
                    match new_char {
                        'a'..='z' | 'A'..='Z' | '0'..='9' | '_' | '-' => { ident.push(new_char) }
                        ' ' | '\n' | '(' | ')' | '.' | ',' | '{' | '}' | ':' | '<' | '>' | '[' | ']' | '@' => {
                            if self.column != 0 {
                                self.column -= 1;
                            }
                            break;
                        },
                        _ => { return Err(format!("Illegal Identifier character: \'{}\'    {}:{}", &new_char, self.line, self.column)); },
                    }
                }

                Ok(TokenType::from_str(&ident).unwrap())

            },
            '0'..='9' => {
                let mut num_lit = c.to_string();
                let mut floating = false;

                loop {
                    if let Some(new_char) = self.next() {
                        match new_char {
                            '0'..='9' => { num_lit.push(new_char)},
                            '_' => {},
                            '.' => {
                                num_lit.push(new_char);
                                floating = true;
                                match self.peek().unwrap() {
                                    '0'..='9' => {},
                                    _ => {return Err(format!("Float cannot end in '.' {}:{}", self.line, self.column))},
                                }
                            }
                            _ => {break}
                        }
                    }
                }
                if self.column != 0 {
                    self.column -= 1;
                }

                if floating {
                    if let Ok(n) = num_lit.parse::<f64>() {
                        Ok(TokenType::FloatLit(Box::new(n)))
                    } else {
                        Err(format!("Cannot parse Float Literal {}:{}", self.line, self.column))
                    }
                } else {
                    if let Ok(n) = num_lit.parse::<i64>() {
                        Ok(TokenType::IntLit(Box::new(n)))
                    } else {
                        Err(format!("Cannot parse Int Literal {}:{}", self.line, self.column))
                    }
                }
            },

            c if c.is_ascii_whitespace() => {Ok(TokenType::Whitespace)},

            _ => return Err(format!("Unexpected atom: {} {}:{}", c, self.line, self.column))
        }
    }

}

