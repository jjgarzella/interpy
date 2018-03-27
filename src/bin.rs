
extern crate interpy;
use std::io;
use std::io::prelude::*;

fn main() {
    loop {
        print!("interpy>> ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        let stdin = io::stdin();
        stdin.read_line(&mut input).ok().expect("Could not read from standard input");
        println!("Input: {}", input);
        let val = interpy::interpret(input);
        println!("Answer: {}", val.to_string()); 
        io::stdout().flush().unwrap();
    }
}
