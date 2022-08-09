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

#[allow(dead_code)]
pub fn evaluate_rpn(expr: Expr) {
    match expr {
        Expr::Binary { left, op, right } => {
            evaluate_rpn(*left);
            evaluate_rpn(*right);

            match op {
                TokenType::Plus => print!(" +"),
                TokenType::Minus => print!(" -"),
                TokenType::Star => print!(" *"),
                TokenType::Slash => print!(" /"),
                TokenType::Power => print!(" ^"),
                _ => unreachable!(),
            }
        }
        Expr::Grouping(expr_grouping) => {
            evaluate_rpn(*expr_grouping);
        }
        Expr::Int(value) => {
            print!(" {}", value);
        }
    }
}

#[allow(dead_code)]
pub fn evaluate_lisp(expr: Expr) {
    match expr {
        Expr::Binary { left, op, right } => {
            print!("(");
            match op {
                TokenType::Plus => print!(" +"),
                TokenType::Minus => print!(" -"),
                TokenType::Star => print!(" *"),
                TokenType::Slash => print!(" /"),
                TokenType::Power => print!(" ^"),
                _ => unreachable!(),
            }

            evaluate_lisp(*left);
            evaluate_lisp(*right);

            print!(")");
        }
        Expr::Grouping(expr_grouping) => {
            evaluate_lisp(*expr_grouping);
        }
        Expr::Int(value) => {
            print!(" {}", value);
        }
    }
}
