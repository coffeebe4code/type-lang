use std::rc::Rc;

#[derive(Debug)]
pub struct FileContainer {
    pub top_items: Vec<Rc<Box<TypeTree>>>,
    pub curried: Vec<Type>,
}

#[derive(Debug)]
pub struct ErrorInfo {
    pub message: String,
    pub code: usize,
}

#[derive(Debug)]
pub struct TagInfo {
    pub name: String,
    pub props: Vec<Rc<Box<TypeTree>>>,
    pub types: Vec<Type>,
    pub curried: Type,
}

#[derive(Debug)]
pub struct StructInfo {
    pub props: Vec<String>,
    pub types: Vec<Type>,
    pub curried: Type,
}

#[derive(Debug)]
pub struct ArgInfo {
    pub curried: Type,
}

#[derive(Debug)]
pub struct DeclaratorInfo {
    pub name: String,
    pub curried: Type,
}

#[derive(Debug)]
pub struct MatchOp {
    pub expr: Rc<Box<TypeTree>>,
    pub curried: Type,
    pub arms: Vec<Rc<Box<TypeTree>>>,
    pub curried_arms: Vec<Type>,
}

#[derive(Debug)]
pub struct ForOp {
    pub in_expr: Rc<Box<TypeTree>>,
    pub in_curried: Type,
    pub body: Rc<Box<TypeTree>>,
    pub body_curried: Type,
}

#[derive(Debug)]
pub struct BinaryOp {
    pub left: Rc<Box<TypeTree>>,
    pub right: Rc<Box<TypeTree>>,
    pub curried: Type,
}

#[derive(Debug)]
pub struct UnaryOp {
    pub val: Rc<Box<TypeTree>>,
    pub curried: Type,
}

#[derive(Debug)]
pub struct NoOp {
    pub curried: Type,
}

#[derive(Debug)]
pub struct Invoke {
    pub args: Vec<Rc<Box<TypeTree>>>,
    pub args_curried: Vec<Type>,
    pub ident: Rc<Box<TypeTree>>,
    pub curried: Type,
}

#[derive(Debug)]
pub struct Initialization {
    pub left: String,
    pub right: Rc<Box<TypeTree>>,
    pub curried: Type,
}

#[derive(Debug)]
pub struct Reassignment {
    pub left: Rc<Box<TypeTree>>,
    pub right: Rc<Box<TypeTree>>,
    pub curried: Type,
}

#[derive(Debug)]
pub struct PropAccess {
    pub prev: Rc<Box<TypeTree>>,
    pub ident: String,
    pub curried: Type,
}

#[derive(Debug)]
pub struct SymbolAccess {
    pub ident: String,
    pub curried: Type,
}

#[derive(Debug)]
pub struct ArrayAccess {
    pub prev: Rc<Box<TypeTree>>,
    pub inner: Rc<Box<TypeTree>>,
    pub curried: Type,
}

#[derive(Debug)]
pub struct StructInitialize {
    pub idents: Vec<String>,
    pub vals: Vec<Rc<Box<TypeTree>>>,
    pub vals_curried: Vec<Type>,
    pub curried: Type,
}

#[derive(Debug)]
pub struct ArrayInitialize {
    pub vals: Vec<Rc<Box<TypeTree>>>,
    pub vals_curried: Vec<Type>,
    pub curried: Type,
}

#[derive(Debug)]
pub struct FunctionInitialize {
    pub name: String,
    pub args: Vec<Rc<Box<TypeTree>>>,
    pub args_curried: Vec<Type>,
    pub block: Rc<Box<TypeTree>>,
    pub block_curried: Type,
}

#[derive(Debug)]
pub struct Block {
    pub exprs: Vec<Rc<Box<TypeTree>>>,
    pub curried: Type,
}

#[derive(Debug)]
pub enum TypeTree {
    // info
    StructInfo(StructInfo),
    DeclaratorInfo(DeclaratorInfo),
    TagInfo(TagInfo),
    ErrorInfo(ErrorInfo),
    // flow
    For(ForOp),
    Invoke(Invoke),
    Match(MatchOp),
    Arm(BinaryOp),
    Block(Block),
    Return(UnaryOp),
    ReturnVoid(NoOp),
    Never(NoOp),
    Break(UnaryOp),
    BreakVoid(NoOp),
    // binops
    Plus(BinaryOp),
    Minus(BinaryOp),
    Divide(BinaryOp),
    Multiply(BinaryOp),
    Modulo(BinaryOp),
    Range(BinaryOp),
    CastAs(BinaryOp),
    BubbleUndef(BinaryOp),
    BubbleError(BinaryOp),
    // unops
    ReadBorrow(UnaryOp),
    MutBorrow(UnaryOp),
    Copy(UnaryOp),
    Clone(UnaryOp),
    Negate(UnaryOp),
    Not(UnaryOp),
    // values
    PropAccess(PropAccess),
    SymbolAccess(SymbolAccess),
    RestAccess(NoOp),
    SelfRef(NoOp),
    // data types
    StructInit(StructInitialize),
    PropInit(Initialization),
    ArrayInit(ArrayInitialize),
    FuncInit(FunctionInitialize),
    AnonFuncInit(FunctionInitialize),
    ConstInit(Initialization),
    MutInit(Initialization),
    StringInit(ArrayInitialize),
    // reassignments
    As(Reassignment),
    PlusAs(Reassignment),
    MinusAs(Reassignment),
    MultiplyAs(Reassignment),
    DivideAs(Reassignment),
    ModAs(Reassignment),
    OrAs(Reassignment),
    NotAs(Reassignment),
    XorAs(Reassignment),
    LShiftAs(Reassignment),
    RShiftAs(Reassignment),
    // value types
    UndefinedValue,
    UnknownValue,
    BoolValue(bool),
    I64(i64),
    Char(char),
    I32(i32),
    U64(u64),
    U32(u32),
    F64(f64),
}

