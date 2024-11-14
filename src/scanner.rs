use crate::token::Token;
use crate::token::types::{Literal, TokenKind};

pub trait ScanningErrorHandler
{
    fn callback(&mut self, line: u32, message: &str);
}

pub struct Scanner<ErrorHandler: ScanningErrorHandler>
{
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
    error_handler: ErrorHandler,
}

impl<ErrorHandler: ScanningErrorHandler> Scanner<ErrorHandler>
{
    pub fn new(source: String, error_handler: ErrorHandler) -> Self
    {
        return Scanner {
            source: source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
            error_handler: error_handler,
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

            _ => self.error_handler.callback(self.line as u32, "Unexpected character"),
        }
    }

    fn add_token(&mut self, kind: TokenKind, literal: Option<Literal>)
    {
        let text = &self.source[self.start..self.current];

        let new_token = Token::new(kind, text.to_string(), literal, self.line);

        self.tokens.push(new_token);
    }
}

#[cfg(test)]
mod test
{
    use super::*;

    #[derive(Debug, PartialEq)]
    struct ErrorSpy
    {
        line: u32,
        message: String,
    }

    impl ScanningErrorHandler for ErrorSpy
    {
        fn callback(&mut self, line: u32, message: &str)
        {
            self.line = line;
            self.message = message.to_string();
        }
    }

    #[test]
    fn should_get_single_char_token()
    {
        let error_spy: ErrorSpy = ErrorSpy{line: 0, message: "".to_string()};

        let mut scanner = Scanner::new(" (){},.-+;*".to_string(), error_spy);
        let tokens = scanner.scan_tokens();
        let expected_token_kinds:[TokenKind; 10] = [
            TokenKind::LeftParen,
            TokenKind::RightParen,
            TokenKind::LeftBrace,
            TokenKind::RightBrace,
            TokenKind::Comma,
            TokenKind::Dot,
            TokenKind::Minus,
            TokenKind::Plus,
            TokenKind::Semicolon,
            TokenKind::Star
        ];

        assert_eq!(tokens.len(), "(){},.-+;*".len());
        for (i, token) in tokens.iter().enumerate()
        {
            assert_eq!(token.kind, expected_token_kinds[i]);
        }
    }

    #[test]
    fn should_get_error_notification()
    {
        let error_spy: ErrorSpy = ErrorSpy{line: 0, message: "".to_string()};

        // Cat emoji for invalid lexeme
        let mut scanner = Scanner::new("🐱".to_string(), error_spy);
        let tokens = scanner.scan_tokens();

        assert_eq!(tokens.len(), 0);
        assert_eq!(scanner.error_handler, ErrorSpy{line: 1, message: "Unexpected character".to_string()});
    }
}
