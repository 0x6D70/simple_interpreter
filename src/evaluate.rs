use crate::{lexer::token::TokenType, parser::Expr};

pub fn evaluate(expr: Expr) -> isize {
    match expr {
        Expr::Binary { left, op, right } => {
            let left_value = evaluate(*left);
            let right_value = evaluate(*right);

            match op {
                TokenType::Plus => left_value + right_value,
                TokenType::Minus => left_value - right_value,
                TokenType::Star => left_value * right_value,
                TokenType::Slash => left_value / right_value,
                TokenType::Power => left_value.pow(right_value as u32),
                _ => unreachable!(),
            }
        }
        Expr::Grouping(expr_grouping) => evaluate(*expr_grouping),
        Expr::Int(value) => value,
    }
}
