// <ExprCore> : 
//              num
//              plus
//              times
//              id
//              lambda
//              app

enum ExprCore {
    Num(i64),
    Plus(Box<ExprCore>, Box<ExprCore>),
    Times(Box<ExprCore>, Box<ExprCore>),
//    Id(String),
//    Lambda(String, Box<ExprCore>),
//    App(Box<ExprCore>, Box<ExprCore>),
}

type Value = i64;

fn interp(expr: ExprCore) -> Value {
    use self::ExprCore::*;
    match expr {
        Num(n) => n,
        Plus(box lhs, box rhs) => interp(lhs) + interp(rhs),
        Times(box lhs, box rhs) => interp(lhs) * interp(rhs),
    }
}

#[cfg(test)]
mod tests {
    use core::ExprCore::*;
    use core::interp;
    #[test]
    fn test_num() {
        assert_eq!(interp(Num(5)), 5);
    }

    #[test]
    fn test_plus() {
        let add_interp = |x, y| { interp(Plus(Box::new(Num(x)),Box::new(Num(y)))) };
        assert_eq!(add_interp(5,6), 11);
        assert_eq!(add_interp(1,3), 4);
        assert_eq!(add_interp(0,1000), 1000);
    }

    #[test]
    fn test_times() {
        let multiply_interp = |x, y| { interp(Times(Box::new(Num(x)),Box::new(Num(y)))) };
        assert_eq!(multiply_interp(5,6), 30);
        assert_eq!(multiply_interp(1,3), 3);
        assert_eq!(multiply_interp(0,1000), 0);
    }
}
