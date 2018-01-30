
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
  println!("{:?}", tokens);
  println!("Generating Syntax...");
  let generic_syntax = genericsyntax::GenericSyntaxTree::from_tokens(&tokens);
  println!("{}", generic_syntax);
  println!("Parsing...");
  let exprs = parse::parse_statements(&generic_syntax);
  println!("Interpreting...");
  let mut vals: Vec<expr::Value> = vec![];
  for expr in exprs {
    println!("{}", expr);
    vals.push(expr::interp(expr, &expr::Environment::new()));
  }
  vals.pop().unwrap()
}

#[cfg(test)]
mod tests {
    use super::interp;
    
    fn test_string_interp(code: &str, result: i64) {

        assert_eq!(interp(code.to_string()).num(), Ok(result));
    }

    #[test]
    fn test_numlit() {
        test_string_interp("7", 7);
        test_string_interp("2", 2);
    }

    #[test]
    fn test_plus() {
        test_string_interp("(+ 7 5)", 12);
        test_string_interp("(+ 2 2)", 4);
    }

    #[test]
    fn test_times() {
        test_string_interp("(* 3 4)", 12);
        test_string_interp("(* 7 5)", 35);
    }

    #[test]
    fn test_negative() {
        test_string_interp("-5", -5);
        test_string_interp("(+ -5 3)", -2);
        test_string_interp("(* -5 3)", -15);
    }

    #[test]
    fn test_more_digits() {
        test_string_interp("45", 45);
        test_string_interp("(+ 3 12)", 15);
        test_string_interp("(+ 3 -12", -9);
        test_string_interp("(* 4 10)", 40);
        test_string_interp("(* 4 -10", -40);
    }

    #[test]
    fn test_function() {
        test_string_interp("((lambda (x) x) 5)", 5);
        test_string_interp("((lambda (y) (+ y 3)) (+ 3 4))", 10);
    }
    
    //#[test]
    fn test_complex_func() {
        test_string_interp("(lambda (x) 3) (lambda (y) y))", 3);
        test_string_interp("(((lambda (x) (lambda (y) (* x y))) 6) 7)", 42);
        test_string_interp("((lambda (x) (x 3)) (lambda (z) (+ z 2)))", 5);
    }
}
