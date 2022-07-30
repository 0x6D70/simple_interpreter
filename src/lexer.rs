
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TokenType {
    Integer(isize),
    Plus,
    Minus,
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
            text,
            pos: 0,
            current_token: TokenType::Eof,
        }
    }

    fn error(&self) -> ! {
        panic!("Error parsing input: {:#?}", self);
    }

    fn integer(&mut self) -> TokenType {
        let start_index = self.pos;

        while let Some(c) = self.text.chars().nth(self.pos) {
            if !c.is_ascii_digit() {
                break;
            }

            self.pos += 1;
        }

        TokenType::Integer(self.text[start_index..self.pos].parse::<isize>().unwrap())
    }

    fn get_next_token(&mut self) -> TokenType {
        if self.pos >= self.text.len() {
            return TokenType::Eof;
        }

        let current_char = self.text.chars().nth(self.pos).unwrap();

        if current_char.is_ascii_whitespace() {
            self.pos += 1;
            return self.get_next_token();
        }

        if current_char.is_ascii_digit() {
            return self.integer();
        }

        if current_char == '+' {
            self.pos += 1;
            return TokenType::Plus;
        }

        if current_char == '-' {
            self.pos += 1;
            return TokenType::Minus;
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

        let op = self.current_token;

        if op == TokenType::Plus {
            self.eat(TokenType::Plus);
        } else {
            self.eat(TokenType::Minus);
        }

        let right = match self.current_token {
            TokenType::Integer(value) => value,
            _ => self.error(),
        };

        self.eat(TokenType::Integer(right));

        match op {
            TokenType::Plus => left + right,
            TokenType::Minus => left - right,
            _ => self.error()
        }
    }

}
