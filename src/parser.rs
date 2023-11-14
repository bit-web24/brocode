use crate::lexer::token::Token;
use std::error::Error;

mod data;

use data::Data;

pub struct Parser {}

impl Parser {
    pub fn parse_syntax(tokens: &Vec<Token>) -> Result<ParseTree, Box<dyn Error>> {
        Ok(ParseTree {
            fn_name: &tokens[1],
            args: Some(vec![Arg::Name]),
        })
    }
}

#[derive(Debug)]
pub struct ParseTree<'a> {
    fn_name: &'a Token,
    args: Option<Vec<Arg<'a>>>,
}

#[derive(Debug)]
enum Arg<'a> {
    Name,
    Data(Data<'a>),
    Index,
    FnCall(ParseTree<'a>),
}
