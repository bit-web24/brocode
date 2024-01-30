#[derive(Debug)]
pub enum Value {
    Char(char),
    String(String),
    Number(Number),
    Array(Array),
    Matrix(Matrix),
}

#[derive(Debug)]
enum Number {
    Dec(i64),
    Oct(i64),
    Hex(i64),
    Bin(i64),
}

#[derive(Debug)]
struct Array {
    val: Vec<i64>,
}

#[derive(Debug)]
struct Matrix {
    val: Vec<Array>,
}
