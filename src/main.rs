#![allow(dead_code)]
#![allow(unused_variables)]

use std::env;
use std::fs;

struct Text {
    content: String,
    pos: u32,
    pre: u32,
    buffer: String,
}

impl Text {
    fn clear_buffer(self) {
        println!("clear_buffer()");
    }

    fn getchar(self) -> char {
        println!("getchar()");
        return ' ';
    }

    fn ungetc(self) {
        println!("ungetc()");
    }
}

fn contents_parse(contents: String) -> Text {
    Text {
        content: contents,
        pos: 0,
        pre: 0,
        buffer: "".to_string(),
    }
}

fn scanner(contents: String) {
    let text = contents_parse(contents);
    println!("scanner()");
    let in_char: char = text.getchar();
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
                let c = text.getchar();
                if isalnum(c)||c=='_' { buffer_char(c); } else { break; }
            }
            text.ungetc();
            check_reserved();
        } else if isdigit(in_char) {
            /*
             * INILITERAL ::= DIGIT | INTLITERAL DIGIT
             */
            buffer_char(in_char);
            loop {
                let c = text.getchar();
                if isdigit(c) { buffer_char(c); } else { break; }
            } 
            println!("INTLITERAL");
        } else if in_char == ':' {
            let c = text.getchar();
            if c == '=' {
                println!("ASSIGNOP");
            } else {
                text.ungetc();
                lexical_error();
            }
        } else if in_char == '-' {
            let c = text.getchar();
            if c == '-' {
                loop {
                    let in_char = text.getchar();
                    if in_char == '\n' { break; }
                }
            } else {
                text.ungetc();
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
fn feof(in_char: char) {
    println!("feof()");
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

