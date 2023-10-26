pub struct Data {
    pub metadata: MetaData,
    pub value: Value,
}

pub enum MetaData {
    FunctionSign(Vec<MetaData>, Box<MetaData>),
    MatrixSign(Dimension, DataType),
}

pub struct Dimension {
    pub rows: i64,
    pub columns: i64,
}

pub enum DataType {
    Num(Number),
    Str,
    Chr,
}

enum Number {
    Dec,
    Hex,
    Bin,
    Oct,
}

pub enum Value {
    FunctionSign(Parameters, RawFunction),
    Matrix(Vec<Vec<Box<dyn UnitVal>>>),
}

trait UnitVal {}

pub enum Datum {
    Chr(CharVal),
    Num(NumVal),
}

impl UnitVal for Datum {}

pub struct CharVal {
    value: char,
}

enum NumVal {
    Dec(f64),
    Hex(String),
    Bin(u64),
    Oct(u64),
}

pub struct Str {
    value: Vec<CharVal>,
}

impl UnitVal for Str {}

pub struct Parameters {
    value: Vec<String>,
}

pub enum RawFunction {
    ByCall(Function),
    ByRef(String),
}

pub struct Function {
    Name: String,
    Args: Vec<RawFunction>,
}