impl TypeTree {
    pub fn into_declarator(&self) -> &DeclaratorInfo {
        match self {
            TypeTree::DeclaratorInfo(x) => x,
            _ => panic!("issue declarator not found"),
        }
    }
    pub fn into_symbol_access(&self) -> &SymbolAccess {
        match self {
            TypeTree::SymbolAccess(x) => x,
            _ => panic!("issue declarator not found"),
        }
    }
    pub fn into_prop_init(&self) -> &Initialization {
        match self {
            TypeTree::PropInit(x) => x,
            _ => panic!("issue declarator not found"),
        }
    }
    pub fn into_binary_op(&self) -> &BinaryOp {
        match self {
            TypeTree::Arm(x) => x,
            _ => panic!("issue binary op not found"),
        }
    }
    pub fn whatami(&self) -> &'static str {
        match self {
            TypeTree::StructInfo(_) => "struct declaration",
            TypeTree::DeclaratorInfo(_) => "property declaration",
            TypeTree::TagInfo(_) => "tag declaration",
            TypeTree::ErrorInfo(_) => "error declaration",
            TypeTree::For(_) => "for loop",
            TypeTree::Invoke(_) => "function invocation",
            TypeTree::Match(_) => "match",
            TypeTree::Arm(_) => "pattern match arm",
            TypeTree::Block(_) => "block of statements",
            TypeTree::Return(_) => "return expression",
            TypeTree::ReturnVoid(_) => "return",
            TypeTree::Never(_) => "never",
            TypeTree::Break(_) => "break expression",
            TypeTree::BreakVoid(_) => "break",
            TypeTree::Plus(_) => "addition",
            TypeTree::Minus(_) => "subtraction",
            TypeTree::Divide(_) => "division",
            TypeTree::Multiply(_) => "multiplication",
            TypeTree::Modulo(_) => "modulus",
            TypeTree::Range(_) => "range",
            TypeTree::CastAs(_) => "cast",
            TypeTree::BubbleUndef(_) => "undefinded bubble",
            TypeTree::BubbleError(_) => "error bubble",
            TypeTree::ReadBorrow(_) => "read borrow",
            TypeTree::MutBorrow(_) => "mutable borrow",
            TypeTree::Copy(_) => "unsized copy",
            TypeTree::Clone(_) => "sized clone",
            TypeTree::Negate(_) => "negation",
            TypeTree::Not(_) => "boolean negatation",
            TypeTree::PropAccess(_) => "property access",
            TypeTree::SymbolAccess(_) => "symbol reference",
            TypeTree::RestAccess(_) => "rest access",
            TypeTree::SelfRef(_) => "self reference",
            TypeTree::StructInit(_) => "struct initialization",
            TypeTree::PropInit(_) => "property assignment",
            TypeTree::ArrayInit(_) => "array initialization",
            TypeTree::FuncInit(_) => "function initialization",
            TypeTree::AnonFuncInit(_) => "anonymous function initialization",
            TypeTree::ConstInit(_) => "constant initialization",
            TypeTree::MutInit(_) => "mutable initialization",
            TypeTree::StringInit(_) => "string initialization",
            TypeTree::As(_) => "reassignment",
            TypeTree::PlusAs(_) => "addition reassignment",
            TypeTree::MinusAs(_) => "subtraction reassignment",
            TypeTree::MultiplyAs(_) => "multiplication reassignment",
            TypeTree::DivideAs(_) => "division reassignment",
            TypeTree::ModAs(_) => "modulus reassignemnt",
            TypeTree::OrAs(_) => "or reassignment",
            TypeTree::NotAs(_) => "logical not reassignment",
            TypeTree::XorAs(_) => "xor reassignment",
            TypeTree::LShiftAs(_) => "left shift reassignment",
            TypeTree::RShiftAs(_) => "right shift reassignment",
            TypeTree::UndefinedValue => "undefined",
            TypeTree::BoolValue(_) => "boolean value",
            TypeTree::I64(_) => "integer 64 bit",
            TypeTree::I32(_) => "integer 32 bit",
            TypeTree::U64(_) => "unsigned integer 64 bit",
            TypeTree::U32(_) => "unsigned integer 32 bit",
            TypeTree::F64(_) => "floating point double precision 64 bit",
            TypeTree::Char(_) => "ascii character",
            TypeTree::UnknownValue => "unknown value",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Type {
    I64,
    I32,
    U64,
    U32,
    F64,
    Unknown,
    Rest,
    Undefined,
    ErrorDecl,
    Void,
    Never,
    Bool,
    Char,
    String,
    MutBorrow(Box<Type>),
    ReadBorrow(Box<Type>),
    Frame(Vec<Type>),
    Error(Box<Type>),
    Struct(Vec<Type>),
    Tag(Vec<Type>),
    Function(Vec<Type>, Box<Type>),
    Custom(String),
    Array(Box<Type>),
    Multi(Vec<Type>),
}

#[macro_export]
macro_rules! simple_tree {
    ($val:ident) => {
        Rc::new(Box::new(TypeTree::$val))
    };
}

#[macro_export]
macro_rules! ok_simple_tree {
    ($val:ident, $curried:ident) => {
        Ok((Rc::new(Box::new(TypeTree::$val)), $curried))
    };
}

#[macro_export]
macro_rules! ok_tree {
    ($val:ident, $op:ident, $curried:ident) => {
        Ok((Rc::new(Box::new(TypeTree::$val($op))), $curried))
    };
}

#[macro_export]
macro_rules! tree {
    ($val:ident, $op:ident) => {
        Rc::new(Box::new(TypeTree::$val($op)))
    };
}
