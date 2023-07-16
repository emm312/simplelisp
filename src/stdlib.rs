use crate::{object::Object, printing};

pub fn print_fmt(objs: Vec<Object>, debug: bool) {
    for arg in objs {
        match arg {
            Object::Integer(n) => printing::print(format!("{n}")),
            Object::String(n) => {
                if debug {
                    printing::print(format!("\"{n}\""))
                } else {
                    printing::print(format!("{n}"))
                }
            }
            Object::Float(n) => printing::print(format!("{n}")),
            Object::WalkedList(n) => {
                printing::print("[".to_string());
                for (pos, obj) in n.iter().enumerate() {
                    if pos == n.len() - 1 {
                        print_fmt(vec![obj.clone()], true);
                    } else {
                        print_fmt(vec![obj.clone()], true);
                        printing::print(", ".to_string());
                    }
                }
                printing::print("]".to_string());
            }
            _ => printing::print(format!("{:?}", arg)),
        }
    }
}

pub fn print(objs: Vec<Object>) -> Object {
    print_fmt(objs, false);

    Object::Void
}

pub fn println(objs: Vec<Object>) -> Object {
    print(objs);
    printing::println(String::default());
    Object::Void
}
