use std::fmt::format;
use std::io::{BufRead, BufReader};
use std::fs::File;
use std::str::FromStr;
use crate::tokens::Token;

pub struct Lexer {
    pub stream: Vec<Vec<u8>>,
    line: usize,
    column: usize,
    depth: u8
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
            depth: 0,
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

    pub(crate) fn lex(mut self) -> (Vec<(Token, usize, usize)>, Vec<String>) {
        let mut error_list: Vec<String> = Vec::new();
        let mut token_list: Vec<(Token, usize, usize)> = Vec::new();

        while let Some(c) = self.next() {

            match self.match_chars(c) {
                Ok(t) =>
                    token_list.push((t, self.line, self.column)),
                Err(e)  =>
                    error_list.push(e.to_owned())
            }
        }
        token_list.push((Token::EOF, self.line, self.column));

        return (token_list, error_list)
    }

    fn match_chars(&mut self, c: char) -> Result<Token, String> {

        match c {
            '\n' => return Ok(Token::Newline),
            '\'' => return Ok(Token::SingleQuote),
            '\"' => {
                let mut s = c.to_string();
                while let Some(new_char) = self.next() {
                    match new_char {
                        '\"' => break,
                        '\n' => return Err("Multiline strings not supported".to_string()),
                        _ => s.push(new_char)
                    }
                }

                return Ok(Token::StringLit(s))
            },
            '\\' => return Ok(Token::Delim),
            '(' => return Ok(Token::LParen),
            ')' => return Ok(Token::RParen),
            '[' => return Ok(Token::RBracket),
            ']' => return Ok(Token::LBracket),
            ':' => return Ok(Token::Colon),
            ';' => return Ok(Token::SemiColon),
            ',' => return Ok(Token::Comma),
            '.' => return Ok(Token::Dot),
            '<' => return Ok(Token::LT),
            '>' => return Ok(Token::GT),
            '@' => return Ok(Token::At),
            '_' => return Ok(Token::Underscore),
            '?' => return Ok(Token::Question),
            '{' => {self.depth += 1; return Ok(Token::LBrace)},
            '}' => {self.depth -= 1; return Ok(Token::RBrace)},
            '!' => { if let Some('=') = self.peek() {self.column += 1; Ok(Token::BangEq)} else {Ok(Token::Bang)}},
            '%' => { if let Some('=') = self.peek() {self.column += 1; Ok(Token::PercentEq)} else {Ok(Token::Percent)}},
            '+' => { if let Some('=') = self.peek() {self.column += 1; Ok(Token::PlusEq)} else {Ok(Token::Plus)}},
            '*' => { if let Some('=') = self.peek() {self.column += 1; Ok(Token::StarEq)} else {Ok(Token::Star)}},
            '=' => { if let Some('=') = self.peek() {self.column += 1; Ok(Token::EqualsEq)} else {Ok(Token::Equals)}},
            '/' => { match self.peek() {
                Some('/') => {self.line += 1; self.column = 0; Ok(Token::Newline)},
                Some('=') => {self.next(); Ok(Token::SlashEq)}
                _ => {Ok(Token::Slash)}
            }},
            '-' => {match self.peek() {
                Some('>') => {self.next(); Ok(Token::Arrow)}
                Some('=') => {self.next(); Ok(Token::MinusEq)}
                _ => {Ok(Token::Minus)}
            }},
            'a'..='z' | 'A'..='Z' => {
                let mut ident = c.to_string();

                while let Some(new_char) = self.next() {
                    match new_char {
                        'a'..='z' | 'A'..='Z' | '0'..='9' | '_' | '-' => { ident.push(new_char) }
                        ' ' | '\n' | '(' | ')' | '.' | '{' | '}' | ':' | '<' | '>' | '[' | ']' => {
                            if self.column != 0 {
                                self.column -= 1;
                            }
                            break;
                        },
                        _ => { return Err(format!("Illegal Identifier character: \'{}\'", &new_char)); }
                    }
                }

                return Ok(match Token::from_str(&ident) {
                    Ok(t) => t,
                    Err(s) => Token::Ident(s)
                });

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
                        Ok(Token::FloatLit(n))
                    } else {
                        Err(format!("Cannot parse Float Literal {}:{}", self.line, self.column))
                    }
                } else {
                    if let Ok(n) = num_lit.parse::<i64>() {
                        Ok(Token::IntLit(n))
                    } else {
                        Err(format!("Cannot parse Int Literal {}:{}", self.line, self.column))
                    }
                }
            },

            ' ' => {Ok(Token::WhiteSpace)},

            _ => return Err(format!("Unexpected atom: {} {}:{}", c, self.line, self.column))
        }
    }

}

