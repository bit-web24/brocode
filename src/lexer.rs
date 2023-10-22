mod stdin;
use stdin::Expression;

pub fn tokenize(expression: Expression) -> Result<Vec<Token>, error::Error> {
	// code to tokenize an expression
}

struct Token {
	lexeme: String,
	Ref: Symbol,
}


enum SymbolTable {
	Function(FnType),
}

enum FnType {
	Identifier,
	Flow,
	ArithmeticOperator(AO_t),
	ComparisonOperator(CO_t),
	LogicalOperator(LO_t),
}

enum ArithmeticOperator {
	Assignment,
	Increment
}
