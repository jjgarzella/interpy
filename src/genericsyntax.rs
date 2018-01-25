
use super::tokenizer::{Token, Tokens};

pub enum GenericSyntaxTree {
    Symbol(String),
    List(Vec<GenericSyntaxTree>),
}

impl GenericSyntaxTree {

    pub fn from_tokens(tokens: &Tokens) -> GenericSyntaxTree {
        let mut syntax: Vec<GenericSyntaxTree> = vec![];

        for i in 0..tokens.len() {
            match &tokens[i] {
                &Token::Symbol(ref s) => syntax.push(GenericSyntaxTree::Symbol(s.clone())),
                &Token::OpenParen => syntax.push(GenericSyntaxTree::from_tokens(&tokens[i+1..tokens.len()].to_vec())),
                &Token::CloseParen => return GenericSyntaxTree::List(syntax),
                &Token::EOF => return GenericSyntaxTree::List(syntax), // add in error?
            }
        }

        panic!("should be unreachable - no EOF token or close paren found");

    }

}
