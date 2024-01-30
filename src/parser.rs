use crate::lexer::token::kind::Kind;
use crate::lexer::token::Token;
use std::error::Error;

mod value;
use value::Value;

pub struct Parser {}

impl Parser {
    // Performs Syntax Analysis
    pub fn parse(tokens: &Vec<Token>) -> Result<Node, Box<dyn Error>> {}

    // Performs Semantic Analysis
    pub fn validate(ast: &Node) -> Result<(), Box<dyn Error>> {}
}

#[derive(Debug)]
pub struct Node {
    Type: Kind,
    Args: Option<Vec<Arg>>,
}

#[derive(Debug)]
enum Arg {
    Val(Value),
    Ref(Node),
}
