use std::collections::HashMap;

use crate::{ast::{AST, Expr, BinOp}, object::Object, stdlib};

impl AST {
    pub fn unwrap_function(&self) -> (String, Vec<String>, Vec<AST>) {
        match self {
            AST::FuncDef(name, args, body) => (name.to_string(), args.to_vec(), body.to_vec()),
            _ => panic!("unwrapping function object failed")
        }
    }
}

struct Scope {
    vars: HashMap<String, Object>
}

impl Scope {
    pub fn new() -> Scope {
        Scope { vars: HashMap::new() }
    }

    pub fn inheret(&mut self, other: &Scope) {
        self.vars.extend(other.vars.clone())
    }

    pub fn get_var(&self, name: String) -> Object {
        self.vars[&name].clone()
    }

    pub fn set_var(&mut self, name: String, val: Object) {
        self.vars.insert(name, val);
    }
}

pub struct Interpreter {
    funcs: HashMap<String, FuncBody>
}

#[derive(Clone)]
enum FuncBody {
    AstBody(AST),
    ClosureBody(Box<&'static dyn Fn(Vec<Object>) -> Object>)
}

impl Interpreter {
    pub fn interpret(ast: Vec<AST>) -> Object {
        let mut int = Interpreter {
            funcs: HashMap::new()
        };

        let functions = vec![
            ("println".to_string(), FuncBody::ClosureBody(Box::new(&|args: Vec<Object>| {
                stdlib::println(args)
            }))),
            ("print".to_string(), FuncBody::ClosureBody(Box::new(&|args: Vec<Object>| {
                stdlib::print(args)
            }))),
        ];

        for func in ast {
            int.funcs.insert(func.unwrap_function().0, FuncBody::AstBody(func));
        }
        int.funcs.extend(functions);
        int.walk_function(int.funcs["main"].clone(), vec![])
    }

    fn walk_function(&mut self, func: FuncBody, args: Vec<Object>) -> Object {
        match func {
            FuncBody::AstBody(fun) => {
                let mut env = Scope::new();
                let f = fun.unwrap_function();
                for (pos, var) in f.1.into_iter().enumerate() {
                    env.set_var(var, args[pos].clone());
                }
                self.walk_body(f.2, &mut env).unwrap_or(Object::Void)        
            }
            FuncBody::ClosureBody(function) => {
                function(args)
            }
        }
    }

    fn walk_body(&mut self, body: Vec<AST>, env: &mut Scope) -> Option<Object> {
        for node in body {
            match node {
                AST::Expr(e) => { self.walk_expr(e, &env); },
                AST::Return(e) => return Some(self.walk_expr(e, &env)),
                AST::VarAssign(var, val) | AST::VarDecl(var, val) => {
                    let comped_expr = self.walk_expr(val, &env);
                    env.set_var(var, comped_expr);
                }
                AST::If(expr, body) => {
                    let val = self.walk_expr(expr, &env);
                    if let Object::Boolean(true) = val {
                        let mut new_env = Scope::new();
                        new_env.inheret(env);
                        if let Some(v) = self.walk_body(body, &mut new_env) {
                            return Some(v);
                        }
                    }
                }
                AST::ArrayAssign(arr, pos, val) => {
                    let walked_pos = self.walk_expr(pos, env);
                    let walked_val = self.walk_expr(val, env);
                    match env.get_var(arr.clone()) {
                        Object::WalkedList(mut l) => {
                            match walked_pos {
                                Object::Integer(n) => {
                                    l[n as usize] = walked_val;
                                    env.set_var(arr, Object::WalkedList(l));
                                }
                                _ => panic!("invalid type")
                            }
                        }
                        _ => panic!("invalid_type")
                    }
                }
                AST::ReturnVoid => {
                    return Some(Object::Void);
                }
                _ => unreachable!()
            };
        }
        None
    }

    fn walk_expr(&mut self, expr: Expr, env: &Scope) -> Object {
        match expr {
            Expr::Literal(l) => match l {
                Object::List(exprs) => {
                    let mut ret = vec![];
                    for expr in exprs {
                        ret.push(self.walk_expr(expr, env));
                    }
                    Object::WalkedList(ret)
                }
                _ => l,
            },
            Expr::BiOp(lhs, op, rhs) => {
                let lhs_val = self.walk_expr(*lhs, env);
                let rhs_val = self.walk_expr(*rhs, env);
                match op {
                    BinOp::Add => lhs_val+rhs_val,
                    BinOp::Sub => lhs_val-rhs_val,
                    BinOp::Mul => lhs_val*rhs_val,
                    BinOp::Div => lhs_val/rhs_val,
                    BinOp::Eq => Object::Boolean(lhs_val == rhs_val),
                    BinOp::Gt => Object::Boolean(lhs_val > rhs_val),
                    BinOp::Lt => Object::Boolean(lhs_val < rhs_val),
                    BinOp::Neq => Object::Boolean(lhs_val != rhs_val),
                    BinOp::Mod => lhs_val%rhs_val
                }
            }
            Expr::Var(v) => env.get_var(v),
            Expr::FuncCall(name, args) => {
                let walked_args = args.into_iter().map(|arg| self.walk_expr(arg, env)).collect();
                self.walk_function(self.funcs[&name].clone(), walked_args)
            }
            Expr::ArrayAccess(arr, expr) => {
                let walked_arr = self.walk_expr(*arr, env);
                let walked_expr = self.walk_expr(*expr, env);
                match walked_arr {
                    Object::WalkedList(list) => {
                        match walked_expr {
                            Object::Integer(n) => list[n as usize].clone(),
                            _ => panic!("invalid type")
                        }
                    }
                    _ => panic!("invalid type for array access")
                }
            }
        }
    }
}