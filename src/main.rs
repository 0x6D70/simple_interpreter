mod lexer;

use std::io::Write;

fn main() {
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

        let mut interpreter = lexer::Interpreter::new(input);
        let result = interpreter.expr();
        println!("{}", result);
    }
}
