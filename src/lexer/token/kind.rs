use super::lexeme::Lexeme;

#[derive(Debug)]
pub enum Kind {
    Seperator(SeperatorKind),
    Function(FnType),
    Name,
    Data(DataKind),
}

impl Kind {
    pub fn get_kind(lexeme: &Lexeme) -> Self {
        return Kind::Name;
    }
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
