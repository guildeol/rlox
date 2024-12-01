use crate::token::types::TokenKind;
use crate::token::types::Literal;

use std::fmt::Display;

#[derive(Clone, PartialEq)]
pub struct Token
{
    pub kind: TokenKind,
    pub lexeme: String,
    pub literal: Option<Literal>,
    pub line: u32,
}

impl Token
{
    pub fn new(kind: TokenKind, lexeme: &str, literal: Option<Literal>, line: u32) -> Self
    {
        return Token {kind, lexeme: lexeme.to_string(), literal, line};
    }
}

impl Display for Token
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        return write!(f, "{}", self.lexeme);
    }
}