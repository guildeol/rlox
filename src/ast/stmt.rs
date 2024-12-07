use crate::ast::Expr;

#[derive(Clone, Debug, PartialEq)]
pub enum Stmt {
    ExprStmt { expr: Box<Expr> },
    PrintStmt { expr: Box<Expr> },
}

pub trait StmtVisitor<R> {
    fn visit_expr_stmt(&self, expr: &Expr) -> R;
    fn visit_print_stmt(&self, expr: &Expr) -> R;
}

impl Stmt {
    pub fn new_expr_stmt(expr: Expr) -> Self {
        return Stmt::ExprStmt {
            expr: Box::new(expr),
        };
    }

    pub fn new_print_stmt(expr: Expr) -> Self {
        return Stmt::PrintStmt {
            expr: Box::new(expr),
        };
    }

    pub fn accept<R>(&self, visitor: &dyn StmtVisitor<R>) -> R {
        match self {
            Stmt::ExprStmt { expr } => visitor.visit_expr_stmt(expr),
            Stmt::PrintStmt { expr } => visitor.visit_print_stmt(expr),
        }
    }
}
