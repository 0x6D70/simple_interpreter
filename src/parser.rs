use crate::lexer::token::{Token, TokenType};
use crate::reporter;

#[macro_export]
macro_rules! match_multiple_tokens {
    ( $parser:expr, $( $x:expr ),* ) => {
        {
            let mut ret = false;
            $(
                if $parser.check($x) {
                    $parser.advance();
                    ret = true;
                }
            )*
            ret
        }
    };
}

#[macro_export]
macro_rules! is_token {
    ( $parser:expr, $( $x:expr ),* ) => {
        {
            let mut ret = false;
            $(
                if $parser.check($x) {
                    ret = true;
                }
            )*
            ret
        }
    };
}

#[derive(Debug)]
pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, current: 0 }
    }

    pub fn parse(&mut self) {
        let result = self.expr();

        println!("{}", result);
    }

    fn expr(&mut self) -> isize {
        let mut left = self.term();

        while is_token!(self, TokenType::Plus, TokenType::Minus) {
            let op = self.advance();
            let right = self.term();

            left = match op.token_type {
                TokenType::Plus => left + right,
                TokenType::Minus => left - right,
                _ => unreachable!(),
            };
        }

        left
    }

    fn term(&mut self) -> isize {
        let mut left = self.factor();

        while is_token!(self, TokenType::Star, TokenType::Slash) {
            let op = self.advance();
            let right = self.factor();

            left = match op.token_type {
                TokenType::Star => left * right,
                TokenType::Slash => left / right,
                _ => unreachable!(),
            };
        }

        left
    }

    fn factor(&mut self) -> isize {
        let mut left = self.number();

        while is_token!(self, TokenType::Power) {
            let op = self.advance();
            let right = self.factor(); // maybe use factor() here

            left = match op.token_type {
                TokenType::Power => left.pow(right as u32),
                _ => unreachable!(),
            };
        }

        left
    }

    fn number(&mut self) -> isize {
        if is_token!(self, TokenType::Leftparen) {
            self.advance();
            let value = self.expr();
            self.consume_token(TokenType::Rightparen, "expected closing parenthesis");
            return value;
        } else if is_token!(self, TokenType::Int) {
            let token = self.consume_token(TokenType::Int, "expected integer");
            return token.lexeme.parse().unwrap();
        }

        Self::error(self.advance(), "expected parenthesis or integer");
    }

    fn consume_token(&mut self, token_type: TokenType, msg: &str) -> Token {
        if self.check(token_type) {
            return self.advance();
        }

        Self::error(self.peek(), msg);
    }

    fn error(token: Token, msg: &str) -> ! {
        // TODO: properly report file and line numbe here
        reporter::report_error(format!("{:#?} {}", token, msg).as_str(), "", token.line);
        panic!("parsing error");
    }

    fn check(&mut self, token_type: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }

        self.peek().token_type == token_type
    }

    fn previous(&self) -> Token {
        self.tokens[self.current - 1].clone()
    }

    fn peek(&self) -> Token {
        self.tokens[self.current].clone()
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }

        self.previous()
    }

    fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::Eof
    }
}
