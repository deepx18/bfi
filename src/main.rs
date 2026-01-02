use bfi::interpreter::Interpreter;

fn main() {
    let mut inter = Interpreter::new();
    inter.get_file_content();
    inter.produce_tokens();
    inter.interpretate();
}
