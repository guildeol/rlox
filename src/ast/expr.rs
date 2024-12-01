use crate::token::{types::Literal, Token};

pub enum Expr
{
    Binary
    {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Grouping {
        expression: Box<Expr>,
    },
    LiteralValue
    {
        value: Literal
    },
    Unary {
        operator: Token,
        right: Box<Expr>,
    },
}

pub trait Visitor<R> {
    fn visit_binary_expr(&self, left: &Expr, operator: &Token, right: &Expr) -> R;
    fn visit_grouping_expr(&self, expression: &Expr) -> R;
    fn visit_literal_expr(&self, value: &Literal) -> R;
    fn visit_unary_expr(&self, operator: &Token, right: &Expr) -> R;
}

impl Expr
{
    pub fn new_binary(left: Expr, operator: Token, right: Expr) -> Self
    {
        return Expr::Binary { left: Box::new(left), operator: operator, right: Box::new(right) };
    }

    pub fn new_grouping(expression: Expr) -> Self
    {
        return Expr::Grouping { expression: Box::new(expression) };
    }

    pub fn new_literal(value: Literal) -> Self
    {
        return Expr::LiteralValue { value }
    }

    pub fn new_unary(operator: Token, right: Expr) -> Self
    {
        return Expr::Unary { operator: operator, right: Box::new(right) };
    }

    pub fn accept<R>(&self, visitor: &dyn Visitor<R>) -> R {
        match self {
            Expr::Binary {left, operator, right} => visitor.visit_binary_expr(left, operator, right),
            Expr::Grouping { expression } => visitor.visit_grouping_expr(expression),
            Expr::LiteralValue { value } => visitor.visit_literal_expr(value),
            Expr::Unary {operator, right } => visitor.visit_unary_expr(operator, right),

            _ => todo!()
        }
    }
}