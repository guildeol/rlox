use crate::ast::Expr;
use crate::token::Token;

#[derive(PartialEq)]
pub enum Stmt {
    ExprStmt {
        expr: Box<Expr>,
    },
    IfStmt {
        condition: Box<Expr>,
        then_branch: Box<Stmt>,
        else_branch: Box<Option<Stmt>>,
    },
    PrintStmt {
        expr: Box<Expr>,
    },
    VarStmt {
        name: Token,
        initializer: Option<Expr>,
    },
    BlockStmt {
        declarations: Vec<Stmt>,
    },
    WhileStmt {
        condition: Box<Expr>,
        body: Box<Stmt>,
    },
}

pub trait StmtVisitor<R> {
    fn visit_expr_stmt(&mut self, expr: &Expr) -> R;
    fn visit_if_stmt(&mut self, condition: &Expr, then_branch: &Stmt, else_branch: &Option<Stmt>) -> R;
    fn visit_print_stmt(&mut self, expr: &Expr) -> R;
    fn visit_var_stmt(&mut self, name: &Token, initializer: &Option<Expr>) -> R;
    fn visit_block_stmt(&mut self, declarations: &Vec<Stmt>) -> R;
    fn visit_while_stmt(&mut self, condition: &Expr, body: &Stmt) -> R;
}

impl Stmt {
    pub fn new_expr_stmt(expr: Expr) -> Self {
        return Stmt::ExprStmt { expr: Box::new(expr) };
    }

    pub fn new_if_stmt(condition: Expr, then_branch: Stmt, else_branch: Option<Stmt>) -> Self {
        return Stmt::IfStmt {
            condition: Box::new(condition),
            then_branch: Box::new(then_branch),
            else_branch: Box::new(else_branch),
        };
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

    pub fn new_while_stmt(condition: Expr, body: Stmt) -> Self {
        return Stmt::WhileStmt {
            condition: Box::new(condition),
            body: Box::new(body),
        };
    }

    pub fn accept<R>(&self, visitor: &mut dyn StmtVisitor<R>) -> R {
        match self {
            Stmt::ExprStmt { expr } => visitor.visit_expr_stmt(expr),
            Stmt::IfStmt {
                condition,
                then_branch,
                else_branch,
            } => visitor.visit_if_stmt(condition, then_branch, else_branch),
            Stmt::PrintStmt { expr } => visitor.visit_print_stmt(expr),
            Stmt::VarStmt { name, initializer } => visitor.visit_var_stmt(name, initializer),
            Stmt::BlockStmt { declarations } => visitor.visit_block_stmt(declarations),
            Stmt::WhileStmt { condition, body } => visitor.visit_while_stmt(condition, body),
        }
    }
}
