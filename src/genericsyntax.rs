
use super::tokenizer::{Token, Tokens};
use std::fmt;

pub enum GenericSyntaxTree {
    Symbol(String),
    List(Vec<GenericSyntaxTree>),
}

impl GenericSyntaxTree {

    fn from_tokens_helper(tokens: &Tokens, index: usize) -> (GenericSyntaxTree, usize) {
        let mut syntax: Vec<GenericSyntaxTree> = vec![];

        let mut i = index;
        while i < tokens.len() {
            match &tokens[i] {
                &Token::Symbol(ref s) => syntax.push(GenericSyntaxTree::Symbol(s.clone())),
                &Token::OpenParen => {
                    let (tree, new_ind) = GenericSyntaxTree::from_tokens_helper(tokens, i+1);
                    syntax.push(tree);
                    i = new_ind;
                    continue;
                },
                &Token::CloseParen => return (GenericSyntaxTree::List(syntax), i+1),
                &Token::EOF => return (GenericSyntaxTree::List(syntax), i+1), // add in error?
            }
            i += 1
        }

        panic!("should be unreachable - no EOF token or close paren found");
    }

    pub fn from_tokens(tokens: &Tokens) -> GenericSyntaxTree {
        GenericSyntaxTree::from_tokens_helper(tokens, 0).0
    }
}

impl fmt::Display for GenericSyntaxTree {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::GenericSyntaxTree::*;
        match self {
            &Symbol(ref id) => write!(f, "\"{}\"", id),
            &List(ref child_trees) => {
                try!(write!(f, "( "));
                for tree in child_trees {
                    try!(write!(f, "{} ", tree));
                }
                write!(f, ")")
            }
        }
    }
}
