use crate::token::Token;
use crate::token::types::{Literal, TokenKind};

pub struct Scanner
{
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner
{
    pub fn new(source: String) -> Scanner
    {
        return Scanner {
            source: source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        };
    }

    pub fn scan_tokens(&mut self) -> &Vec<Token>
    {
        while !self.is_at_end()
        {
            self.start = self.current;

            self.scan_single_token();
        }

        return &self.tokens;
    }

    fn is_at_end(&self) -> bool
    {
        return self.current >= self.source.len();
    }

    fn advance(&mut self) -> Option<char>
    {
        let c = self.source.chars().nth(self.current);
        self.current = self.current + 1;

        return c;
    }

    fn scan_single_token(&mut self)
    {
        match self.advance()
        {
            Some('(') => self.add_token(TokenKind::LeftParen, None),
            Some(')') => self.add_token(TokenKind::RightParen, None),
            Some('{') => self.add_token(TokenKind::LeftBrace, None),
            Some('}') => self.add_token(TokenKind::RightBrace, None),
            Some(',') => self.add_token(TokenKind::Comma, None),
            Some('.') => self.add_token(TokenKind::Dot, None),
            Some('-') => self.add_token(TokenKind::Minus, None),
            Some('+') => self.add_token(TokenKind::Plus, None),
            Some(';') => self.add_token(TokenKind::Semicolon, None),
            Some('*') => self.add_token(TokenKind::Star, None),

            _ => super::error(self.line as u32, "Unexpected character"),
        }
    }

    fn add_token(&mut self, kind: TokenKind, literal: Option<Literal>)
    {
        let text = &self.source[self.start..self.current];

        let new_token = Token::new(kind, text.to_string(), literal, self.line);

        self.tokens.push(new_token);
    }

}
