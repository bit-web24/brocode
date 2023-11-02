use super::lexeme::Lexeme;

#[derive(Debug)]
pub enum Kind {
    Seperator(SeperatorKind),
    Function(FnType),
    Name,
    Data(DataKind),
}

impl Kind {
    pub fn get(lexeme: &Lexeme) -> Self {
        if let Some(kind) = SeperatorKind::get(&lexeme) {
            return Kind::Seperator(kind);
        }
        if let Some(kind) = DataKind::get(&lexeme) {
            return Kind::Data(kind);
        }

        return Kind::Name;
    }
}

#[derive(Debug)]
pub enum DataKind {
    Number,
    Chr,
}

impl DataKind {
    pub fn get(lexeme: &Lexeme) -> Option<Self> {
        use DataKind::*;
        let mut data_iter = lexeme.value.chars();
        if lexeme.value.len() == 3
            && data_iter.nth(0) == Some('\'')
            && data_iter.last() == Some('\'')
        {
            return Some(Chr);
        }
        if let Ok(_) = lexeme.value.parse::<i32>() {
            return Some(Number);
        }

        None
    }
}

#[derive(Debug)]
pub enum SeperatorKind {
    FnBegin,                 // [
    ArgsSeperator,           //
    FnEnd,                   // ]
    DataBegin,               // <
    ParamsAndValueSeperator, // ,
    MetaDataSeperator,       // |
    DataEnd,                 // >
    ListBegin,               // {
    ListEnd,                 // }
    ParamBegin,              // (
    ParamEnd,                // )
}

impl SeperatorKind {
    pub fn get(lexeme: &Lexeme) -> Option<Self> {
        use SeperatorKind::*;
        let value: &str = &lexeme.value;
        let kind: Option<SeperatorKind> = match value {
            "[" => Some(FnBegin),
            " " => Some(ArgsSeperator),
            "]" => Some(FnEnd),
            "<" => Some(DataBegin),
            "," => Some(ParamsAndValueSeperator),
            "|" => Some(MetaDataSeperator),
            ">" => Some(DataEnd),
            "{" => Some(ListBegin),
            "}" => Some(ListEnd),
            "(" => Some(ParamBegin),
            ")" => Some(ParamEnd),
            _ => None,
        };

        return kind;
    }
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
