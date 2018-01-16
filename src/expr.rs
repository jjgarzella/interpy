// <Expr> : 
//              num
//              plus
//              times
//              id
//              lambda
//              app

use std::collections::HashMap;
use std::cell::RefCell;

type Identifier = String;

#[derive(Clone,Debug)]
struct Environment(RefCell<HashMap<Identifier, Value>>);

impl Environment {
    pub fn new() -> Environment {
        Environment {
            0: RefCell::new(HashMap::new()),
        }
    }

    fn lookup(&self, id: Identifier) -> Value {
        match self.0.borrow().get(&id) {
            Some(val) => val.clone(),
            None => panic!("Free variable!"),
        }
    }

    fn extend(&self, id: Identifier, val: Value) {
        self.0.borrow_mut().insert(id,val);
    }
}

#[derive(Clone,Debug)]
enum Value {
    Num(i64),
    Func(Identifier,Expr,Environment),
}

impl Value {
    pub fn num(&self) -> Result<i64,&str> {
        use expr::Value::*;
        match self {
            &Num(n) => Ok(n),
            &Func(_,_,_) => Err("function"),
        }
    }
}

#[derive(Clone,Debug)]
pub enum Expr {
    Num(i64),
    Plus(Box<Expr>, Box<Expr>),
    Times(Box<Expr>, Box<Expr>),
    Id(Identifier),
    Func(Identifier, Box<Expr>),
    Apply(Box<Expr>, Box<Expr>),
}

fn binary_num_op(op: &Fn(i64, i64) -> i64, left: Value, right: Value) -> Value {
    use self::Value::Num;
    match (left, right) {
        (Num(l), Num(r)) => Num(op(l,r)),
        _ => panic!("Not a number"),
    }
}


fn interp(expr: Expr, env: &Environment) -> Value {
    use self::Expr::*;
    match expr {
        Num(n) => Value::Num(n),
        Plus(box lhs, box rhs) => binary_num_op(&|x,y| {x+y}, interp(lhs,env), interp(rhs,env)),
        Times(box lhs, box rhs) => binary_num_op(&|x,y| {x*y}, interp(lhs,env), interp(rhs,env)),
        Id(id) => env.lookup(id),
        Func(param,box body) => Value::Func(param,body,env.clone()),
        Apply(box func, box arg) => match interp(func,env) {
            Value::Func(id,body,env) => {
                env.extend(id,interp(arg,&env));
                interp(body,&env)
            }
            _ => panic!("Not a function"),
        },
    }
}

#[cfg(test)]
mod tests {
    use expr::Expr::*;
    use expr::interp;
    use expr::Environment;
    use expr::Expr; 
    fn expr(i: i64) -> Box<Expr> {
        Box::new(Num(i))
    }
    
    #[test]
    fn test_num() {
        assert_eq!(interp(Num(5), &Environment::new()).num(), Ok(5));
    }


    #[test]
    fn test_plus() {
        let add_interp = |x, y| { 
            interp(Plus(expr(x),expr(y)), &Environment::new()) 
        };
        assert_eq!(add_interp(5,6).num(), Ok(11));
        assert_eq!(add_interp(1,3).num(), Ok(4));
        assert_eq!(add_interp(0,1000).num(), Ok(1000));
    }

    #[test]
    fn test_times() {
        let multiply_interp = |x, y| { 
            interp(Times(expr(x),expr(y)), &Environment::new()) 
        };
        assert_eq!(multiply_interp(5,6).num(), Ok(30));
        assert_eq!(multiply_interp(1,3).num(), Ok(3));
        assert_eq!(multiply_interp(0,1000).num(), Ok(0));
    }

    #[test]
    fn test_functions() {
        let fexpr = Apply(box Func("x".to_string(), box Plus(expr(2),box Id("x".to_string()))), expr(2));
        assert_eq!(interp(fexpr,&Environment::new()).num(), Ok(4));
    }
}
