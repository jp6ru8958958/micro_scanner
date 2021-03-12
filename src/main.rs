use std::env;
use std::fs;

fn main() {
    #[allow(unused_variables)]
    let args: Vec<String> = env::args().collect();
    // let filename = &args[1];
    let filename = "micro_prog";
    let contents = fs::read_to_string(filename).expect("Read file error.");
    println!("{}", contents);

}

