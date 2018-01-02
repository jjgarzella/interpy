
pub trait ParenCond {
    fn is_paren(&self) -> bool;
    fn is_open_paren(&self) -> bool;
    fn is_close_paren(&self) -> bool;
}

impl ParenCond for char {
    fn is_paren(&self) -> bool {
        self == &'(' || self == &')'
    }
    
    fn is_open_paren(&self) -> bool {
        self == &'('
    }
    
    fn is_close_paren(&self) -> bool {
        self == &')'
    }
}


#[cfg(test)]
mod test {
    use parens::ParenCond;

    #[test]
    fn test_is_paren() {
        assert!('('.is_paren());
        assert!('a'.is_paren());
        assert!(')'.is_paren());
    }

    #[test]
    fn test_is_open_paren() {
        assert!('('.is_open_paren());
        assert!('a'.is_open_paren());
        assert!(')'.is_open_paren());
    }

    #[test]
    fn test_is_close_paren() {
        assert!('('.is_close_paren());
        assert!('a'.is_close_paren());
        assert!(')'.is_close_paren());
    }
}
