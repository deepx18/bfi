use std::{env, process};

use bfi::interpreter::Interpreter;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Prohibited usage of the interpreter\nexecute <bfi -h> to print the usage");
        process::exit(0);
    }
    if args[1] == "-h" {
        println!(
            "Usage: bfi [OPTIONS...] <input_file>\noptions:\n\t-h: print help\n\t-d: print language docs"
        );
        process::exit(0);
    }

    if args[1] == "-d" {
        println!(
            ">	increment pointer\n<	decrement pointer\n+	increment value at pointer\n-	decrement value at pointer\n[	begin loop (continues while value at pointer is non-zero)\n]	end loop\n,	read one character from input into value at pointer\n.	print value at pointer to output as a character\n#	display debugging info (in debug mode)\nAny other characters are ignored."
        )
    }

    let mut inter = Interpreter::new(&args[1]);
    inter.get_file_content();
    inter.produce_tokens();
    inter.interpretate();
}
