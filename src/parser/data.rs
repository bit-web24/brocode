use super::ParseTree;
use crate::lexer::token::Token;
use std::fmt;

#[derive(Debug)]
pub struct Data<'d> {
    pub metadata: MetaData,
    pub value: Value<'d>,
}

#[derive(Debug)]
pub enum MetaData {
    ValueSign(DataType),
    ArraySign(Index, DataType),
    FunctionSign(Vec<MetaData>, Box<MetaData>),
    MatrixSign(Dimension, DataType),
}

#[derive(Debug)]
pub struct Index {
    pub value: u32,
}

#[derive(Debug)]
pub struct Dimension {
    pub rows: i64,
    pub columns: i64,
}

#[derive(Debug)]
pub enum DataType {
    Num(Number),
    Chr,
}

#[derive(Debug)]
enum Number {
    Dec,
    Hex,
    Bin,
    Oct,
}

#[derive(Debug)]
pub enum Value<'a> {
    FunctionSign(Parameters, Box<ParseTree<'a>>),
    UnitVal(Option<Box<dyn UnitVal>>),
    Vector(Option<Vec<Box<dyn UnitVal>>>),
    Matrix(Option<Vec<Vec<Box<dyn UnitVal>>>>),
}

trait UnitVal: fmt::Debug {}

#[derive(Debug)]
pub enum Datum {
    Chr(CharVal),
    Num(NumVal),
}

impl UnitVal for Datum {}

#[derive(Debug)]
pub struct CharVal {
    value: char,
}

#[derive(Debug)]
enum NumVal {
    Dec(f64),
    Hex(String),
    Bin(u64),
    Oct(u64),
}

#[derive(Debug)]
pub struct Str {
    value: Vec<CharVal>,
}

impl UnitVal for Str {}

#[derive(Debug)]
pub struct Parameters {
    value: Vec<Token>,
}
