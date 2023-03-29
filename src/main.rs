use std::{env, process::exit};

fn run(input: &str) {
    println!("hello {input}");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Please provide exactly one argument");
        exit(1);
    }
    
    run(&args[1]);
}
