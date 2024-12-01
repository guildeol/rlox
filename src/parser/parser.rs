use crate::ast::expr::Expr;
use crate::token::types::TokenKind;
use crate::token::{self, Token};
use crate::token::types::Literal;

pub struct Parser
{
    pub tokens: Vec<Token>,
    pub current: usize,
}

impl Parser
{
    pub fn new(tokens: Vec<Token>) -> Self
    {
        return Parser {tokens: tokens, current: 0};
    }

    fn expression(&mut self) -> Expr
    {
        return self.equality();
    }

    fn equality(&mut self) -> Expr
    {
        let mut expr = self.comparison();

        while self.consume_if_one_of(vec![TokenKind::BangEqual, TokenKind::EqualEqual])
        {
            let operator = self.previous().clone();
            let right = self.comparison();

            expr = Expr::new_binary(expr, operator, right)
        }

        return expr;
    }

    fn consume_if_one_of(&mut self, candidates: Vec<TokenKind>) -> bool
    {
        for entry in candidates
        {
            if self.check(entry)
            {
                self.advance();
                return true;
            }
        }

        return false;
    }

    fn check(&mut self, candidate: TokenKind) -> bool
    {
        if self.is_at_end()
        {
            return false;
        }
        else
        {
            return self.peek().kind == candidate;
        }
    }

    fn advance(&mut self) -> Token
    {
        todo!();
    }

    fn is_at_end(&self) -> bool
    {
        return self.peek().kind == TokenKind::EndOfFile;
    }

    fn peek(&self) -> Token
    {
        return self.tokens[self.current].clone();
    }

    fn previous(&mut self) -> Token
    {
        return self.tokens[self.current - 1].clone();
    }

    fn comparison(&mut self) -> Expr
    {
        let mut expr = self.term();

        while self.consume_if_one_of(vec![TokenKind::Greater, TokenKind::GreaterEqual, TokenKind::Less, TokenKind::LessEqual])
        {
            let operator = self.previous();
            let right = self.term();

            expr = Expr::new_binary(expr, operator, right);
        }

        return expr;
    }

    fn term(&mut self) -> Expr
    {
        let mut expr = self.factor();

        while self.consume_if_one_of(vec![TokenKind::Minus, TokenKind::Plus])
        {
            let operator = self.previous();
            let right = self.factor();

            expr = Expr::new_binary(expr, operator, right);
        }

        return expr;
    }

    fn factor(&mut self) -> Expr
    {
        let mut expr = self.unary();

        while self.consume_if_one_of(vec![TokenKind::Slash, TokenKind::Star])
        {
            let operator = self.previous();
            let right = self.unary();

            expr = Expr::new_binary(expr, operator, right);
        }

        return expr;
    }

    fn unary(&mut self) -> Expr
    {
        while self.consume_if_one_of(vec![TokenKind::Bang, TokenKind::Minus])
        {
            let operator = self.previous();
            let right = self.unary();

            return Expr::new_unary(operator, right);
        }

        return self.primary();
    }

    fn primary(&mut self) -> Expr
    {
        let token = self.peek();
        match token.kind
        {
            TokenKind::False => Expr::new_literal(Literal::Boolean(false)),
            TokenKind::True => Expr::new_literal(Literal::Boolean(true)),
            TokenKind::Nil => Expr::new_literal(Literal::Nil),
            TokenKind::Number => Expr::new_literal(token.literal.expect("No literal number available")),
            TokenKind::String => Expr::new_literal(token.literal.expect("No literal string available")),
            TokenKind::LeftParen =>
            {
                let expr = self.expression();
                self.consume(TokenKind::RightParen, "Expect ')' after expression.");

                return Expr::new_grouping(expr);
            }
            _ =>
                todo!()
        }
    }

    fn consume(&mut self, kind: TokenKind, message: &str)
    {
        todo!();
    }

}
