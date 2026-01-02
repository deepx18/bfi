use std::env;

use bfi::interpreter::Interpreter;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut inter = Interpreter::new(&args[1]);
    inter.get_file_content();
    inter.produce_tokens();
    inter.interpretate();
}
