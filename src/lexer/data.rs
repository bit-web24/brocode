pub struct Data {
    pub metadata: MetaData,
    pub value: String,
}

pub enum MetaData {
    FnSig(Vec<MetaData>, Box<MetaData>),
    FnRef,
    FnInv,
    Matrix(Dimension, DataType),
}

pub struct Dimension {
    pub rows: i64,
    pub columns: i64,
}

pub enum DataType {
    Dec,
    Hex,
    Bin,
    Oct,
    Str,
    Chr,
}
