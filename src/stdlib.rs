use crate::object::Object;

pub fn print_fmt(objs: Vec<Object>, debug: bool) {
    for arg in objs {
        match arg {
            Object::Integer(n) => print!("{n}"),
            Object::String(n) => if debug { print!("\"{n}\"") } else { print!("{}", n) },
            Object::Float(n) => print!("{n}"),
            Object::WalkedList(n) => {
                print!("[");
                for (pos, obj) in n.iter().enumerate() {
                    if pos == n.len()-1 {
                        print_fmt(vec![obj.clone()], true);
                    } else {
                        print_fmt(vec![obj.clone()], true);
                        print!(", ");
                    }
                }
                print!("]");
            },
            _ => print!("{:?}", arg),
        }
    }
}

pub fn print(objs: Vec<Object>) -> Object {
    print_fmt(objs, false);

    Object::Void
}

pub fn println(objs: Vec<Object>) -> Object {
    print(objs);
    println!();
    Object::Void
}