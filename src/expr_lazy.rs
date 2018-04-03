// <Expr> : 
//              num
//              plus
//              times
//              id
//              lambda
//              app

use std::collections::HashMap;
use std::cell::RefCell;
use std::fmt;
use std::ops::Deref;

type Identifier = String;

#[derive(Clone,Debug)]
struct Thunk {
    body: Expr,
    env: Environment,
    done_val: RefCell<Option<Value>>,
}

impl Thunk {
    fn force(&self) -> Value {
        if let &Some(ref v) = self.done_val.borrow().deref() {
            return (*v).clone()
        }
        let v = interp(self.body.clone(),&self.env);
        self.done_val.replace(Some(v.clone()));
        v
    }
}

#[derive(Clone,Debug)]
pub struct Environment(RefCell<HashMap<Identifier, Thunk>>);

impl Environment {
    pub fn new() -> Environment {
        Environment {
            0: RefCell::new(HashMap::new()),
        }
    }

    fn lookup(&self, id: Identifier) -> Value {
        match self.0.borrow().get(&id) {
            Some(val) => val.force(),
            None => panic!("Free variable {}", id),
        }
    }

    fn extend(&self, id: Identifier, val: Thunk) {
        self.0.borrow_mut().insert(id,val);
    }
}

#[derive(Clone,Debug)]
pub enum Value {
    Num(i64),
    Func(Identifier,Expr,Environment),
}

impl Value {
    pub fn num(&self) -> Result<i64,&str> {
        use expr_lazy::Value::*;
        match self {
            &Num(n) => Ok(n),
            &Func(_,_,_) => Err("function"),
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Value::*;
        match self {
            &Num(n) => write!(f, "Num({})", n),
            &Func(ref id, ref expr, ref env) => write!(f, "Func({},{},{:?})", id, expr, env),
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

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Expr::*;
        match self {
            &Num(x) => write!(f, "Num({})", x),
            &Plus(box ref lhs, box ref rhs) => write!(f, "Plus({},{})", lhs, rhs),
            &Times(box ref lhs, box ref rhs) => write!(f, "Times({},{})", lhs, rhs),
            &Id(ref id) => write!(f, "Id({})", id),
            &Func(ref param, box ref body) => write!(f, "Func({},{})", param, body),
            &Apply(box ref func, box ref arg) => write!(f, "Apply({}, {})", func, arg),
        }
    }
}

fn binary_num_op(op: &Fn(i64, i64) -> i64, left: Value, right: Value) -> Value {
    use self::Value::Num;
    match (left, right) {
        (Num(l), Num(r)) => Num(op(l,r)),
        _ => panic!("Not a number"),
    }
}

pub fn interp(expr: Expr, env: &Environment) -> Value {
    use self::Expr::*;
    match expr {
        Num(n) => Value::Num(n),
        Plus(box lhs, box rhs) => binary_num_op(&|x,y| {x+y}, interp(lhs,env), interp(rhs,env)),
        Times(box lhs, box rhs) => binary_num_op(&|x,y| {x*y}, interp(lhs,env), interp(rhs,env)),
        Id(id) => env.lookup(id),
        Func(param,box body) => Value::Func(param,body,env.clone()),
        Apply(box func, box arg) => match interp(func,env) {
            Value::Func(id,body,env) => {
                env.extend(id,Thunk {
                    body: arg.clone(),
                    env: env.clone(), 
                    done_val: RefCell::new(None) 
                });
                interp(body,&env)
            }
            _ => panic!("Not a function"),
        },
    }
}

#[cfg(test)]
mod tests {
    use expr_lazy::Expr::*;
    use expr_lazy::interp;
    use expr_lazy::Environment;
    use expr_lazy::Expr; 
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
    
    #[test]
    fn test_lazy() {
        let bad_eval = Apply(expr(1),expr(2));
        let constant_function = Func("x".to_string(), expr(5));
        let good_lazy_program = Apply(box constant_function, box bad_eval);

        assert_eq!(interp(good_lazy_program,&Environment::new()).num(),Ok(5));
    }
}
