use super::lexeme::Lexeme;
use regex::Regex;

#[derive(Debug)]
pub enum Kind {
    Seperator(SeperatorKind),
    Function(FnType),
    Name,
    Data(DataKind),
}

impl Kind {
    pub fn get(lexeme: &Lexeme) -> Option<Self> {
        if let Some(kind) = SeperatorKind::get(&lexeme) {
            return Some(Self::Seperator(kind));
        }
        if let Some(kind) = DataKind::get(&lexeme) {
            return Some(Self::Data(kind));
        }
        if let Some(kind) = FnType::get(&lexeme) {
            return Some(Self::Function(kind));
        }
        let re = Regex::new(r"^[a-z]+(?:-[a-z]+)*$").unwrap();
        if re.is_match(&lexeme.value) {
            return Some(Self::Name);
        }

        None
    }
}

#[derive(Debug)]
pub enum DataKind {
    Number(NumKind),
    Chr,
    Str,
    Bool(BoolKind),
}

impl DataKind {
    pub fn get(lexeme: &Lexeme) -> Option<Self> {
        use DataKind::*;
        let mut re = Regex::new(r"^'[^']*'$").unwrap();
        if re.is_match(&lexeme.value) {
            return Some(Chr);
        }
        re = Regex::new(r#"^".*"$"#).unwrap();
        if re.is_match(&lexeme.value) {
            return Some(Str);
        }
        if let Some(numkind) = NumKind::get(&lexeme) {
            return Some(Number(numkind));
        }
        if let Some(boolkind) = BoolKind::get(&lexeme) {
            return Some(Bool(boolkind));
        }

        None
    }
}

#[derive(Debug)]
pub enum BoolKind {
    True,
    False,
}

impl BoolKind {
    pub fn get(lexeme: &Lexeme) -> Option<Self> {
        use BoolKind::*;
        let val: &str = &lexeme.value;
        match val {
            "true" => Some(True),
            "false" => Some(False),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub enum NumKind {
    Hexadecimal,
    Decimal,
    Octal,
    Binary,
}

impl NumKind {
    pub fn get(lexeme: &Lexeme) -> Option<Self> {
        use NumKind::*;
        let mut re = Regex::new(r"^0[xX][0-9a-fA-F]+$").unwrap();
        if re.is_match(&lexeme.value) {
            return Some(Hexadecimal);
        }
        re = Regex::new(r"^-?(:?0d)?[0-9]+$").unwrap();
        if re.is_match(&lexeme.value) {
            return Some(Decimal);
        }
        re = Regex::new(r"^0o[0-7]+$").unwrap();
        if re.is_match(&lexeme.value) {
            return Some(Octal);
        }
        re = Regex::new(r"^0b[01]+$").unwrap();
        if re.is_match(&lexeme.value) {
            return Some(Binary);
        }

        None
    }
}

#[derive(Debug)]
pub enum SeperatorKind {
    FnBegin,        // (
    ArgsSeperator,  //
    FnEnd,          // )
    ValueSeperator, // ,
    ListBegin,      // [
    ListEnd,        // ]
}

impl SeperatorKind {
    pub fn get(lexeme: &Lexeme) -> Option<Self> {
        use SeperatorKind::*;
        let value: &str = &lexeme.value;
        let kind: Option<SeperatorKind> = match value {
            "(" => Some(FnBegin),
            " " => Some(ArgsSeperator),
            ")" => Some(FnEnd),
            "," => Some(ValueSeperator),
            "[" => Some(ListBegin),
            "]" => Some(ListEnd),
            _ => None,
        };

        return kind;
    }
}

#[derive(Debug)]
pub enum FnType {
    Io(Iotype),
    Operator(OperatorType),
    ControlFlow(FlowType),
}

impl FnType {
    pub fn get(lexeme: &Lexeme) -> Option<Self> {
        if let Some(func) = Iotype::get(&lexeme) {
            return Some(Self::Io(func));
        }
        if let Some(operator) = OperatorType::get(&lexeme) {
            return Some(Self::Operator(operator));
        }
        if let Some(flowtype) = FlowType::get(&lexeme) {
            return Some(Self::ControlFlow(flowtype));
        }

        None
    }
}

#[derive(Debug)]
pub enum Iotype {
    Input,
    Print,
    Error,
}

impl Iotype {
    pub fn get(lexeme: &Lexeme) -> Option<Self> {
        let value: &str = &lexeme.value;
        use Iotype::*;
        let typ = match value {
            "input" => Some(Input),
            "print" => Some(Print),
            "error" => Some(Error),
            _ => None,
        };

        return typ;
    }
}

#[derive(Debug)]
pub enum FlowType {
    FnFlow,
    IfElse,
    Loop,
    InPlace,
    Match,
    Case,
    Interceptor(FlowInterceptor),
}

impl FlowType {
    pub fn get(lexeme: &Lexeme) -> Option<Self> {
        let value: &str = &lexeme.value;
        use FlowType::*;
        let mut typ = match value {
            "loop" => Some(Loop),
            "begin" => Some(FnFlow),
            "if-else" => Some(IfElse),
            "match" => Some(Match),
            "case" => Some(Case),
            "%%" => Some(InPlace),
            _ => None,
        };

        if let None = typ {
            if let Some(interceptor_type) = FlowInterceptor::get(&lexeme) {
                typ = Some(Self::Interceptor(interceptor_type));
            }
        }

        return typ;
    }
}

#[derive(Debug)]
pub enum FlowInterceptor {
    Break,
    Continue,
    Return,
    Exit,
}

impl FlowInterceptor {
    pub fn get(lexeme: &Lexeme) -> Option<Self> {
        let value: &str = &lexeme.value;
        use FlowInterceptor::*;
        let typ = match value {
            "break" => Some(Break),
            "continue" => Some(Continue),
            "return" => Some(Return),
            "exit" => Some(Exit),
            _ => None,
        };

        return typ;
    }
}

#[derive(Debug)]
pub enum OperatorType {
    Arithmetic(ArithmeticOperator),
    Assignment(AssignmentOperator),
    Comparison(ComparisonOperator),
    Logical(LogicalOperator),
    Bitwise(BitwiseOperator),
}

impl OperatorType {
    pub fn get(lexeme: &Lexeme) -> Option<Self> {
        if let Some(kind) = ArithmeticOperator::get(&lexeme) {
            return Some(Self::Arithmetic(kind));
        }
        if let Some(kind) = AssignmentOperator::get(&lexeme) {
            return Some(Self::Assignment(kind));
        }
        if let Some(kind) = ComparisonOperator::get(&lexeme) {
            return Some(Self::Comparison(kind));
        }
        if let Some(kind) = LogicalOperator::get(&lexeme) {
            return Some(Self::Logical(kind));
        }
        if let Some(kind) = BitwiseOperator::get(&lexeme) {
            return Some(Self::Bitwise(kind));
        }

        None
    }
}

#[derive(Debug)]
pub enum ArithmeticOperator {
    Addition,       // +
    Subtraction,    // -
    Multiplication, // *
    Division,       // /
}

impl ArithmeticOperator {
    pub fn get(lexeme: &Lexeme) -> Option<Self> {
        use ArithmeticOperator::*;
        let value: &str = &lexeme.value;
        let kind: Option<ArithmeticOperator> = match value {
            "add" => Some(Addition),
            "sub" => Some(Subtraction),
            "mul" => Some(Multiplication),
            "div" => Some(Division),
            _ => None,
        };

        return kind;
    }
}

#[derive(Debug)]
pub enum AssignmentOperator {
    Assign,    // =
    AddAssign, // +=
    SubAssign, // -=
    MulAssign, // *=
    DivAssign, // /=
}

impl AssignmentOperator {
    pub fn get(lexeme: &Lexeme) -> Option<Self> {
        use AssignmentOperator::*;
        let value: &str = &lexeme.value;
        let kind: Option<AssignmentOperator> = match value {
            "assign" => Some(Assign),
            "add-by" => Some(AddAssign),
            "sub-by" => Some(SubAssign),
            "mul-by" => Some(MulAssign),
            "div-by" => Some(DivAssign),
            _ => None,
        };

        return kind;
    }
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

impl ComparisonOperator {
    pub fn get(lexeme: &Lexeme) -> Option<Self> {
        use ComparisonOperator::*;
        let value: &str = &lexeme.value;
        let kind: Option<ComparisonOperator> = match value {
            "assert-eq" => Some(Equal),
            "assert_ne" => Some(NotEqual),
            "assert_gt" => Some(GreaterThan),
            "assert_lt" => Some(LessThan),
            "assert_ge" => Some(GreatorThanOrEqual),
            "assert_le" => Some(LessThanOrEqual),
            _ => None,
        };

        return kind;
    }
}

#[derive(Debug)]
pub enum LogicalOperator {
    LogicalAnd, // &&
    LogicalOr,  // ||
    LogicalNot, // !!
}

impl LogicalOperator {
    pub fn get(lexeme: &Lexeme) -> Option<Self> {
        use LogicalOperator::*;
        let value: &str = &lexeme.value;
        let kind: Option<LogicalOperator> = match value {
            "and" => Some(LogicalAnd),
            "or" => Some(LogicalOr),
            "not" => Some(LogicalNot),
            _ => None,
        };

        return kind;
    }
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

impl BitwiseOperator {
    pub fn get(lexeme: &Lexeme) -> Option<Self> {
        use BitwiseOperator::*;
        let value: &str = &lexeme.value;
        let kind: Option<BitwiseOperator> = match value {
            "bitwise-and" => Some(BitwiseAnd),
            "bitwise-or" => Some(BitwiseOr),
            "bitwise-not" => Some(BitwiseNot),
            "bitwise-xor" => Some(BitwiseXOr),
            "bitwise-ls" => Some(BitwiseLeftShift),
            "bitwise-rs" => Some(BitwiseRightShift),
            _ => None,
        };

        return kind;
    }
}
