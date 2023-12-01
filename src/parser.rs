use crate::lexer::token::Token;
use std::error::Error;

mod assert;
mod data;

use data::Data;

pub struct Parser {}

impl Parser {
    pub fn parse_syntax(tokens: &Vec<Token>) -> Result<ParseTree, Box<dyn Error>> {
        let mut parse_tree = ParseTree::new(&tokens[1]);
        // for field in get_fields(tokens)? {
        //     parse_tree.join(field)?
        // }
        Ok(parse_tree)
    }
}

#[derive(Debug)]
pub struct ParseTree<'a> {
    fn_name: &'a Token,
    args: Option<Vec<Arg<'a>>>,
}

impl ParseTree<'_> {
    pub fn new(fn_name: &Token) -> ParseTree<'_> {
        return ParseTree {
            fn_name,
            args: None,
        };
    }
}

#[derive(Debug)]
enum Arg<'a> {
    Name,
    Data(Data<'a>),
    Index,
    FnCall(ParseTree<'a>),
}
