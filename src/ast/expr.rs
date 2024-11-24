use crate::ast::LiteralValue;

pub trait Expr
{
    fn accept<V:Visitor>(&self, visitor: &mut V) -> V::Item;
}

pub trait Visitor
{
    type Item;
    fn visit_literal(&mut self, literal: &LiteralValue) -> Self::Item;
}
