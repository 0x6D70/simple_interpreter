mod evaluate;
mod lexer;
mod parser;
mod reporter;

use std::io::Write;

use lexer::Lexer;
use parser::Parser;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() == 2 {
        run_file(&args[1]);
        return;
    }

    loop {
        print!("calc> ");
        std::io::stdout().flush().unwrap();

        // read input from stdin
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if input.is_empty() {
            continue;
        }

        let mut lexer = Lexer::from_string(input.to_string());

        let tokens = lexer.lex_tokens();

        if tokens.is_none() {
            println!("Lexer returned no tokens");
            std::process::exit(-1);
        }

        let tokens = tokens.unwrap();

        let mut parser = Parser::new(tokens);

        parser.parse();
    }
}

fn run_file(path: &str) {
    let mut lexer = Lexer::from_file(path);

    let tokens = lexer.lex_tokens();

    if tokens.is_none() {
        std::process::exit(-1);
    }

    let tokens = tokens.unwrap();

    let mut parser = Parser::new(tokens);

    parser.parse();
}
