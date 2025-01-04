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
            match self.declaration() {
                Ok(s) => statements.push(s),
                Err(_) => {
                    self.synchronize();
                    return Ok(vec![]);
                }
            }
        }

        return Ok(statements);
    }

    fn expression(&mut self) -> Result<Expr, RuntimeError> {
        return self.assignment();
    }

    fn declaration(&mut self) -> Result<Stmt, RuntimeError> {
        if self.consume_if_one_of(vec![TokenKind::Var]) {
            return self.var_declaration();
        }

        return self.statement();
    }

    fn statement(&mut self) -> Result<Stmt, RuntimeError> {
        if self.consume(TokenKind::For)
        {
            return self.for_statement();
        }
        else if self.consume(TokenKind::If) {
            return self.if_statement();
        } else if self.consume(TokenKind::Print) {
            return self.print_statement();
        } else if self.consume(TokenKind::While) {
            return self.while_statement();
        } else if self.consume(TokenKind::LeftBrace) {
            let declarations = self.block()?;

            return Ok(Stmt::new_block_stmt(declarations));
        }

        return self.expression_statement();
    }

    fn for_statement(&mut self) -> Result<Stmt, RuntimeError> {
        self.consume_or(TokenKind::LeftParen, "Expect '(' after 'for'.")?;

        let initializer: Option<Stmt>;
        if self.consume(TokenKind::Semicolon) {
            initializer = None;
        } else if self.consume(TokenKind::Var) {
            initializer = Some(self.var_declaration()?);
        }
        else {
            initializer = Some(self.expression_statement()?);
        }

        let mut condition: Option<Expr> = None;
        if !self.check(TokenKind::Semicolon) {
            condition = Some(self.expression()?);
        }

        self.consume_or(TokenKind::Semicolon, "Expect ';' after loop condition.")?;

        let mut increment: Option<Expr> = None;
        if !self.check(TokenKind::RightParen) {
            increment = Some(self.expression()?);
        }

        self.consume_or(TokenKind::RightParen, "Expect ')' after for clause.")?;

        let mut body = self.statement()?;

        if let Some(increment) = increment {
            body = Stmt::new_block_stmt(vec![body, Stmt::new_expr_stmt(increment)]);
        }

        let condition = condition.unwrap_or_else(|| Expr::new_literal(Literal::Boolean(true)));
        body = Stmt::new_while_stmt(condition, body);

        if let Some(initializer) = initializer {
            body = Stmt::new_block_stmt(vec![initializer, body]);
        }

        return Ok(body);
    }

    fn if_statement(&mut self) -> Result<Stmt, RuntimeError> {
        self.consume_or(TokenKind::LeftParen, "Expect '(' after 'if'.")?;

        let condition = self.expression()?;

        self.consume_or(TokenKind::RightParen, "Expect '(' after 'if condition.")?;

        let then_branch = self.statement()?;
        let mut else_branch: Option<Stmt> = None;
        if self.consume(TokenKind::Else) {
            else_branch = Some(self.statement()?);
        }

        return Ok(Stmt::new_if_stmt(condition, then_branch, else_branch));
    }

    fn print_statement(&mut self) -> Result<Stmt, RuntimeError> {
        let expr = self.expression()?;

        self.consume_or(TokenKind::Semicolon, "Expect ';' after value.")?;

        return Ok(Stmt::new_print_stmt(expr));
    }

    fn var_declaration(&mut self) -> Result<Stmt, RuntimeError> {
        let name = self.consume_or(TokenKind::Identifier, "Expect variable name")?;

        let mut initializer: Option<Expr> = None;
        if self.consume(TokenKind::Equal) {
            initializer = Some(self.expression()?);
        }

        self.consume_or(TokenKind::Semicolon, "Expect ';' after variable declaration.")?;

        return Ok(Stmt::new_var_stmt(name, initializer));
    }

    fn while_statement(&mut self) -> Result<Stmt, RuntimeError> {
        self.consume_or(TokenKind::LeftParen, "Expect '(' after 'while'.")?;

        let condition = self.expression()?;

        self.consume_or(TokenKind::RightParen, "Expect ')' after condition.")?;

        let body = self.statement()?;

        return Ok(Stmt::new_while_stmt(condition, body));
    }

    fn block(&mut self) -> Result<Vec<Stmt>, RuntimeError> {
        let mut statements: Vec<Stmt> = Vec::new();

        while !self.check(TokenKind::RightBrace) && !self.is_at_end() {
            statements.push(self.declaration()?);
        }

        self.consume_or(TokenKind::RightBrace, "Expect '}' after block.")?;

        return Ok(statements);
    }

    fn expression_statement(&mut self) -> Result<Stmt, RuntimeError> {
        let expr = self.expression()?;

        self.consume_or(TokenKind::Semicolon, "Expect ';' after expression.")?;

        return Ok(Stmt::new_expr_stmt(expr));
    }

    fn assignment(&mut self) -> Result<Expr, RuntimeError> {
        let expr = self.or()?;

        if self.consume(TokenKind::Equal) {
            let equals = self.previous();
            let value = self.assignment()?;

            match expr {
                Expr::Variable { name } => {
                    return Ok(Expr::new_assignment(name, value));
                }
                _ => {
                    return Err(self.error(&equals, "Invalid assignment target."));
                }
            }
        }

        return Ok(expr);
    }

    fn or(&mut self) -> Result<Expr, RuntimeError> {
        let mut expr = self.and()?;

        while self.consume(TokenKind::Or) {
            let operator = self.previous();
            let right = self.and()?;

            expr = Expr::new_logical(expr, operator, right);
        }

        return Ok(expr);
    }

    fn and(&mut self) -> Result<Expr, RuntimeError> {
        let mut expr = self.equality()?;

        while self.consume(TokenKind::And) {
            let operator = self.previous();
            let right = self.equality()?;

            expr = Expr::new_logical(expr, operator, right);
        }

        return Ok(expr);
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

    fn consume(&mut self, candidate: TokenKind) -> bool {
        if self.check(candidate) {
            self.advance();
            return true;
        }

        return false;
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
        if self.consume(TokenKind::False) {
            return Ok(Expr::new_literal(Literal::Boolean(false)));
        }

        if self.consume(TokenKind::True) {
            return Ok(Expr::new_literal(Literal::Boolean(true)));
        }

        if self.consume(TokenKind::Nil) {
            return Ok(Expr::new_literal(Literal::Nil));
        }

        if self.consume_if_one_of(vec![TokenKind::Number, TokenKind::String]) {
            let prev = self.previous();
            return Ok(Expr::new_literal(prev.literal.unwrap()));
        }

        if self.consume(TokenKind::LeftParen) {
            let expr = self.expression().ok();
            self.consume_or(TokenKind::RightParen, "Expect ')' after expression.")?;
            return Ok(Expr::new_grouping(expr.unwrap()));
        }

        if self.consume(TokenKind::Identifier) {
            return Ok(Expr::new_variable(self.previous()));
        }

        return Err(RuntimeError::parse_error(&format!(
            "Unexpected primary expression token '{}'",
            self.peek()
        )));
    }

    fn consume_or(&mut self, kind: TokenKind, message: &str) -> Result<Token, RuntimeError> {
        if self.check(kind) {
            return Ok(self.advance());
        } else {
            return Err(self.error(&self.peek(), message));
        }
    }

    fn error(&mut self, token: &Token, message: &str) -> RuntimeError {
        if token.kind == TokenKind::EndOfFile {
            self.error_handler.parsing_error(token.line, " at end", message);
        } else {
            self.error_handler
                .parsing_error(token.line, &format!(" at '{}'", token.lexeme), message);
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
