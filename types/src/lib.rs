#[derive(Debug)]
pub struct FileContainer<'t> {
    pub top_items: &'t [TypeTree<'t>],
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
pub struct MatchOp<'t> {
    pub expr: &'t TypeTree<'t>,
    pub curried: Type,
    pub arms_left: &'t [TypeTree<'t>],
    pub curried_left: Vec<Type>,
    pub arms_right: &'t [TypeTree<'t>],
    pub curried_right: Vec<Type>,
}

#[derive(Debug)]
pub struct ForOp<'t> {
    pub in_expr: &'t TypeTree<'t>,
    pub in_curried: Type,
    pub body: &'t TypeTree<'t>,
}

#[derive(Debug)]
pub struct BinaryOp<'t> {
    pub left: &'t TypeTree<'t>,
    pub right: &'t TypeTree<'t>,
    pub curried: Type,
}

#[derive(Debug)]
pub struct UnaryOp<'t> {
    pub val: &'t TypeTree<'t>,
    pub curried: Type,
}

#[derive(Debug)]
pub struct NoOp {
    pub curried: Type,
}

#[derive(Debug)]
pub struct Invoke<'t> {
    pub args: &'t [TypeTree<'t>],
    pub args_curried: Vec<Type>,
    pub ident: String,
    pub curried: Type,
}

#[derive(Debug)]
pub struct Initialization<'t> {
    pub left: String,
    pub right: &'t TypeTree<'t>,
    pub curried: Type,
}

#[derive(Debug)]
pub struct Reassignment<'t> {
    pub left: &'t TypeTree<'t>,
    pub right: &'t TypeTree<'t>,
}

#[derive(Debug)]
pub struct PropAccess<'t> {
    pub prev: &'t TypeTree<'t>,
    pub ident: String,
    pub curried: Type,
}

#[derive(Debug)]
pub struct SymbolAccess {
    pub ident: String,
    pub curried: Type,
}

#[derive(Debug)]
pub struct ArrayAccess<'t> {
    pub prev: &'t TypeTree<'t>,
    pub inner: &'t TypeTree<'t>,
    pub curried: Type,
}

#[derive(Debug)]
pub struct StructInitialize<'t> {
    pub name: String,
    pub idents: Vec<String>,
    pub vals: &'t [TypeTree<'t>],
    pub curried: Type,
}

#[derive(Debug)]
pub struct ArrayInitialize<'t> {
    pub name: String,
    pub vals: &'t [TypeTree<'t>],
    pub curried: Type,
}

// todo:: import typetree
// todo:: traitinfo typetree

#[derive(Debug)]
pub struct FunctionInitialize<'t> {
    pub name: String,
    pub args: &'t [TypeTree<'t>],
    pub args_curried: Vec<Type>,
    pub block: &'t TypeTree<'t>,
}

#[derive(Debug)]
pub struct Block<'t> {
    pub exprs: &'t [TypeTree<'t>],
    pub exprs_curried: Vec<Type>,
}

#[derive(Debug)]
pub enum TypeTree<'t> {
    // info
    StructInfo(StructInfo),
    TagInfo(TagInfo),
    ErrorInfo(ErrorInfo),
    // flow
    For(ForOp<'t>),
    Match(MatchOp<'t>),
    Return(UnaryOp<'t>),
    ReturnVoid(NoOp),
    Never(NoOp),
    Break(UnaryOp<'t>),
    BreakVoid(NoOp),
    // binops
    Plus(BinaryOp<'t>),
    Minus(BinaryOp<'t>),
    Divide(BinaryOp<'t>),
    Multiply(BinaryOp<'t>),
    Modulo(BinaryOp<'t>),
    Range(BinaryOp<'t>),
    CastAs(BinaryOp<'t>),
    BubbleUndef(BinaryOp<'t>),
    BubbleError(BinaryOp<'t>),
    // unops
    ReadBorrow(UnaryOp<'t>),
    MutBorrow(UnaryOp<'t>),
    Copy(UnaryOp<'t>),
    Clone(UnaryOp<'t>),
    Negate(UnaryOp<'t>),
    Not(UnaryOp<'t>),
    // values
    PropAccess(PropAccess<'t>),
    SymbolAccess(PropAccess<'t>),
    RestAccess(NoOp),
    SelfRef(NoOp),
    // data types
    StructInit(StructInitialize<'t>),
    ArrayInit(ArrayInitialize<'t>),
    FuncInit(FunctionInitialize<'t>),
    AnonFuncInit(FunctionInitialize<'t>),
    ConstInit(Initialization<'t>),
    MutInit(Initialization<'t>),
    StringInit(ArrayInitialize<'t>),
    // reassignments
    As(Reassignment<'t>),
    PlusAs(Reassignment<'t>),
    MinusAs(Reassignment<'t>),
    MultiplyAs(Reassignment<'t>),
    DivideAs(Reassignment<'t>),
    ModAs(Reassignment<'t>),
    OrAs(Reassignment<'t>),
    NotAs(Reassignment<'t>),
    XorAs(Reassignment<'t>),
    LShiftAs(Reassignment<'t>),
    RShiftAs(Reassignment<'t>),
    // value types
    UndefinedValue(),
    BoolValue(bool),
    I64(i64),
    I32(i32),
    U64(u64),
    U32(u32),
    F64(f64),
}

impl<'t> TypeTree<'t> {
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
