use crate::ast::{Expr, Visitor};
use crate::token::Token;
use crate::token::types::Literal;

pub struct AstFormatter;

impl AstFormatter
{
    fn format(&self, e: &Expr) -> String
    {
       return e.accept(self);
    }

    fn parenthesize(&self, name: &str, expressions: Vec<&Expr>) -> String
    {
        let mut builder = String::new();

        builder.push('(');
        builder.push_str(name);
        for e in expressions
        {
            builder.push(' ');
            builder.push_str(&e.accept(self));
        }

        builder.push(')');

        return builder;
    }
}

impl Visitor<String> for AstFormatter
{
    fn visit_binary_expr(&self, left: &Expr, operator: &Token, right: &Expr) -> String
    {
        return self.parenthesize(&operator.lexeme, vec![left, right]);
    }

    fn visit_grouping_expr(&self, expression: &Expr) -> String
    {
        return self.parenthesize("group", vec![expression])
    }

    fn visit_literal_expr(&self, literal: &Literal) -> String
    {
        return format!("{}", literal);
    }

    fn visit_unary_expr(&self, operator: &Token, right: &Expr) -> String
    {
        return self.parenthesize(&operator.lexeme, vec![right])
    }
}

#[cfg(test)]
mod test
{
    use crate::ast::*;
    use crate::token::types::Literal;
    use crate::token::Token;
    use crate::token::types::TokenKind;

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

        let ast_formatter: AstFormatter = AstFormatter{};

        for e in expressions
        {
            let repr = ast_formatter.format(&e.lit);

            assert_eq!(repr, e.repr);
        }
    }

    #[test]
    fn should_print_unary_expr()
    {
        let ast_formatter: AstFormatter = AstFormatter{};

        let right = Expr::new_literal(Literal::Number(432.1));
        let unary_expr = &Expr::new_unary(Token::new(TokenKind::Minus, "-", None, 0), right);

        let repr = ast_formatter.format(unary_expr);

        assert_eq!(repr, "(- 432.1)");
    }

    #[test]
    fn should_print_grouping_expr()
    {
        let ast_formatter: AstFormatter = AstFormatter{};

        let right = Expr::new_literal(Literal::Number(432.1));
        let unary_expr = Expr::new_unary(Token::new(TokenKind::Minus, "-", None, 0), right);

        let grouping = Expr::new_grouping(unary_expr);

        let repr = ast_formatter.format(&grouping);

        assert_eq!(repr, "(group (- 432.1))");
    }

    #[test]
    fn should_print_binary_expr()
    {
        let ast_formatter: AstFormatter = AstFormatter{};

        let minus = Token::new(TokenKind::Minus, "-", None, 0);
        let left = Expr::new_unary(minus, Expr::new_literal(Literal::Number(123.0)));

        let operator = Token::new(TokenKind::Star, "*", None, 0);

        let right = Expr::new_grouping(Expr::new_literal(Literal::Number(45.67)));

        let binary_expr = Expr::new_binary(left, operator, right);
        let repr = ast_formatter.format(&binary_expr);

        assert_eq!(repr, "(* (- 123) (group 45.67))");
    }
}
