
#![allow(dead_code)] // just about every bit of code is dead.

#![feature(box_syntax, box_patterns)]

mod tokenizer;
mod parens;
mod genericsyntax;
mod parse;
mod expr;

pub fn main() {
    // get input and interp!
    use std::io::{self, BufRead};
    loop {
        print!("interpy>> ");
        let mut input = String::new();
        let stdin = io::stdin();
        stdin.lock().read_line(&mut input).expect("Could not read from standard input");
        println!("{}", interp(input))
    }
}



pub fn interp(code: String) -> expr::Value {
  use tokenizer::Tokenizer;
  println!("Tokenizing...");
  let tokens = code.tokenize();
  println!("Generating Syntax...");
  let generic_syntax = genericsyntax::GenericSyntaxTree::from_tokens(&tokens);
  println!("Parsing...");
  let expr = parse::parse_lambda(&generic_syntax);
  println!("Interpreting...");
  let val = expr::interp(expr, &expr::Environment::new());
  val
}
