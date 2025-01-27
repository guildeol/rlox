use std::fmt::Display;

use crate::token::{types::Literal, Token};

#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Call {
        callee: Box<Expr>,
        paren: Token,
        arguments: Vec<Expr>,
    },
    Grouping {
        expression: Box<Expr>,
    },
    LiteralValue {
        value: Literal,
    },
    Unary {
        operator: Token,
        right: Box<Expr>,
    },
    Variable {
        name: Token,
    },
    Assignment {
        name: Token,
        value: Box<Expr>,
    },
    Logical {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Nil,
}

pub trait ExprVisitor<R> {
    fn visit_binary_expr(&mut self, left: &Expr, operator: &Token, right: &Expr) -> R;
    fn visit_call_expr(&mut self, callee: &Expr, paren: &Token, arguments: &Vec<Expr>) -> R;
    fn visit_grouping_expr(&mut self, expression: &Expr) -> R;
    fn visit_literal_expr(&mut self, value: &Literal) -> R;
    fn visit_unary_expr(&mut self, operator: &Token, right: &Expr) -> R;
    fn visit_variable_expr(&mut self, name: &Token) -> R;
    fn visit_assignment_expr(&mut self, name: &Token, value: &Expr) -> R;
    fn visit_logical_expr(&mut self, left: &Expr, operator: &Token, right: &Expr) -> R;
}

impl Expr {
    pub fn new_binary(left: Expr, operator: Token, right: Expr) -> Self {
        return Expr::Binary {
            left: Box::new(left),
            operator: operator,
            right: Box::new(right),
        };
    }

    pub fn new_call(callee: Expr, paren: Token, arguments: Vec<Expr>) -> Self {
        return Expr::Call {
            callee: Box::new(callee),
            paren: paren,
            arguments,
        };
    }

    pub fn new_grouping(expression: Expr) -> Self {
        return Expr::Grouping {
            expression: Box::new(expression),
        };
    }

    pub fn new_literal(value: Literal) -> Self {
        return Expr::LiteralValue { value };
    }

    pub fn new_unary(operator: Token, right: Expr) -> Self {
        return Expr::Unary {
            operator: operator,
            right: Box::new(right),
        };
    }

    pub fn new_variable(name: Token) -> Self {
        return Expr::Variable { name: name };
    }

    pub fn new_assignment(name: Token, value: Expr) -> Self {
        return Expr::Assignment {
            name: name,
            value: Box::new(value),
        };
    }

    pub fn new_logical(left: Expr, operator: Token, right: Expr) -> Self {
        return Expr::Logical {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        };
    }

    pub fn accept<R>(&self, visitor: &mut dyn ExprVisitor<R>) -> R {
        match self {
            Expr::Binary { left, operator, right } => visitor.visit_binary_expr(left, operator, right),
            Expr::Call { callee, paren, arguments } => visitor.visit_call_expr(callee, paren, arguments),
            Expr::Grouping { expression } => visitor.visit_grouping_expr(expression),
            Expr::LiteralValue { value } => visitor.visit_literal_expr(value),
            Expr::Unary { operator, right } => visitor.visit_unary_expr(operator, right),
            Expr::Variable { name } => visitor.visit_variable_expr(name),
            Expr::Assignment { name, value } => visitor.visit_assignment_expr(name, value),
            Expr::Logical { left, operator, right } => visitor.visit_logical_expr(left, operator, right),
            Expr::Nil => panic!("Can't visit Nil expression!"),
        }
    }
}

impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Binary { left, operator, right } => {
                return write!(f, "({} {} {})", operator.lexeme, left, right);
            }
            Expr::Call {
                callee,
                paren: _,
                arguments,
            } => {
                write!(f, "(call {} [", callee)?;
                for (i, arg) in arguments.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", arg)?;
                }
                write!(f, "])")
            }
            Expr::Grouping { expression } => {
                return write!(f, "(group {})", expression);
            }
            Expr::LiteralValue { value } => {
                return write!(f, "{}", value);
            }
            Expr::Unary { operator, right } => {
                return write!(f, "({} {})", operator.lexeme, right);
            }
            Expr::Variable { name } => {
                return write!(f, "{}", name);
            }
            Expr::Assignment { name, value } => {
                return write!(f, "{} = {}", name, value);
            }
            Expr::Logical { left, operator, right } => {
                return write!(f, "{} {} {}", left, operator.lexeme, right);
            }
            Expr::Nil => {
                return write!(f, "Nil");
            }
        }
    }
}
