use std::{
    io::{stdin, stdout, Write},
    str::Chars,
};

fn main() {
    loop {
        print!("calc>");
        stdout().flush().expect("stdout flush failed");
        let mut text = String::new();
        stdin().read_line(&mut text).expect("read stdin failed");
        println!("{}", text)
    }
}

const ADD: &str = "+";
const SUB: &str = "-";
const MUL: &str = "*";
const DIV: &str = "/";
const INTEGER: &str = "INTEGER";
const LEFT_BRACKET: &str = "(";
const RIGHT_BRACKET: &str = ")";

#[derive(Debug)]
struct Lexer {
    text: String,
    current_char: char,
    iter: Chars,
    pos: i32,
}

impl Lexer {
    pub fn new(text: &str) -> Lexer {
        Lexer {
            text: text.to_string(),
            current_char: text.chars().next().expect("invalid input"),
            pos: 0,
        }
    }

    pub fn advance(&mut self) {
        self.pos += 1;
        self.current_char
    }
}
