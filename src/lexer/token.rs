#[derive(Debug)]
pub struct Token {
    pub lexeme: Lexeme,
    pub location: Location,
    pub kind: Kind,
}

#[derive(Debug)]
pub struct Lexeme {
    pub value: String,
}

#[derive(Debug)]
pub struct Location {
    pub line: usize,
    pub column: usize,
    pub len: usize,
}

#[derive(Debug)]
pub enum Kind {
    Seperator(SeperatorKind),
    Function(FnType),
    Name,
    Data(DataKind),
}

#[derive(Debug)]
pub enum DataKind {
    Number(NumKind),
    Chr,
}

#[derive(Debug)]
pub enum NumKind {
    Dec,
    Hex,
    Bin,
    Oct,
}

#[derive(Debug)]
pub enum SeperatorKind {
    FnBegin,           // [
    ArgsSeperator,     //
    FnEnd,             // ]
    DataBegin,         // <
    DataumSeperator,   // ,
    MetaDataSeperator, // |
    DataEnd,           // >
    ListBegin,         // {
    ListEnd,           // }
    ParamBegin,        // (
    ParamEnd,          // )
    ParamsSeperator,   // ,
}

#[derive(Debug)]
pub enum FnType {
    UserDefined,
    BuiltIn(BuiltInFnType),
}

#[derive(Debug)]
pub enum BuiltInFnType {
    General, // Need to define General functions
    Operator(Operator),
    Functionality(Functionality),
}

#[derive(Debug)]
pub enum Functionality {
    FnFlow,
    IfElse,
    Loop,
}

#[derive(Debug)]
pub enum Operator {
    Arithmetic(ArithmeticOperator),
    Assignment(AssignmentOperator),
    Comparison(ComparisonOperator),
    Logical(LogicalOperator),
    Bitwise(BitwiseOperator),
}

#[derive(Debug)]
pub enum ArithmeticOperator {
    Addition,       // +
    Subtraction,    // -
    Multiplication, // *
    Division,       // /
}

#[derive(Debug)]
pub enum AssignmentOperator {
    Assign,    // =
    AddAssign, // +=
    SubAssign, // -=
    MulAssign, // *=
    DivAssign, // /=
}

#[derive(Debug)]
pub enum ComparisonOperator {
    Equal,              // ==
    NotEqual,           // !=
    GreaterThan,        // >
    LessThan,           // <
    GreatorThanOrEqual, // >=
    LessThanOrEqual,    // <=
}

#[derive(Debug)]
pub enum LogicalOperator {
    LogicalAnd, // &&
    LogicalOr,  // ||
    LogicalNot, // !!
}

#[derive(Debug)]
pub enum BitwiseOperator {
    BitwiseAnd,        // &
    BitwiseOr,         // |
    BitwiseNot,        // !
    BitwiseXOr,        // ^
    BitwiseLeftShift,  // <<
    BitwiseRightShift, // >>
}
