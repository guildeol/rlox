use crate::ast::Expr;
use crate::token::Token;

#[derive(Debug, Clone, PartialEq)]
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
    FunctionStmt {
        name: Token,
        parameters: Vec<Token>,
        body: Vec<Stmt>,
    },
    ReturnStmt {
        keyword: Token,
        value: Box<Expr>,
    },
}

pub trait StmtVisitor<R> {
    fn visit_expr_stmt(&mut self, expr: &Expr) -> R;
    fn visit_if_stmt(&mut self, condition: &Expr, then_branch: &Stmt, else_branch: &Option<Stmt>) -> R;
    fn visit_print_stmt(&mut self, expr: &Expr) -> R;
    fn visit_var_stmt(&mut self, name: &Token, initializer: &Option<Expr>) -> R;
    fn visit_block_stmt(&mut self, declarations: &Vec<Stmt>) -> R;
    fn visit_while_stmt(&mut self, condition: &Expr, body: &Stmt) -> R;
    fn visit_function_stmt(&mut self, name: &Token, parameters: &Vec<Token>, body: &Vec<Stmt>) -> R;
    fn visit_return_stmt(&mut self, keyword: &Token, value: &Expr) -> R;
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

    pub fn new_function(name: Token, parameters: Vec<Token>, body: Vec<Stmt>) -> Self {
        return Stmt::FunctionStmt { name, parameters, body };
    }

    pub fn new_return_stmt(keyword: Token, value: Expr) -> Self {
        return Stmt::ReturnStmt {
            keyword: keyword,
            value: Box::new(value),
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
            Stmt::FunctionStmt { name, parameters, body } => visitor.visit_function_stmt(name, parameters, body),
            Stmt::ReturnStmt { keyword, value } => visitor.visit_return_stmt(keyword, value),
        }
    }
}

use std::fmt;

impl fmt::Display for Stmt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Stmt::ExprStmt { expr } => write!(f, "ExprStmt({})", expr),
            Stmt::IfStmt {
                condition,
                then_branch,
                else_branch,
            } => {
                writeln!(
                    f,
                    "IfStmt(condition: {}, then: {}, else: {})",
                    condition,
                    then_branch,
                    match **else_branch {
                        Some(ref else_branch) => format!("{}", else_branch),
                        None => "None".to_string(),
                    }
                )
            }
            Stmt::PrintStmt { expr } => writeln!(f, "PrintStmt({})", expr),
            Stmt::VarStmt { name, initializer } => {
                writeln!(
                    f,
                    "VarStmt(name: {}, initializer: {})",
                    name,
                    match initializer {
                        Some(expr) => format!("{}", expr),
                        None => "None".to_string(),
                    }
                )
            }
            Stmt::BlockStmt { declarations } => {
                let decls: Vec<String> = declarations.iter().map(|stmt| format!("{}", stmt)).collect();
                writeln!(f, "BlockStmt([{}])", decls.join(", "))
            }
            Stmt::WhileStmt { condition, body } => {
                writeln!(f, "WhileStmt(condition: {}, body: {})", condition, body)
            }
            Stmt::FunctionStmt { name, parameters, body } => {
                let params: Vec<String> = parameters.iter().map(|param| format!("{}", param)).collect();
                let body_stmts: Vec<String> = body.iter().map(|stmt| format!("{}", stmt)).collect();
                writeln!(
                    f,
                    "FunctionStmt(name: {}, parameters: [{}], body: [{}])",
                    name,
                    params.join(", "),
                    body_stmts.join(", ")
                )
            }
            Stmt::ReturnStmt { keyword, value } => {
                writeln!(f, "ReturnStmt(keyword: {}, value: {})", keyword, value)
            }
        }
    }
}
