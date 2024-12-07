use crate::ast::expr::Expr;
use crate::ast::stmt::Stmt;
use crate::error::{ProcessingErrorHandler, RuntimeError};
use crate::token::types::Literal;
use crate::token::types::TokenKind;
use crate::token::Token;

pub struct Parser<'a, ErrorHandler: ProcessingErrorHandler> {
    pub tokens: Vec<Token>,
    pub current: usize,
    pub error_handler: &'a mut ErrorHandler,
}

impl<'a, ErrorHandler: ProcessingErrorHandler> Parser<'a, ErrorHandler> {
    pub fn new(tokens: Vec<Token>, error_handler: &'a mut ErrorHandler) -> Self {
        return Parser {
            tokens: tokens,
            current: 0,
            error_handler: error_handler,
        };
    }

    pub fn parse(&mut self) -> Result<Vec<Stmt>, RuntimeError> {
        let mut statements: Vec<Stmt> = Vec::new();

        while !self.is_at_end() {
            let s = self.statement()?;
            statements.push(s);
        }

        return Ok(statements);
    }

    fn expression(&mut self) -> Result<Expr, RuntimeError> {
        return self.equality();
    }

    fn statement(&mut self) -> Result<Stmt, RuntimeError> {
        if self.consume_if_one_of(vec![TokenKind::Print]) {
            return self.print_statement();
        }

        return self.expression_statement();
    }

    fn print_statement(&mut self) -> Result<Stmt, RuntimeError> {
        let expr = self.expression()?;

        self.consume(TokenKind::Semicolon, "Expect ';' after value.")?;

        return Ok(Stmt::new_print_stmt(expr));
    }

    fn expression_statement(&mut self) -> Result<Stmt, RuntimeError> {
        let expr = self.expression()?;

        self.consume(TokenKind::Semicolon, "Expect ';' after expression.")?;

        return Ok(Stmt::new_expr_stmt(expr));
    }

    fn equality(&mut self) -> Result<Expr, RuntimeError> {
        let mut expr = self.comparison()?;

        while self.consume_if_one_of(vec![TokenKind::BangEqual, TokenKind::EqualEqual]) {
            let operator = self.previous().clone();
            let right = self.comparison()?;

            expr = Expr::new_binary(expr, operator, right)
        }

        return Ok(expr);
    }

    fn consume_if_one_of(&mut self, candidates: Vec<TokenKind>) -> bool {
        for entry in candidates {
            if self.check(entry) {
                self.advance();
                return true;
            }
        }

        return false;
    }

    fn check(&mut self, candidate: TokenKind) -> bool {
        if self.is_at_end() {
            return false;
        } else {
            return self.peek().kind == candidate;
        }
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current = self.current + 1;
        }

