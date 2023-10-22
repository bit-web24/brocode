mod stdin;
use stdin::Expression;

pub fn tokenize(expression: Expression) -> Result<Vec<Token>, error::Error> {
	// code to tokenize an expression
}


#[derive(Debug, Clone, PartialEq)]
struct Token {
	lexeme: Lexeme,
	location: Location,
	kind: Kind,
}

struct Lexeme {
	value: String,
}

#[derive(Debug, Clone, PartialEq)]
struct Location {
	pub line: usize,
	pub column: usize,
	pub len: usize,
}

#[derive(Debug, Clone, PartialEq)]
enum Kind {
	FnBegin,
	FnName(FnType),
	Argument(Data),
	FnEnd,
}

enum FnType {
	UserDefined(String),
	BuiltIn(BuiltInFnType),
}

enum BuiltInFnType {
	General, // Need to define General functions
	Operator(Operator),
	Functionality(Functionality),
}

enum Functionality {
	FnFlow,
	IfElse,
	Loop,
}

enum Operator {
	Arithmetic(ArithmeticOperator),
	Assignment(AssignmentOperator),
	Comparison(ComparisonOperator),
	Logical(LogicalOperator),
	Bitwise(BitwiseOperator),
}

enum ArithmeticOperator {
	Addition, // +
	Subtraction, // -
	Multiplication, // *
	Division, // /
}

enum AssignmentOperator {
	Assign, // =
	AddAssign, // +=
	SubAssign, // -=
	MulAssign, // *=
	DivAssign, // /=
}

enum ComparisonOperator {
	Equal, // ==
	NotEqual, // !=
	GreaterThan, // >
	LessThan, // <
	GreatorThanOrEqual, // >=
	LessThanOrEqual, // <=
}

enum LogicalOperator {
	LogicalAnd, // &&
	LogicalOr, // ||
	LogicalNot, // !!
}

enum BitwiseOperator {
	BitwiseAnd, // &
	BitwiseOr, // |
	BitwiseNot, // !
	BitwiseXOr, // ^
	BitwiseLeftShift, // <<
	BitwiseRightShift, // >>
}

