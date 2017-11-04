
enum GenericSyntaxTree<'a> {
    Symbol(&'a str),
    List(Vec<GenericSyntaxTree<'a>>),
}

struct Parser {
    position: usize,
    code: String,
}

enum Token<'a> {
    Symbol(&'a str),
    OpenParen,
    CloseParen,
    EOF,
}

impl Parser {

    pub fn new(code: String) -> Parser {
        Parser {
            code: code,
            position: 0,
        }
    }
    
    fn isWhitespace(&self, c: char) -> bool {
        [' ','\t','\n','\r'].contains(&c)
    }

    fn nextWhitespace(&self) -> usize {
        let mut p = self.position;
        
        while !self.isWhitespace(self.code.chars().nth(p).unwrap()) {
            p += 1;
        }

        p
    }

    fn skipWhitespace(&mut self) {
        let mut p = self.position;
        
        while self.isWhitespace(self.code.chars().nth(p).unwrap()) {
            p += 1;
        }

        self.position = p;
    }

    fn eat(&mut self) -> Token {
        
        match self.code.chars().nth(self.position) {
            Some(c) if ['('].contains(&c) => Token::OpenParen,
            Some(c) if [')'].contains(&c) => Token::OpenParen,
            Some(_) => {
                let end = self.nextWhitespace();
                let slice = &self.code[self.position..end];
                self.position = end;
                Token::Symbol(slice)
            }
            None => Token::EOF,
        }
    }

    fn parse(&mut self) -> GenericSyntaxTree {
        self.parse_rec(0)
    }

    fn parse_rec(&mut self, level: usize) -> GenericSyntaxTree {
        let mut components = Vec::new();
        self.skipWhitespace();
        loop {
            match self.eat() {
                Token::OpenParen => {
                    self.position += 1;
                    components.push(self.parse_rec(level + 1));
                },
                Token::Symbol(s) => components.push(GenericSyntaxTree::Symbol(s)),
                Token::CloseParen => break,
                Token::EOF => {
                    if level == 0 {
                        break;
                    } else {
                        panic!("Parser error: Unmatch parens");
                    }
                },
            }
        }
        GenericSyntaxTree::List(components)
    }
}

#[cfg(test)]
mod tests {
    
    #[test]
    fn test_basic() {
        let tree = Parser::new("(blah)").parse();
    }

    #[test]
    fn test_levels() {
        let tree = Parser::new("(boom (chicka chicka))").parse();
    }

    #[test]
    fn test_complex() {
        let tree = Parser::new("(+ ((lambda (x) (* 3 x)) 5) (+ (+ (* (+ 2 3) 2) -1) 10))").parse();
    }
}
