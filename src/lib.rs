pub mod ast;
pub mod interpreter;
pub mod object;
pub mod stdlib;

#[cfg(feature = "wasm")]
pub mod wasm {
    use crate::ast::parse;
    use crate::interpreter::Interpreter;
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    pub fn interpret(code: &str) {
        Interpreter::interpret(parse(code));
    }
    #[wasm_bindgen]
    pub fn init_rust() {
        console_error_panic_hook::set_once();
    }
}

#[cfg(not(feature = "wasm"))]
pub mod printing {
    pub fn print(val: String) {
        print!("{val}");
    }
    pub fn println(val: String) {
        println!("{val}");
    }
}

#[cfg(feature = "wasm")]
pub mod printing {
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace=console)]
        pub fn log(s: &str);
    }
    pub fn print(val: String) {
        log(&val);
    }
    pub fn println(val: String) {
        log(&val);
    }
}
