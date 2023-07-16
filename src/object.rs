use crate::ast::Expr;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Object {
    Integer(i32),
    Float(f32),
    String(String),
    Void,
    Boolean(bool),
    WalkedList(Vec<Object>),
    List(Vec<Expr>),
}

impl std::ops::Add for Object {
    type Output = Object;
    fn add(self, rhs: Self) -> Self::Output {
        match self {
            Object::Float(l) => {
                match rhs {
                    Object::Float(r) => Object::Float(l+r),
                    _ => panic!("invalid type")
                }
            }
            Object::Integer(l) => {
                match rhs {
                    Object::Integer(r) => Object::Integer(l+r),
                    _ => panic!("invalid type")
                }
            }
            Object::String(l) => {
                match rhs {
                    Object::String(r) => Object::String({
                        let mut modified = l.clone();
                        modified.push_str(&r);
                        modified
                    }),
                    _ => panic!("invalid type")
                }
            }
            _ => panic!("invalid type")
        }
    }
}

impl std::ops::Sub for Object {
    type Output = Object;
    fn sub(self, rhs: Self) -> Self::Output {
        match self {
            Object::Float(l) => {
                match rhs {
                    Object::Float(r) => Object::Float(l-r),
                    _ => panic!("invalid type")
                }
            }
            Object::Integer(l) => {
                match rhs {
                    Object::Integer(r) => Object::Integer(l-r),
                    _ => panic!("invalid type")
                }
            }
            _ => panic!("invalid type")
        }
    }
}

impl std::ops::Mul for Object {
    type Output = Object;
    fn mul(self, rhs: Self) -> Self::Output {
        match self {
            Object::Float(l) => {
                match rhs {
                    Object::Float(r) => Object::Float(l*r),
                    _ => panic!("invalid type")
                }
            }
            Object::Integer(l) => {
                match rhs {
                    Object::Integer(r) => Object::Integer(l*r),
                    _ => panic!("invalid type")
                }
            }
            _ => panic!("invalid type")
        }
    }
}

impl std::ops::Div for Object {
    type Output = Object;
    fn div(self, rhs: Self) -> Self::Output {
        match self {
            Object::Float(l) => {
                match rhs {
                    Object::Float(r) => Object::Float(l/r),
                    _ => panic!("invalid type")
                }
            }
            Object::Integer(l) => {
                match rhs {
                    Object::Integer(r) => Object::Integer(l/r),
                    _ => panic!("invalid type")
                }
            }
            _ => panic!("invalid type")
        }
    }
}

impl std::ops::Rem for Object {
    type Output = Object;
    fn rem(self, rhs: Self) -> Self::Output {
        match self {
            Object::Float(l) => {
                match rhs {
                    Object::Float(r) => Object::Float(l%r),
                    _ => panic!("invalid type")
                }
            }
            Object::Integer(l) => {
                match rhs {
                    Object::Integer(r) => Object::Integer(l%r),
                    _ => panic!("invalid type")
                }
            }
            _ => panic!("invalid type")
        }
    }
}