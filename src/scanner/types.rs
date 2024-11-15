use crate::token::types::TokenKind;

pub struct Keyword
{
    pub key: &'static str,
    pub value: TokenKind,
}

impl Keyword
{
    pub const fn new(lexeme: &'static str, kind: TokenKind) -> Self
    {
        return Keyword{key: lexeme, value: kind};
    }
}

pub trait ScanningErrorHandler
{
    fn callback(&mut self, line: u32, message: &str);
}


