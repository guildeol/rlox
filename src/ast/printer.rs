use crate::ast::{Expr, Visitor, LiteralValue};

pub struct AstFormatter;

impl AstFormatter
{
    pub fn format(&mut self, e: &impl Expr) -> String
    {
       return e.accept(self);
    }

    pub fn println(&mut self, e: &impl Expr)
    {
        println!("{}", e.accept(self));
    }
}

impl Visitor for AstFormatter
{
    type Item = String;

    fn visit_literal(&mut self, literal: &LiteralValue) -> Self::Item
    {
        return format!("{}", literal.value);
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
            lit: LiteralValue,
            repr: String,
        }

        impl LiteralReprPair
        {
            fn new(lit: Literal, repr: &str) -> Self
            {
                return LiteralReprPair {lit: LiteralValue::new(lit), repr: repr.to_string()};
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
