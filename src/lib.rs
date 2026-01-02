pub mod interpreter {

    use std::{fs::File, io::Read};
    #[derive(Debug)]
    enum InstKind {
        IncreByte,
        DecreByte,
        PrintByte,
        FormwardPtr,
        BackwardPtr,
        PrintBuf,
    }

    #[derive(Debug)]
    struct InstToken {
        kind: InstKind,
    }

    impl InstToken {
        pub fn new(kind: InstKind) -> Self {
            Self { kind: kind }
        }
    }

    pub struct Interpreter {
        content: String,
        tokens: Vec<InstToken>,
        pointer: usize,
        buffer: [u8; 10],
    }

    impl Interpreter {
        pub fn new() -> Self {
            Self {
                content: "".to_string(),
                tokens: Vec::new(),
                pointer: 0,
                buffer: [0; 10],
            }
        }

        pub fn get_file_content(&mut self) {
            let mut file: File;
            match File::open("./dist/sample.bf") {
                Ok(f) => file = f,
                Err(err) => panic!("Can't open the file : {err}\n"),
            }

            let mut content: String = String::new();
            match file.read_to_string(&mut content) {
                Ok(msg) => println!("Message : {msg}"),
                Err(err) => panic!("Can't read the file : {err}\n"),
            }
            self.content = content;
        }

        pub fn produce_tokens(&mut self) {
            let chars: Vec<char> = self.content.as_bytes().iter().map(|&x| x as char).collect();
            for c in chars {
                match c {
                    '+' => self.tokens.push(InstToken::new(InstKind::IncreByte)),
                    '-' => self.tokens.push(InstToken::new(InstKind::DecreByte)),
                    '>' => self.tokens.push(InstToken::new(InstKind::FormwardPtr)),
                    '<' => self.tokens.push(InstToken::new(InstKind::BackwardPtr)),
                    '#' => self.tokens.push(InstToken::new(InstKind::PrintBuf)),
                    ',' => self.tokens.push(InstToken::new(InstKind::PrintByte)),
                    _ => println!("{:?}", c),
                }
            }
            for token in &self.tokens {
                println!("{:?}", token);
            }
        }

        pub fn interpretate(&mut self) {
            for token in &self.tokens {
                match token.kind {
                    InstKind::IncreByte => self.buffer[self.pointer] += 1,
                    InstKind::DecreByte => self.buffer[self.pointer] -= 1,
                    InstKind::FormwardPtr => self.pointer += 1,
                    InstKind::BackwardPtr => self.pointer -= 1,
                    InstKind::PrintBuf => println!("{:?}", self.buffer),
                    InstKind::PrintByte => println!("{:?}", self.buffer[self.pointer] as char),
                }
            }
        }
    }
}
