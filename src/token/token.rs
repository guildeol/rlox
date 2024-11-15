use crate::token::types::TokenKind;
use crate::token::types::Literal;

#[derive(Debug)]
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

    pub fn to_string(&self) -> String
    {
        let literal_str = match &self.literal
        {
            Some(lit) => format!("{:?}", lit),
            None => "None".to_string(),
        };

        return format!("{:?} {} {}", self.kind, self.lexeme, literal_str);
    }
}
