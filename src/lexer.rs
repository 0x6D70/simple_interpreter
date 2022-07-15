
#[derive(Debug, PartialEq)]
pub enum TokenType {
    Integer(isize),
    Plus,
    Eof
}

#[derive(Debug)]
pub struct Interpreter<'a> {
    text: &'a str,
    pos: usize,
    current_token: TokenType,
}

impl<'a> Interpreter<'a> {
    pub fn new(text: &str) -> Interpreter {
        Interpreter {
            text: text,
            pos: 0,
            current_token: TokenType::Eof,
        }
    }

    fn error(&self) -> ! {
        panic!("Error parsing input: {:#?}", self);
    }

    fn get_next_token(&mut self) -> TokenType {
        if self.pos >= self.text.len() {
            return TokenType::Eof;
        }

        let current_char = self.text.chars().nth(self.pos).unwrap();

        if current_char.is_digit(10) {
            self.pos += 1;
            return TokenType::Integer(current_char.to_digit(10).unwrap() as isize);
        }

        if current_char == '+' {
            self.pos += 1;
            return TokenType::Plus;
        }

        self.error()
    }

    fn eat(&mut self, token_type: TokenType) {
        if self.current_token == token_type {
            self.current_token = self.get_next_token();
        } else {
            self.error();
        }
    }

    pub fn expr(&mut self) -> isize {
        
        self.current_token = self.get_next_token();

        let left = match self.current_token {
            TokenType::Integer(value) => value,
            _ => self.error(),
        };

        self.eat(TokenType::Integer(left));

        let op = &self.current_token;

        self.eat(TokenType::Plus);

        let right = match self.current_token {
            TokenType::Integer(value) => value,
            _ => self.error(),
        };

        self.eat(TokenType::Integer(right));

        let result = left + right;

        result
    }

}
