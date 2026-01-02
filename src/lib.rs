pub mod interpreter {

    use std::{
        fs::File,
        io::{Read, stdin},
        process,
    };
    #[derive(Debug)]
    enum InstKind {
        IncreByte,
        DecreByte,
        PrintByte,
        FormwardPtr,
        BackwardPtr,
        PrintBuf,
        TakeInput,
        OpenLoop,
        CloseLoop,
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
        file_path: String,
        in_loop: bool,
        open_index: usize,
        content: String,
        tokens: Vec<InstToken>,
        pointer: usize,
        buffer: [u8; 20],
    }

    impl Interpreter {
        pub fn new(fp: &String) -> Self {
            Self {
                file_path: fp.clone(),
                in_loop: false,
                open_index: 0,
                content: "".to_string(),
                tokens: Vec::new(),
                pointer: 0,
                buffer: [0; 20],
            }
        }

        pub fn get_file_content(&mut self) {
            let mut file: File;
            match File::open(self.file_path.clone()) {
                Ok(f) => file = f,
                Err(err) => {
                    println!("Can't open the file : {err}\n");
                    process::exit(0);
                }
            }

            let mut content: String = String::new();
            match file.read_to_string(&mut content) {
                Ok(_msg) => {}
                Err(err) => {
                    println!("Can't read the file : {err}\n");
                    process::exit(0);
                }
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
                    '.' => self.tokens.push(InstToken::new(InstKind::PrintByte)),
                    ',' => self.tokens.push(InstToken::new(InstKind::TakeInput)),
                    '[' => self.tokens.push(InstToken::new(InstKind::OpenLoop)),
                    ']' => self.tokens.push(InstToken::new(InstKind::CloseLoop)),
                    _ => {}
                }
            }
        }

        pub fn interpretate(&mut self) {
            let mut i: usize = 0;

            while i < self.tokens.len() {
                let token = &self.tokens[i];
                match token.kind {
                    InstKind::IncreByte => self.buffer[self.pointer] += 1,
                    InstKind::DecreByte => self.buffer[self.pointer] -= 1,
                    InstKind::FormwardPtr => self.pointer += 1,
                    InstKind::BackwardPtr => self.pointer -= 1,
                    InstKind::PrintBuf => println!("{:?}", self.buffer),
                    InstKind::PrintByte => print!("{}", self.buffer[self.pointer] as char),
                    InstKind::TakeInput => {
                        let mut buf: [u8; 1] = [0];
                        let mut stdin = stdin();
                        if stdin.read_exact(&mut buf).is_ok() {
                            self.buffer[self.pointer] = buf[0];
                        }
                    }
                    InstKind::OpenLoop => {
                        self.in_loop = true;
                        self.open_index = i;
                    }
                    InstKind::CloseLoop => {
                        if self.buffer[self.pointer] == 0 {
                            self.in_loop = false
                        } else {
                            i = self.open_index;
                            continue;
                        }
                    }
                }
                i += 1;
            }
        }
    }
}
