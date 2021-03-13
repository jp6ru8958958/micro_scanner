#![allow(dead_code)]
#![allow(unused_variables)]

use std::env;
use std::fs;

fn scanner(contents: String) {
    println!("scanner()");
    let mut in_char: char = getchar();
    if in_char == '$' {
        println!("SCANEOF");
    }
    while in_char != '$' {
        println!("{}", in_char);
        if isspace(in_char) {
            continue;
        } else if isalpha(in_char) {
            /*
             * ID ::= LETTER | ID LETTER
             *               | ID DIGIT
             *               | ID UNDERSCORE
             */
            buffer_char(in_char);
        }
    }
}

fn clear_buffer() {
    println!("clear_buffer()");
}

fn feof(in_char: char) {
    println!("feof()");
}

fn getchar() -> char {
    println!("getchar()");
    let c = ' ';
    return c;
}

fn buffer_char(c: char) {
    println!("buffer_char()");
}

fn isspace(c: char) -> bool{
    println!("isspace()");
    true
}

fn isalpha(c: char) -> bool{
    println!("isalpha()");
    true
}

fn isalnum(c: char) -> bool{
    println!("isalnum()");
    true
}

fn isdigit(c: char) -> bool{
    println!("iddigit()");
    true
}

fn main() {
    let args: Vec<String> = env::args().collect();
    // let filename = &args[1];
    let filename = "micro_prog";
    let contents = fs::read_to_string(filename).expect("Read file error.");
    scanner(contents);
}

