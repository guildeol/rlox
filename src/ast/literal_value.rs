use crate::ast::{Expr, Visitor};
use crate::token::types::Literal;

use std::fmt::Display;

pub struct LiteralValue
{
    pub value: Literal,
}

impl LiteralValue
{
    pub fn new(lit: Literal) -> Self
    {
        return LiteralValue{ value: lit };
    }
}

impl Display for LiteralValue
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        return write!(f, "{}", self.value);
    }
}

impl Expr for LiteralValue
{
    fn accept<V:Visitor>(&self, visitor: &mut V) -> V::Item
    {
        return visitor.visit_literal(self);
    }
}

