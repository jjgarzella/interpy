// Much of this code was inspired by http://keepcalmandlearnrust.com/2016/08/iterator-and-peekable/

use super::parens::ParenCond;
use std::fmt;

// For consume_while
use std::str::Chars;
use std::iter::Peekable;

#[derive(Clone, Debug)]
pub enum Token {
    Symbol(String),
    OpenParen,
    CloseParen,
    EOF,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Token::*;
        match self {
            &Symbol(ref id) => write!(f, "Sym({})", id),
            &OpenParen => write!(f, "("),
            &CloseParen => write!(f, ")"),
            &EOF => write!(f, "EOF"),
        }
    }
}

pub type Tokens = Vec<Token>;

pub trait Tokenizer {
    fn tokenize(&self) -> Tokens;
}

impl Tokenizer for String {
    fn tokenize(&self) -> Tokens {
        let mut iterator = self.chars().peekable();
        let mut tokens: Tokens = vec![];
        loop {
            //debug!("Next token: {:?}", iterator.peek());
            match iterator.peek() {
                Some(&c) if c.is_open_paren() => {
                        tokens.push(Token::OpenParen);
                        iterator.next().unwrap();
                },
                Some(&c) if c.is_close_paren() => {
                        tokens.push(Token::CloseParen);
                        iterator.next().unwrap();
                },
                Some(&c) if c.is_whitespace() => {
                    iterator.next().unwrap();
                }
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
