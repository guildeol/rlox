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
    line: u32,
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

    fn advance_if_equal(&mut self, candidate: char) -> bool
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

    fn peek_next(&mut self) -> char
    {
        if self.current + 1 >= self.source.len()
        {
            return '\0';
        }


        return self.source.chars().nth(self.current + 1).expect("Ran out of characters!");
    }

    fn get_string_literal(&mut self)
    {
        while self.peek() != '"' && !self.is_at_end()
        {
            if self.peek() == '\n'
            {
                self.line = self.line + 1;
            }

            self.advance();
        }

        if self.is_at_end()
        {
            self.error_handler.callback(self.line as u32, "Unterminated string.");
            return;
        }

        self.advance();
        let literal: &str = &self.source[self.start + 1 .. self.current - 1];

        self.add_token(TokenKind::String, Some(Literal::String(literal.to_string())));
    }

    fn get_number_literal(&mut self)
    {
        // Get the integer part
        while self.peek().is_digit(10)
        {
            self.advance();
        }

        // Look for a fractional part
        if self.peek() == '.' && self.peek_next().is_digit(10)
        {
            // Consume the '.'
            self.advance();

            // Get the real part
            while self.peek().is_digit(10)
            {
                self.advance();
            }
        }

        let lexeme = &self.source[self.start .. self.current];
        match lexeme.parse::<f64>()
        {
            Ok(value) =>
            {
                self.add_token(TokenKind::Number, Some(Literal::Number(value)));
            }

            Err(_) =>
            {
                self.error_handler.callback(self.line, "Invalid number literal");
            }
        }
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
                if self.advance_if_equal('=')
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
                if self.advance_if_equal('=')
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
                if self.advance_if_equal('=')
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
                if self.advance_if_equal('=')
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
                if self.advance_if_equal('/')
                {
                    while self.peek() != '\n' && !self.is_at_end()
                    {
                        self.advance();
                    }
                }
                else
                {
                    self.add_token(TokenKind::Slash, None);
                }
            }

            Some('"') =>
            {
                self.get_string_literal();
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

            Some(c) =>
            {
                if c.is_digit(10)
                {
                    self.get_number_literal();
                }
                else
                {
                    self.error_handler.callback(self.line as u32, "Unexpected character");
                }
            }

            None =>
            {
                self.error_handler.callback(self.line as u32, "No character retrieved")
            }
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
    fn should_get_string_literal()
    {
        let error_spy: ErrorSpy = ErrorSpy{line: 0, message: "".to_string(), had_error: false};

        // Cat emoji for invalid lexeme
        let mut scanner = Scanner::new("\"foo\"".to_string(), error_spy);
        let tokens = scanner.scan_tokens();

        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].kind, TokenKind::String);
        assert_eq!(tokens[0].literal, Some(Literal::String("foo".to_string())));
        assert_eq!(scanner.error_handler.had_error, false);
    }

    #[test]
    fn should_get_number_literal()
    {
        let error_spy: ErrorSpy = ErrorSpy{line: 0, message: "".to_string(), had_error: false};

        // Cat emoji for invalid lexeme
        let mut scanner = Scanner::new("123\n456.789".to_string(), error_spy);
        let tokens = scanner.scan_tokens();

        let expected_numbers = [123.0, 456.789];

        for (i, token) in tokens.iter().enumerate()
        {
            assert_eq!(token.kind, TokenKind::Number);
            assert_eq!(token.literal, Some(Literal::Number(expected_numbers[i])))
        }

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
