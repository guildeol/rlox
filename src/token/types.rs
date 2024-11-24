use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub enum TokenKind
{
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals.
    Identifier,
    String,
    Number,

    // Keywords.
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    EndOfFile
}

#[derive(Debug, PartialEq)]
pub enum Literal
{
    String(String),
    Number(f64),
    Boolean(bool),
    Nil
}

impl Display for Literal
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        match self
        {
            Literal::String(s) =>
            {
                return write!(f, "\"{}\"", s);
            }

            Literal::Number(n) =>
            {
                return write!(f, "{}", n);
            }

            Literal::Boolean(b) =>
            {
                return write!(f, "{}", b);
            }

            Literal::Nil =>
            {
                return write!(f, "Nil");
            }
        }
    }
}