use crate::token::types::TokenKind;
use crate::token::types::Literal;

pub struct Token
{
    pub kind: TokenKind,
    pub lexeme: String,
    pub literal: Option<Literal>,
    pub line: u32,
}

impl Token
{
    pub fn new(kind: TokenKind, lexeme: String, literal: Option<Literal>, line: u32) -> Self
    {
        return Token {kind, lexeme, literal, line};
    }
}
