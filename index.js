import init, { interpret, init_rust } from "./pkg/simplelisp.js";

init().then(() => {
    init_rust();
    interpret("(FN main () ($println(\"hello world\")))");
});