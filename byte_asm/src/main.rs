#![allow(dead_code, unused_imports, unused_variables)]

use byte_asm::lexer::Lexer;
use std::env::args;

fn main() {
    if let Some(file_path) = args().nth(1) {
        println!("INFO: lexing: {file_path}");

        let mut tokens = Vec::new();
        let mut lexer =
            Lexer::new(std::fs::read_to_string(file_path).expect("failed to read the file"));

        while let Some(token) = lexer.scan_token() {
            tokens.push(token);
        }

        println!("{:#?}", tokens);
    }
}
