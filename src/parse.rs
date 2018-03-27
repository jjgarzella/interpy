
use super::genericsyntax::GenericSyntaxTree;
use super::genericsyntax::GenericSyntaxTree::*;

#[cfg(feature = "lazy")]
use super::expr_lazy::Expr;

#[cfg(not(feature = "lazy"))]
use super::expr::Expr;



pub fn parse(tree: &GenericSyntaxTree) -> Expr {
    match *tree {
        Symbol(ref s) if s.parse::<isize>().is_ok() => Expr::Num(s.parse().unwrap()),
        Symbol(ref s) => Expr::Id(s.clone()),
        List(ref syntax) 
            if syntax.len() == 1 => panic!("unknown length 1 operator"),
        List(ref syntax) 
            if syntax.len() == 2 => Expr::Apply(box parse(&syntax[0]), 
                                                   box parse(&syntax[1])),
        List(ref syntax)
            if syntax.len() == 3 => match syntax[0] {
                Symbol(ref p) if p == "+" => Expr::Plus(box parse(&syntax[1]), 
                                                      box parse(&syntax[2])),
                Symbol(ref t) if t == "*" => Expr::Times(box parse(&syntax[1]), 
                                                       box parse(&syntax[2])),
                Symbol(ref l) if l == "lambda" => match syntax[1] {
                    List(ref arg) if arg.len() == 1 => match arg[0] {
                        Symbol(ref s) => Expr::Func(s.clone(), box parse(&syntax[2])),
                        _ => panic!("unsupported inside of second part of function"),
                    },
                    _ => panic!("unsupported second part of function"),
                },
                _ => panic!("unknown length 3 operator"),
            },
        _ => panic!("unknown operator length or structure"),
    }
}

pub fn parse_statements(tree: &GenericSyntaxTree) -> Vec<Expr> {
    match *tree {
        Symbol(_) => panic!("Symbol is not a list of statements"),
        List(ref statements) => {
            let mut exprs: Vec<Expr> = vec![];
            for statement in statements {
                exprs.push(parse(statement));
            }
            exprs
        }
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
