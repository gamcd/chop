use crate::parser::{Parse, Parser};

mod tokens;
mod lexer;
mod parser;
mod abstract_syntax_tree;
mod operator;

fn main() {


    let file = std::fs::File::open("./examples/main.chop").expect("File Open Error");

    let lexer = lexer::Lexer::new(file);
    let (token_stream, error_list) = lexer.lex();

    if !error_list.is_empty() {
        for x in error_list {
            dbg!("{}", x);
        }
    }

    dbg!(&token_stream);
    let mut parser = Parser::new(token_stream);

}
