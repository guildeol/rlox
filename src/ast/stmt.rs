use crate::ast::Expr;
use crate::token::Token;

#[derive(Clone, Debug, PartialEq)]
pub enum Stmt {
    ExprStmt { expr: Box<Expr> },
    PrintStmt { expr: Box<Expr> },
    VarStmt { name: Token, initializer: Option<Expr> },
    BlockStmt { declarations: Vec<Stmt> },
}

pub trait StmtVisitor<R> {
    fn visit_expr_stmt(&mut self, expr: &Expr) -> R;
    fn visit_print_stmt(&mut self, expr: &Expr) -> R;
    fn visit_var_stmt(&mut self, name: &Token, initializer: &Option<Expr>) -> R;
    fn visit_block_stmt(&mut self, declarations: &Vec<Stmt>) -> R;
}

impl Stmt {
    pub fn new_expr_stmt(expr: Expr) -> Self {
        return Stmt::ExprStmt { expr: Box::new(expr) };
    }

    pub fn new_print_stmt(expr: Expr) -> Self {
        return Stmt::PrintStmt { expr: Box::new(expr) };
    }

    pub fn new_var_stmt(name: Token, initializer: Option<Expr>) -> Self {
        return Stmt::VarStmt {
            name: name,
            initializer: initializer,
        };
    }

    pub fn new_block_stmt(declarations: Vec<Stmt>) -> Self {
        return Stmt::BlockStmt {
            declarations: declarations,
        };
    }

    pub fn accept<R>(&self, visitor: &mut dyn StmtVisitor<R>) -> R {
        match self {
            Stmt::ExprStmt { expr } => visitor.visit_expr_stmt(expr),
            Stmt::PrintStmt { expr } => visitor.visit_print_stmt(expr),
            Stmt::VarStmt { name, initializer } => visitor.visit_var_stmt(name, initializer),
            Stmt::BlockStmt { declarations } => visitor.visit_block_stmt(declarations),
        }
    }
}
