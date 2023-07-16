use std::process::exit;

use lalrpop_util::lalrpop_mod;

use crate::object::Object;

lalrpop_mod!(grammar);

#[derive(Debug, Clone)]
pub enum AST {
    FuncDef(String, Vec<String>, Vec<AST>),
    Expr(Expr),
    VarDecl(String, Expr),
    VarAssign(String, Expr),
    ArrayAssign(String, Expr, Expr),
    Return(Expr),
    ReturnVoid,
    If(Expr, Vec<AST>),
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Expr {
    BiOp(Box<Expr>, BinOp, Box<Expr>),
    Var(String),
    Literal(Object),
    FuncCall(String, Vec<Expr>),
    ArrayAccess(Box<Expr>, Box<Expr>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
    Gt,
    Lt,
    Eq,
    Neq,
    Mod,
}

pub fn parse(code: &str) -> Vec<AST> {
    match grammar::SimpleLispParser::new().parse(code) {
        Ok(ast) => ast,
        Err(e) => {
            crate::printing::print(format!("{}", e));
            exit(-1);
        }
    }
}
