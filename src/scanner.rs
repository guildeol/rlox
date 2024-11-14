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

    fn is_next_char(&mut self, candidate: char) -> bool
    {
        if self.is_at_end()
        {
            return false;
        }

        if self.source.chars().nth(self.current) != Some(candidate)
        {
            return false;
        }

        self.current = self.current + 1;
        return true;
    }

    fn peek(&mut self) -> char
    {
        if self.is_at_end()
        {
            return '\0'
        };

        return self.source.chars().nth(self.current).expect("Ran out of characters!");
    }

    fn add_token(&mut self, kind: TokenKind, literal: Option<Literal>)
    {
        let text = &self.source[self.start..self.current];

        let new_token = Token::new(kind, text.to_string(), literal, self.line);

        self.tokens.push(new_token);
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

            Some('!') =>
            {
                if self.is_next_char('=')
                {
                    self.add_token(TokenKind::BangEqual, None);
                }
                else
                {
                    self.add_token(TokenKind::Bang, None);
                }
            }
            Some('=') =>
            {
                if self.is_next_char('=')
                {
                    self.add_token(TokenKind::EqualEqual, None);
                }
                else
                {
                    self.add_token(TokenKind::Equal, None);
                }
            }
            Some('<') =>
            {
                if self.is_next_char('=')
                {
                    self.add_token(TokenKind::LessEqual, None);
                }
                else
                {
                    self.add_token(TokenKind::Less, None);
                }
            }
            Some('>') =>
            {
                if self.is_next_char('=')
                {
                    self.add_token(TokenKind::GreaterEqual, None);
                }
                else
                {
                    self.add_token(TokenKind::Greater, None);
                }
            }
            Some('/') =>
            {
                if self.is_next_char('/')
                {
                    while (self.peek() != '\n' && !self.is_at_end())
                    {
                        self.advance();
                    }
                }
                else
                {
                    self.add_token(TokenKind::Slash, None);
                }
            }

            // Whitespace
            Some(' ' | '\r' | '\t') =>
            {
                // Ignore
            }
            Some('\n') =>
            {
                self.line = self.line + 1;
            }

            // Default: call the error handler
            _ => self.error_handler.callback(self.line as u32, "Unexpected character"),
        }
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
        had_error: bool,
    }

    impl ScanningErrorHandler for ErrorSpy
    {
        fn callback(&mut self, line: u32, message: &str)
        {
            self.had_error = true;
            self.line = line;
            self.message = message.to_string();
        }
    }

    struct TokenKindPair
    {
        symbol: String,
        kind: TokenKind,
    }

    impl TokenKindPair
    {
        fn new(symbol: &str, kind: TokenKind) -> Self
        {
            return TokenKindPair{symbol: symbol.to_string(), kind: kind}
        }
    }

    #[test]
    fn should_get_tokens()
    {
        let expected_tokens:Vec<TokenKindPair> = vec![
            TokenKindPair::new("(", TokenKind::LeftParen),
            TokenKindPair::new(")", TokenKind::RightParen),
            TokenKindPair::new("{", TokenKind::LeftBrace),
            TokenKindPair::new("}", TokenKind::RightBrace),
            TokenKindPair::new(",", TokenKind::Comma),
            TokenKindPair::new(".", TokenKind::Dot),
            TokenKindPair::new("-", TokenKind::Minus),
            TokenKindPair::new("+", TokenKind::Plus),
            TokenKindPair::new(";", TokenKind::Semicolon),
            TokenKindPair::new("*", TokenKind::Star),
            TokenKindPair::new("!", TokenKind::Bang),
            TokenKindPair::new("!=", TokenKind::BangEqual),
            TokenKindPair::new("=", TokenKind::Equal),
            TokenKindPair::new("==", TokenKind::EqualEqual),
            TokenKindPair::new("<", TokenKind::Less),
            TokenKindPair::new("<=", TokenKind::LessEqual),
            TokenKindPair::new(">", TokenKind::Greater),
            TokenKindPair::new(">=", TokenKind::GreaterEqual),
            TokenKindPair::new("/", TokenKind::Slash),
        ];

        for token in expected_tokens
        {
            let error_spy: ErrorSpy = ErrorSpy{line: 0, message: "".to_string(), had_error: false};
            let mut scanner = Scanner::new(token.symbol.clone(), error_spy);
            let tokens = scanner.scan_tokens();

            assert_eq!(tokens.len(), 1);
            assert_eq!(token.kind, tokens[0].kind);
            assert_eq!(scanner.error_handler.had_error, false);
        }
    }

    #[test]
    fn should_advance_on_newline()
    {
        let error_spy: ErrorSpy = ErrorSpy{line: 0, message: "".to_string(), had_error: false};
        let mut scanner = Scanner::new("*\n*".to_string(),
                                                          error_spy);
        scanner.scan_tokens();

        assert_eq!(scanner.line, 2);
    }

    #[test]
    fn should_ignore_commented_line()
    {
        let error_spy: ErrorSpy = ErrorSpy{line: 0, message: "".to_string(), had_error: false};
        let mut scanner = Scanner::new("// These are \n //coments!".to_string(),
                                                          error_spy);
        let tokens = scanner.scan_tokens();

        assert_eq!(tokens.len(), 0);
        assert_eq!(scanner.error_handler.had_error, false);
    }

    #[test]
    fn should_get_error_notification()
    {
        let error_spy: ErrorSpy = ErrorSpy{line: 0, message: "".to_string(), had_error: false};

        // Cat emoji for invalid lexeme
        let mut scanner = Scanner::new("🐱".to_string(), error_spy);
        let tokens = scanner.scan_tokens();

        assert_eq!(tokens.len(), 0);
        assert_eq!(scanner.error_handler.had_error, true);
    }
}
