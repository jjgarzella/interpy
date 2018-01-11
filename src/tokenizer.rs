// Much of this code was inspired by http://keepcalmandlearnrust.com/2016/08/iterator-and-peekable/

use super::parens::ParenCond;

// For consume_while
use std::str::Chars;
use std::iter::Peekable;

enum Token {
    Symbol(String),
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
                Some(&c) if c.is_open_paren() => {
                        tokens.push(Token::OpenParen);
                        iterator.next().unwrap();
                },
                Some(&c) if c.is_close_paren() => {
                        tokens.push(Token::CloseParen);
                        iterator.next().unwrap();
                },
                Some(_) => {
                        let is_valid_id_char = |c: char| { !c.is_whitespace() && !c.is_paren() };
                        let ident: String = consume_while(&mut iterator,&is_valid_id_char)
                            .into_iter()
                            .collect();
                        tokens.push(Token::Symbol(ident));
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

// could this be made generic? or possibly improved in some other way?
fn consume_while(it: &mut Peekable<Chars>, cond: &Fn(char) -> bool) -> Vec<char> {
    let mut results: Vec<char> = vec![];

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