        return self.previous();
    }

    fn is_at_end(&self) -> bool {
        return self.peek().kind == TokenKind::EndOfFile;
    }

    fn peek(&self) -> Token {
        return self.tokens[self.current].clone();
    }

    fn previous(&mut self) -> Token {
        return self.tokens[self.current - 1].clone();
    }

    fn comparison(&mut self) -> Result<Expr, RuntimeError> {
        let mut expr = self.term()?;

        while self.consume_if_one_of(vec![
            TokenKind::Greater,
            TokenKind::GreaterEqual,
            TokenKind::Less,
            TokenKind::LessEqual,
        ]) {
            let operator = self.previous();
            let right = self.term()?;

            expr = Expr::new_binary(expr, operator, right);
        }

        return Ok(expr);
    }

    fn term(&mut self) -> Result<Expr, RuntimeError> {
        let mut expr = self.factor()?;

        while self.consume_if_one_of(vec![TokenKind::Minus, TokenKind::Plus]) {
            let operator = self.previous();
            let right = self.factor()?;

            expr = Expr::new_binary(expr, operator, right);
        }

        return Ok(expr);
    }

    fn factor(&mut self) -> Result<Expr, RuntimeError> {
        let mut expr = self.unary()?;

        while self.consume_if_one_of(vec![TokenKind::Slash, TokenKind::Star]) {
            let operator = self.previous();
            let right = self.unary()?;

            expr = Expr::new_binary(expr, operator, right);
        }

        return Ok(expr);
    }

    fn unary(&mut self) -> Result<Expr, RuntimeError> {
        while self.consume_if_one_of(vec![TokenKind::Bang, TokenKind::Minus]) {
            let operator = self.previous();
            let right = self.unary()?;

            return Ok(Expr::new_unary(operator, right));
        }

        return self.primary();
    }

    fn primary(&mut self) -> Result<Expr, RuntimeError> {
        if self.consume_if_one_of(vec![TokenKind::False]) {
            return Ok(Expr::new_literal(Literal::Boolean(false)));
        }

        if self.consume_if_one_of(vec![TokenKind::True]) {
            return Ok(Expr::new_literal(Literal::Boolean(true)));
        }

        if self.consume_if_one_of(vec![TokenKind::Nil]) {
            return Ok(Expr::new_literal(Literal::Nil));
        }

        if self.consume_if_one_of(vec![TokenKind::Number, TokenKind::String]) {
            let prev = self.previous();
            return Ok(Expr::new_literal(prev.literal.unwrap()));
        }

        if self.consume_if_one_of(vec![TokenKind::LeftParen]) {
            let expr = self.expression().ok();
            self.consume(TokenKind::RightParen, "Expect ')' after expression.")?;
            return Ok(Expr::new_grouping(expr.unwrap()));
        }

        return Err(RuntimeError::parse_error(&format!(
            "Unexpected primary expression token '{}'",
            self.peek()
        )));
    }

    fn consume(&mut self, kind: TokenKind, message: &str) -> Result<Token, RuntimeError> {
        if self.check(kind) {
            return Ok(self.advance());
        } else {
            return Err(self.error(&self.peek(), message));
        }
    }

    fn error(&mut self, token: &Token, message: &str) -> RuntimeError {
        if token.kind == TokenKind::EndOfFile {
            self.error_handler
                .parsing_error(token.line, " at end", message);
        } else {
            self.error_handler.parsing_error(
                token.line,
                &format!("at '{}'", token.lexeme),
                message,
            );
        }

        return RuntimeError::parse_error("");
    }

    fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.previous().kind == TokenKind::Semicolon {
                return;
            } else {
                match self.peek().kind {
                    TokenKind::Class
                    | TokenKind::Fun
                    | TokenKind::Var
                    | TokenKind::For
                    | TokenKind::If
                    | TokenKind::While
                    | TokenKind::Print
                    | TokenKind::Return => {
                        return;
                    }

                    _ => {
                        self.advance();
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::error::ProcessingErrorHandler;
    use crate::scanner::Scanner;

    use super::*;

    #[derive(Debug, PartialEq)]
    struct ErrorSpy {
        had_error: bool,
    }

    impl ErrorSpy {
        fn new() -> Self {
            return ErrorSpy { had_error: false };
        }
    }

    impl ProcessingErrorHandler for ErrorSpy {
        fn scanning_error(&mut self, _line: u32, message: &str) {
            panic!("scanning_error: {}", message);
        }

        fn parsing_error(&mut self, _line: u32, _location: &str, message: &str) {
            panic!("parsing_error: {}", message);
        }
    }

    #[test]
    fn should_parse_expression() {
        let mut error_handler = ErrorSpy::new();
        let mut scanner = Scanner::new("1 + 2;", &mut error_handler);

        let tokens = scanner.scan_tokens();
        let mut parser = Parser::new(tokens, &mut error_handler);

        match parser.parse() {
            Ok(statements) => {
                let stmt = &statements[0];

                match stmt {
                    Stmt::ExprStmt { expr } => {
                        let left = Expr::new_literal(Literal::Number(1.0));
                        let operator = Token::new(TokenKind::Plus, "+", None, 1);
                        let right = Expr::new_literal(Literal::Number(2.0));

                        assert_eq!(**expr, Expr::new_binary(left, operator, right));
                    }

                    _ => panic!("Got non-expression statement"),
                }
            }

            Err(error) => eprintln!("Parsing failed: {}", error),
        }
    }
}
