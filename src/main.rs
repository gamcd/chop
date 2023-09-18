

mod tokens;
mod lexer;

fn main() {


    let file = std::fs::File::open("./examples/main.chop").expect("File Open Error");

    let lexer = lexer::Lexer::new(file);
    let (res, errors) = lexer.lex();

    res.iter().for_each(|tok| {
        dbg!(tok);
    });

    errors.iter().for_each(|err| {
        println!("{}", err);
    })

}
