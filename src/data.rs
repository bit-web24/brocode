struct Data {
	metadata: MetaData,
	value: Value,
}

enum MetaData {
	FnSig(Vec<MetaData>, MetaData),
	FnRef,
	FnInv,
	Matrix(Dimension, DataType),
}

struct Dimension {
	rows: i64,
	columns: i64,
}

enum DataType {
	Dec,
	Hex,
	Bin,
	Oct,
	Str,
	Chr,
}

enum Value {
	
}
