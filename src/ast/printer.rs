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
        let mut ast_formatter: AstFormatter = AstFormatter{};
        let expression = LiteralValue::new(Literal::Nil);
        let repr = ast_formatter.format(&expression);

        assert_eq!(repr, "Nil");
    }
}
