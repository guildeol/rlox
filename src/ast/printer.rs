use crate::ast::{Expr, Visitor};
use crate::token::Token;
use crate::token::types::Literal;

pub struct AstFormatter;

impl AstFormatter
{
    pub fn format(&mut self, e: &Expr) -> String
    {
       return e.accept(self);
    }

    pub fn println(&mut self, e: &Expr)
    {
        println!("{}", e.accept(self));
    }
}

impl Visitor<String> for AstFormatter
{
    fn visit_binary_expr(&self, left: &Expr, operator: &Token, right: &Expr) -> String
    {
        todo!()
    }

    fn visit_grouping_expr(&self, expression: &Expr) -> String
    {
        todo!()
    }

    fn visit_literal_expr(&self, literal: &Literal) -> String
    {
        return format!("{}", literal);
    }

    fn visit_unary_expr(&self, operator: &Token, right: &Expr) -> String
    {
        todo!()
    }
}

#[cfg(test)]
mod test
{
    use crate::ast::*;
    use crate::token::types::Literal;

    #[test]
    fn should_print_literal_expr()
    {
        struct LiteralReprPair
        {
            lit: crate::ast::expr::Expr,
            repr: String,
        }

        impl LiteralReprPair
        {
            fn new(lit: Literal, repr: &str) -> Self
            {
                return LiteralReprPair {lit: Expr::new_literal(lit), repr: repr.to_string()};
            }
        }

        let expressions = vec![
            LiteralReprPair::new(Literal::String("test".to_string()), "\"test\""),
            LiteralReprPair::new(Literal::Number(197.0), "197"),
            LiteralReprPair::new(Literal::Boolean(false), "false"),
            LiteralReprPair::new(Literal::Nil, "Nil"),
        ];

        let mut ast_formatter: AstFormatter = AstFormatter{};

        for e in expressions
        {
            let repr = ast_formatter.format(&e.lit);
            assert_eq!(repr, e.repr);
        }
    }
}
