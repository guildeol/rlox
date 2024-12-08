use crate::ast::expr;
use crate::ast::expr::Expr;
use crate::token::types::{Literal, TokenKind};
use crate::token::Token;

pub struct Interpreter;

#[derive(PartialEq)]
enum Interpretable {
    String(String),
    Number(f64),
    Boolean(bool),
    Nil,
}

impl Interpretable {
    fn is_truthy(&self) -> bool {
        match self {
            Interpretable::Boolean(b) => return *b,
            Interpretable::Nil => return false,
            _ => return true,
        }
    }
}

impl Interpreter {
    fn evaluate(&self, expression: &Expr) -> Interpretable {
        return expression.accept(self);
    }
}

impl expr::Visitor<Interpretable> for Interpreter {
    fn visit_binary_expr(&self, left: &Expr, operator: &Token, right: &Expr) -> Interpretable {
        let l_eval = self.evaluate(left);
        let r_eval = self.evaluate(right);

        match(&operator.kind, l_eval, r_eval) {
            (TokenKind::Minus, Interpretable::Number(left_number), Interpretable::Number(right_number)) => {
                    return Interpretable::Number(left_number - right_number)
            }

            (TokenKind::Star, Interpretable::Number(left_number), Interpretable::Number(right_number)) => {
                return Interpretable::Number(left_number * right_number)
            }

            (TokenKind::Slash, Interpretable::Number(left_number), Interpretable::Number(right_number)) => {
                return Interpretable::Number(left_number / right_number)
            }

            (TokenKind::Plus, Interpretable::Number(left_number), Interpretable::Number(right_number)) => {
                return Interpretable::Number(left_number / right_number)
            }

            (TokenKind::Plus, Interpretable::String(s_left), Interpretable::String(s_right)) => {
                return Interpretable::String(format!("{}{}", s_left, s_right));
            }

            (TokenKind::Greater, Interpretable::Number(left_number), Interpretable::Number(right_number)) => {
                return Interpretable::Boolean(left_number > right_number)
            }

            (TokenKind::GreaterEqual, Interpretable::Number(left_number), Interpretable::Number(right_number)) => {
                return Interpretable::Boolean(left_number >= right_number)
            }

            (TokenKind::Less, Interpretable::Number(left_number), Interpretable::Number(right_number)) => {
                return Interpretable::Boolean(left_number < right_number)
            }

            (TokenKind::LessEqual, Interpretable::Number(left_number), Interpretable::Number(right_number)) => {
                return Interpretable::Boolean(left_number <= right_number)
            }

            (TokenKind::BangEqual, l, r) =>
            {
                return Interpretable::Boolean(l != r);
            }

            (TokenKind::EqualEqual, l, r) =>
            {
                return Interpretable::Boolean(l != r);
            }

            (_, _, _) => unreachable!(
                "Cannot apply {} to {:?} and {:?}",
                operator.lexeme, left, right
            ),

           _ => unreachable!("Unsupported binary operator {}.", operator.lexeme),
        }
    }

    fn visit_grouping_expr(&self, expression: &Expr) -> Interpretable {
        return self.evaluate(expression);
    }

    fn visit_literal_expr(&self, value: &Literal) -> Interpretable {
        match value {
            Literal::Number(n) => return Interpretable::Number(*n),
            Literal::Boolean(b) => return Interpretable::Boolean(*b),
            Literal::String(s) => return Interpretable::String(s.clone()),
            Literal::Nil => return Interpretable::Nil,
        }
    }

    fn visit_unary_expr(&self, operator: &Token, expr: &Expr) -> Interpretable {
        let right = self.evaluate(expr);

        match operator.kind {
            TokenKind::Minus => match right {
                Interpretable::Number(n) => return Interpretable::Number(-n),
                _ => unreachable!("Cannot handle '-' on non-number type"),
            },

            TokenKind::Bang => {
                return Interpretable::Boolean(!right.is_truthy());
            }

            _ => unreachable!("Tried to match unexpected operator for unary expression."),
        }
    }
}
