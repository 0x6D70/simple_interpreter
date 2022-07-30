use crate::lexer::token::{Token, TokenType};
use crate::reporter;

#[macro_export]
macro_rules! match_tokens {
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

        let left = self.consume_token(TokenType::Int, "expected integer");
        let mut result: isize = left.lexeme.parse().unwrap();

        while !self.is_at_end() {    
            if !match_tokens!(self, TokenType::Plus, TokenType::Minus, TokenType::Slash, TokenType::Star) {
                Self::error(self.peek(), "expected plus, minus, mul or div");
            }
    
            let op = self.previous();
    
            let right = self.consume_token(TokenType::Int, "expected integer");
            let right_value: isize = right.lexeme.parse().unwrap();
    
            result = match op.token_type {
                TokenType::Plus => result + right_value,
                TokenType::Minus => result - right_value,
                TokenType::Star => result * right_value,
                TokenType::Slash => result / right_value,
                _ => unreachable!()
            };
        }

        println!("{}", result);
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
