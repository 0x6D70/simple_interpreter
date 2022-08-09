use crate::lexer::token::{Token, TokenType};
use crate::{evaluate, reporter};

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
pub enum Expr {
    Binary {
        left: Box<Expr>,
        op: TokenType,
        right: Box<Expr>,
    },
    Unary {
        op: TokenType,
        right: Box<Expr>,
    },
    Grouping(Box<Expr>),
    Int(isize),
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

        println!("{:#?}", result);
        println!("{}", evaluate::evaluate(result));
    }

    fn expr(&mut self) -> Expr {
        let mut left = self.term();

        while is_token!(self, TokenType::Plus, TokenType::Minus) {
            let op = self.advance();
            let right = self.term();

            left = Expr::Binary {
                left: Box::new(left),
                op: op.token_type,
                right: Box::new(right),
            };
        }

        left
    }

    fn term(&mut self) -> Expr {
        let mut left = self.factor();

        while is_token!(self, TokenType::Star, TokenType::Slash) {
            let op = self.advance();
            let right = self.factor();

            left = Expr::Binary {
                left: Box::new(left),
                op: op.token_type,
                right: Box::new(right),
            };
        }

        left
    }

    fn factor(&mut self) -> Expr {
        let mut left = self.number();

        while is_token!(self, TokenType::Power) {
            let op = self.advance();
            let right = self.factor();

            left = Expr::Binary {
                left: Box::new(left),
                op: op.token_type,
                right: Box::new(right),
            };
        }

        left
    }

    fn number(&mut self) -> Expr {
        if is_token!(self, TokenType::Leftparen) {
            self.advance();
            let value = Expr::Grouping(Box::new(self.expr()));
            self.consume_token(TokenType::Rightparen, "expected closing parenthesis");
            return value;
        } else if is_token!(self, TokenType::Int) {
            let token = self.consume_token(TokenType::Int, "expected integer");
            return Expr::Int(token.lexeme.parse().unwrap());
        } else if is_token!(self, TokenType::Minus, TokenType::Plus) {
            let op = self.advance(); // skip the minus
            let value = self.factor();
            return Expr::Unary {
                op: op.token_type,
                right: Box::new(value),
            };
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
