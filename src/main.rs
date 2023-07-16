use clap::Parser;
use simplelisp::{ast::parse, interpreter::Interpreter};

#[derive(Parser)]
struct Args {
    #[arg()]
    input_file: String
}

fn main() {
    let content = std::fs::read_to_string(Args::parse().input_file).unwrap();
    let ast = parse(&content);
    Interpreter::interpret(ast);
}
