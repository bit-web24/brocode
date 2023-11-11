use super::lexeme::Lexeme;
use regex::Regex;

#[derive(Debug)]
pub enum Kind {
    Seperator(SeperatorKind),
    Function(FnType),
    Name,
    Param,
    Data(DataKind),
    Metadata(MetadataKind),
}

impl Kind {
    pub fn get(lexeme: &Lexeme) -> Option<Self> {
        if let Some(kind) = SeperatorKind::get(&lexeme) {
            return Some(Self::Seperator(kind));
        }
        if let Some(kind) = DataKind::get(&lexeme) {
            return Some(Self::Data(kind));
        }
        if let Some(kind) = MetadataKind::get(&lexeme) {
            return Some(Self::Metadata(kind));
        }
        let mut re = Regex::new(r"^\$[a-zA-Z\d]+$").unwrap();
        if re.is_match(&lexeme.value) {
            return Some(Self::Param);
        }
        re = Regex::new(r"^[a-z_A-Z]+\d+?$").unwrap();
        if re.is_match(&lexeme.value) {
            return Some(Self::Name);
        }
        if let Some(kind) = FnType::get(&lexeme) {
            return Some(Self::Function(kind));
        }

        None
    }
}

#[derive(Debug)]
pub enum MetadataKind {
    DataType(DataType),
    ContainerType(ContainerType),
    Dimension,
    Index,
}

impl MetadataKind {
    pub fn get(lexeme: &Lexeme) -> Option<Self> {
        let mut re = Regex::new(r"^[1-9]x[1-9]$").unwrap();
        if re.is_match(&lexeme.value) {
            return Some(Self::Dimension);
        }
        re = Regex::new(r"^#[0-9]+$").unwrap();
        if re.is_match(&lexeme.value) {
            return Some(Self::Index);
        }
        if let Some(typ) = ContainerType::get(&lexeme) {
            return Some(Self::ContainerType(typ));
        }
        if let Some(typ) = DataType::get(&lexeme) {
            return Some(Self::DataType(typ));
        }

        None
    }
}

#[derive(Debug)]
pub enum ContainerType {
    Matrix,
    Function,
    Array,
}

impl ContainerType {
    pub fn get(lexeme: &Lexeme) -> Option<Self> {
        let val: &str = &lexeme.value;
        use ContainerType::*;
        let typ = match val {
            "matrix" => Some(Matrix),
            "fn" => Some(Function),
            "array" => Some(Array),
            _ => None,
        };

        return typ;
    }
}

#[derive(Debug)]
pub enum DataType {
    Character,
    Decimal,
    Octal,
    Binary,
    Hexadecimal,
    Boolean,
}

impl DataType {
    pub fn get(lexeme: &Lexeme) -> Option<Self> {
        let val: &str = &lexeme.value;
        use DataType::*;
        let typ = match val {
            "char" => Some(Character),
            "dec" => Some(Decimal),
            "oct" => Some(Octal),
            "bin" => Some(Binary),
            "hex" => Some(Hexadecimal),
            "bool" => Some(Boolean),
            _ => None,
        };

        return typ;
    }
}

#[derive(Debug)]
pub enum DataKind {
    Number(NumKind),
    Chr,
    Bool(BoolKind),
}

impl DataKind {
    pub fn get(lexeme: &Lexeme) -> Option<Self> {
        use DataKind::*;
        let re = Regex::new(r"^'[^']*'$").unwrap();
        if re.is_match(&lexeme.value) {
            return Some(Chr);
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
        re = Regex::new(r"^(:?0d)?[0-9\.]+$").unwrap();
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
    BuiltIn(BuiltInFnType),
}

impl FnType {
    pub fn get(lexeme: &Lexeme) -> Option<Self> {
        if let Some(builtin_type) = BuiltInFnType::get(&lexeme) {
            return Some(Self::BuiltIn(builtin_type));
        }

        None
    }
}

#[derive(Debug)]
pub enum BuiltInFnType {
    General(General),
    Operator(Operator),
    Functionality(Functionality),
}

impl BuiltInFnType {
    pub fn get(lexeme: &Lexeme) -> Option<Self> {
        if let Some(operator) = Operator::get(&lexeme) {
            return Some(Self::Operator(operator));
        }
        if let Some(func) = Functionality::get(&lexeme) {
            return Some(Self::Functionality(func));
        }
        if let Some(func) = General::get(&lexeme) {
            return Some(Self::General(func));
        }
        None
    }
}

#[derive(Debug)]
pub enum General {
    Break,
    Continue,
    Return,
    Stdout,
    Stdin,
}

impl General {
    pub fn get(lexeme: &Lexeme) -> Option<Self> {
        let value: &str = &lexeme.value;
        use General::*;
        let typ = match value {
            "break" => Some(Break),
            "continue" => Some(Continue),
            "return" => Some(Return),
            "stdout" => Some(Stdout),
            "stdin" => Some(Stdin),
            _ => None,
        };

        return typ;
    }
}

#[derive(Debug)]
pub enum Functionality {
    FnFlow,
    IfElse,
    Loop,
    InPlace,
}

impl Functionality {
    pub fn get(lexeme: &Lexeme) -> Option<Self> {
        let value: &str = &lexeme.value;
        let typ = match value {
            "@" => Some(Self::Loop),
            "~" => Some(Self::FnFlow),
            ":?" => Some(Self::IfElse),
            "%%" => Some(Self::InPlace),
            _ => None,
        };

        return typ;
    }
}

#[derive(Debug)]
pub enum Operator {
    Arithmetic(ArithmeticOperator),
    Assignment(AssignmentOperator),
    Comparison(ComparisonOperator),
    Logical(LogicalOperator),
    Bitwise(BitwiseOperator),
}

impl Operator {
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
            "+" => Some(Addition),
            "-" => Some(Subtraction),
            "*" => Some(Multiplication),
            "/" => Some(Division),
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
            "=" => Some(Assign),
            "+=" => Some(AddAssign),
            "-=" => Some(SubAssign),
            "*=" => Some(MulAssign),
            "/=" => Some(DivAssign),
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
            "==" => Some(Equal),
            "!=" => Some(NotEqual),
            ">" => Some(GreaterThan),
            "<" => Some(LessThan),
            ">=" => Some(GreatorThanOrEqual),
            "<=" => Some(LessThanOrEqual),
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
            "&&" => Some(LogicalAnd),
            "||" => Some(LogicalOr),
            "!!" => Some(LogicalNot),
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
            "&" => Some(BitwiseAnd),
            "|" => Some(BitwiseOr),
            "!" => Some(BitwiseNot),
            "^" => Some(BitwiseXOr),
            "<<" => Some(BitwiseLeftShift),
            ">>" => Some(BitwiseRightShift),
            _ => None,
        };

        return kind;
    }
}
