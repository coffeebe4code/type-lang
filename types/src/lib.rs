#[derive(Debug)]
pub struct FileContainer<'tt> {
    pub top_items: &'tt [TypeTree<'tt>],
    pub curried: Vec<Type>,
}

#[derive(Debug)]
pub struct ErrorInfo {
    pub message: String,
    pub code: usize,
}

#[derive(Debug)]
pub struct TagInfo {
    pub props: Vec<String>,
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
pub struct MatchOp<'tt> {
    pub expr: &'tt TypeTree<'tt>,
    pub curried: Type,
    pub arms_left: &'tt [TypeTree<'tt>],
    pub curried_left: Vec<Type>,
    pub arms_right: &'tt [TypeTree<'tt>],
    pub curried_right: Vec<Type>,
}

#[derive(Debug)]
pub struct ForOp<'tt> {
    pub in_expr: &'tt TypeTree<'tt>,
    pub in_curried: Type,
    pub body: &'tt TypeTree<'tt>,
}

#[derive(Debug)]
pub struct BinaryOp<'tt> {
    pub left: &'tt TypeTree<'tt>,
    pub right: &'tt TypeTree<'tt>,
    pub curried: Type,
}

#[derive(Debug)]
pub struct UnaryOp<'tt> {
    pub val: &'tt TypeTree<'tt>,
    pub curried: Type,
}

#[derive(Debug)]
pub struct NoOp {
    pub curried: Type,
}

#[derive(Debug)]
pub struct Invoke<'tt> {
    pub args: &'tt [TypeTree<'tt>],
    pub args_curried: Vec<Type>,
    pub ident: String,
    pub curried: Type,
}

#[derive(Debug)]
pub struct Initialization<'tt> {
    pub left: String,
    pub right: &'tt TypeTree<'tt>,
    pub curried: Type,
}

#[derive(Debug)]
pub struct Reassignment<'tt> {
    pub left: &'tt TypeTree<'tt>,
    pub right: &'tt TypeTree<'tt>,
}

#[derive(Debug)]
pub struct PropAccess<'tt> {
    pub prev: &'tt TypeTree<'tt>,
    pub ident: String,
    pub curried: Type,
}

#[derive(Debug)]
pub struct SymbolAccess {
    pub ident: String,
    pub curried: Type,
}

#[derive(Debug)]
pub struct ArrayAccess<'tt> {
    pub prev: &'tt TypeTree<'tt>,
    pub inner: &'tt TypeTree<'tt>,
    pub curried: Type,
}

#[derive(Debug)]
pub struct StructInitialize<'tt> {
    pub name: String,
    pub idents: Vec<String>,
    pub vals: &'tt [TypeTree<'tt>],
    pub curried: Type,
}

#[derive(Debug)]
pub struct ArrayInitialize<'tt> {
    pub name: String,
    pub vals: &'tt [TypeTree<'tt>],
    pub curried: Type,
}

// todo:: import typetree
// todo:: traitinfo typetree

#[derive(Debug)]
pub struct FunctionInitialize<'tt> {
    pub name: String,
    pub args: &'tt [TypeTree<'tt>],
    pub args_curried: Vec<Type>,
    pub block: &'tt TypeTree<'tt>,
}

#[derive(Debug)]
pub struct Block<'tt> {
    pub exprs: &'tt [TypeTree<'tt>],
    pub exprs_curried: Vec<Type>,
}

#[derive(Debug)]
pub enum TypeTree<'tt> {
    // info
    StructInfo(StructInfo),
    TagInfo(TagInfo),
    ErrorInfo(ErrorInfo),
    // flow
    For(ForOp<'tt>),
    Match(MatchOp<'tt>),
    Return(UnaryOp<'tt>),
    ReturnVoid(NoOp),
    Never(NoOp),
    Break(UnaryOp<'tt>),
    BreakVoid(NoOp),
    // binops
    Plus(BinaryOp<'tt>),
    Minus(BinaryOp<'tt>),
    Divide(BinaryOp<'tt>),
    Multiply(BinaryOp<'tt>),
    Modulo(BinaryOp<'tt>),
    Range(BinaryOp<'tt>),
    CastAs(BinaryOp<'tt>),
    BubbleUndef(BinaryOp<'tt>),
    BubbleError(BinaryOp<'tt>),
    // unops
    ReadBorrow(UnaryOp<'tt>),
    MutBorrow(UnaryOp<'tt>),
    Copy(UnaryOp<'tt>),
    Clone(UnaryOp<'tt>),
    Negate(UnaryOp<'tt>),
    Not(UnaryOp<'tt>),
    // values
    PropAccess(PropAccess<'tt>),
    SymbolAccess(PropAccess<'tt>),
    RestAccess(NoOp),
    SelfRef(NoOp),
    // data types
    StructInit(StructInitialize<'tt>),
    ArrayInit(ArrayInitialize<'tt>),
    FuncInit(FunctionInitialize<'tt>),
    AnonFuncInit(FunctionInitialize<'tt>),
    ConstInit(Initialization<'tt>),
    MutInit(Initialization<'tt>),
    StringInit(ArrayInitialize<'tt>),
    // reassignments
    As(Reassignment<'tt>),
    PlusAs(Reassignment<'tt>),
    MinusAs(Reassignment<'tt>),
    MultiplyAs(Reassignment<'tt>),
    DivideAs(Reassignment<'tt>),
    ModAs(Reassignment<'tt>),
    OrAs(Reassignment<'tt>),
    NotAs(Reassignment<'tt>),
    XorAs(Reassignment<'tt>),
    LShiftAs(Reassignment<'tt>),
    RShiftAs(Reassignment<'tt>),
    // value types
    UndefinedValue(),
    BoolValue(bool),
    I64(i64),
    I32(i32),
    U64(u64),
    U32(u32),
    F64(f64),
}

impl<'tt> TypeTree<'tt> {
    pub fn into_initialization(&self) -> &Initialization {
        match self {
            TypeTree::ConstInit(x) => x,
            TypeTree::MutInit(x) => x,
            _ => panic!("issue no symbol found"),
        }
    }
    pub fn whatami(&self) -> &'static str {
        match self {
            TypeTree::StructInfo(_) => "struct declaration",
            TypeTree::TagInfo(_) => "tag declaration",
            TypeTree::ErrorInfo(_) => "error declaration",
            TypeTree::For(_) => "for loop",
            TypeTree::Match(_) => "match",
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
            TypeTree::UndefinedValue() => "undefined",
            TypeTree::BoolValue(_) => "boolean value",
            TypeTree::I64(_) => "integer 64 bit",
            TypeTree::I32(_) => "integer 32 bit",
            TypeTree::U64(_) => "unsigned integer 64 bit",
            TypeTree::U32(_) => "unsigned integer 32 bit",
            TypeTree::F64(_) => "floating point double precision 64 bit",
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
    Undefined,
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
macro_rules! tree {
    ($val:ident, $op:ident) => {
        TypeTree::$val($op)
    };
}

#[macro_export]
macro_rules! ok_tree {
    ($val:ident, $op:ident, $curried:ident) => {
        Ok((Box::new(TypeTree::$val($op)), $curried))
    };
}
