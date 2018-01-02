// Much of this code was inspired by http://keepcalmandlearnrust.com/2016/08/iterator-and-peekable/

use super::parens::ParenCond;

enum Token<'a> {
    Symbol(&'a str),
    OpenParen,
    CloseParen,
    EOF,
}

trait Tokenizer {
    fn tokenize(&self) -> Vec<Token>;
}

impl Tokenizer for String {
    fn tokenize(&self) -> Vec<Token> {
        let mut iterator = self.chars().peekable();
        let mut tokens: Vec<Token> = vec![];
        loop {
            match iterator.peek() {
                Some(&'(') => {
                        tokens.push(Token::OpenParen);
                        iterator.next().unwrap();
                },
                Some(&')') => {
                        tokens.push(Token::CloseParen);
                        iterator.next().unwrap();
                },
                Some(_) => {
                        let is_valid_id_char = |c: char| { !c.is_whitespace() && !c.is_paren() };
                        let chars = consume_while(&mut iterator,&is_valid_id_char);
                        
                },
                None => {
                    tokens.push(Token::EOF);
                    break
                }
            }
        }
        tokens
    }
}

use ::std::iter::{Peekable, Iterator};
fn consume_while<T: Sized>(it: &Peekable<Iterator<Item=T>>, cond: &Fn(T) -> bool) -> Vec<T> {
    let mut results: Vec<T> = vec![];

    while let Some(&t) = it.peek() {
        if cond(t) {
            it.next().unwrap();
            results.push(t);
        } else {
            break;
        }
    }
    results
}
