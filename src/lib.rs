
#![allow(dead_code)] // just about every bit of code is dead.

#![feature(box_syntax, box_patterns)]

mod tokenizer;
mod parens;
mod genericsyntax;
mod parse;
mod expr;

pub fn interp(code: String) -> expr::Value {
  use tokenizer::Tokenizer;
  println!("Tokenizing...");
  let tokens = code.tokenize();
  println!("Generating Syntax...");
  let generic_syntax = genericsyntax::GenericSyntaxTree::from_tokens(&tokens);
  println!("Parsing...");
  let exprs = parse::parse_statements(&generic_syntax);
  println!("Interpreting...");
  let mut vals: Vec<expr::Value> = vec![];
  for expr in exprs {
    vals.push(expr::interp(expr, &expr::Environment::new()));
  }
  vals.pop().unwrap()
}
