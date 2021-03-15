#![allow(dead_code)]
#![allow(unused_variables)]

use std::env;
use std::fs;

fn scanner(contents: String) {
    println!("scanner()");
    let in_char: char = getchar();
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
            loop {
                let c = getchar();
                if isalnum(c)||c=='_' { buffer_char(c); } else { break; }
            }
            ungetc();
            check_reserved();
        } else if isdigit(in_char) {
            /*
             * INILITERAL ::= DIGIT | INTLITERAL DIGIT
             */
            buffer_char(in_char);
            loop {
                let c = getchar();
                if isdigit(c) { buffer_char(c); } else { break; }
            } 
            println!("INTLITERAL");
        } else if in_char == ':' {
            let c = getchar();
            if c == '=' {
                println!("ASSIGNOP");
            } else {
                ungetc();
                lexical_error();
            }
        } else if in_char == '-' {
            let c = getchar();
            if c == '-' {
                loop {
                    if in_char == '\n' { break; }
                    in_char == getchar();
                }
            } else {
                ungetc();
                println!("MINUSOP");
            }
        } else if in_char == '(' { 
            println!("LPAREN"); 
        } else if in_char == ')' {
            println!("RPAREN");
        } else if in_char == ';' {
            println!("SEMICOLON");
        } else if in_char == ',' {
            println!("COMMA");
        } else if in_char == '+' {
            println!("PLUSOP");
        } else {
            lexical_error();
        }
    }
}

fn check_reserved() {
    println!("check_reserved()");
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

fn ungetc() {
    println!("ungetc()");
}

fn lexical_error() {
    println!("lexical_error()");
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
    println!("isdigit()");
    true
}

fn main() {
    let args: Vec<String> = env::args().collect();
    // let filename = &args[1];
    let filename = "micro_prog";
    let contents = fs::read_to_string(filename).expect("Read file error.");
    scanner(contents);
}

