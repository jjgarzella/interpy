
use super::genericsyntax::GenericSyntaxTree;
use super::expr::Expr;



fn parse_lambda(tree: GenericSyntaxTree) -> Expr {
    use self::GenericSyntaxTree::*;
    match tree {
        Symbol(ref s) if s.parse::<isize>().is_ok() => Expr::Num(s.parse().unwrap()),
        Symbol(ref s) => Expr::Id(s.clone()),
        List(syntax) 
            if syntax.len() == 1 => panic!("unknown length 1 operator"),
        List(syntax) 
            if syntax.len() == 2 => Expr::Apply(box parse_lambda(syntax[0]), 
                                                   box parse_lambda(syntax[1])),
        List(syntax)
            if syntax.len() == 3 => match syntax[0] {
                Symbol(ref p) if p == "+" => Expr::Plus(box parse_lambda(syntax[1]), 
                                                      box parse_lambda(syntax[2])),
                Symbol(ref t) if t == "*" => Expr::Times(box parse_lambda(syntax[1]), 
                                                       box parse_lambda(syntax[2])),
                Symbol(ref l) if l == "lambda" => match syntax[1] {
                    List(vec) if vec.len() == 1 => match vec[1] {
                        Symbol(ref s) => Expr::Func(s.clone(), box parse_lambda(syntax[2])),
                        _ => panic!("unsupported inside of second part of function"),
                    },
                    _ => panic!("unsupported second part of function"),
                },
                _ => panic!("unknown length 3 operator"),
            },
        _ => panic!("unknown operator length or structure"),
    }
}


//#[cfg(test)]
//mod tests {
//    
//    #[test]
//    fn test_basic() {
//        let tree = Parser::new("(blah)").parse();
//    }
//
//    #[test]
//    fn test_levels() {
//        let tree = Parser::new("(boom (chicka chicka))").parse();
//    }
//
//    #[test]
//    fn test_complex() {
//        let tree = Parser::new("(+ ((lambda (x) (* 3 x)) 5) (+ (+ (* (+ 2 3) 2) -1) 10))").parse();
//    }
//}
