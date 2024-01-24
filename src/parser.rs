use crate::lexer::token::Token;
use std::error::Error;

mod assert;
mod data;

use data::Data;

pub struct Parser {}
/*
impl Parser {
    // Performs Syntax Analysis
    pub fn parse(tokens: &Vec<Token>) -> Result<AbstractSyntaxTree, Box<dyn Error>> {
        let n = tokens.len();
        if tokens[0] != '(' || tokens[n-1] != ')' {
            return Err(error("unbalanced brackets"));
        }

        Ok(...)
    }

    // Performs Semantic Analysis
    pub fn validate(ast: &AbstractSyntaxTree) -> Result<(), Box<dyn Error>> {}
}
*/
#[derive(Debug)]
pub struct AbstractSyntaxTree<'a> {
    fn_name: &'a Token,
    args: Option<Vec<Arg<'a>>>,
}

#[derive(Debug)]
enum Arg<'a> {
    Name,
    Data(Data<'a>),
    Index,
    FnCall(AbstractSyntaxTree<'a>),
}
