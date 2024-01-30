#[derive(Debug)]
pub enum Value {
    String(Str),
    Number(Number),
    Array(Array),
    Char(char),
}
