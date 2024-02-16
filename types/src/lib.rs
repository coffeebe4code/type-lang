#[derive(Debug)]
pub struct FileContainer {
    pub top_items: Vec<Box<TypeTree>>,
    pub curried: Vec<Box<Type>>,
}

#[derive(Debug)]
pub struct ErrorInfo {
    pub message: String,
    pub code: usize,
}

#[derive(Debug)]
pub struct TagInfo {
    pub props: Vec<String>,
    pub types: Vec<Box<Type>>,
    pub curried: Box<Type>,
}

#[derive(Debug)]
pub struct StructInfo {
    pub props: Vec<String>,
    pub types: Vec<Box<Type>>,
    pub curried: Box<Type>,
}

#[derive(Debug)]
pub struct ArgInfo {
    pub curried: Box<Type>,
}

#[derive(Debug)]
pub struct MatchOp {
    pub expr: Box<TypeTree>,
    pub curried: Box<Type>,
    pub arms_left: Vec<Box<TypeTree>>,
    pub curried_left: Vec<Box<Type>>,
    pub arms_right: Vec<Box<TypeTree>>,
    pub curried_right: Vec<Box<Type>>,
}

#[derive(Debug)]
pub struct ForOp {
    pub in_expr: Box<TypeTree>,
    pub in_curried: Box<Type>,
    pub body: Box<TypeTree>,
}

#[derive(Debug)]
pub struct BinaryOp {
    pub left: Box<TypeTree>,
    pub right: Box<TypeTree>,
    pub curried: Box<Type>,
}

#[derive(Debug)]
pub struct UnaryOp {
    pub val: Box<TypeTree>,
    pub curried: Box<Type>,
}

#[derive(Debug)]
pub struct NoOp {
    pub curried: Box<Type>,
}

#[derive(Debug)]
pub struct Invoke {
    pub args: Vec<Box<TypeTree>>,
    pub args_curried: Vec<Box<Type>>,
    pub ident: String,
    pub curried: Box<Type>,
}

#[derive(Debug)]
pub struct Initialization {
    pub left: String,
    pub right: Box<TypeTree>,
}

#[derive(Debug)]
pub struct Reassignment {
    pub left: Box<TypeTree>,
    pub right: Box<TypeTree>,
}

#[derive(Debug)]
pub struct PropAccess {
    pub prev: Box<TypeTree>,
    pub ident: String,
    pub curried: Box<Type>,
}

#[derive(Debug)]
pub struct ArrayAccess {
    pub prev: Box<TypeTree>,
    pub inner: Box<TypeTree>,
    pub curried: Box<Type>,
}

#[derive(Debug)]
pub struct StructInitialize {
    pub name: String,
    pub idents: Vec<String>,
    pub vals: Vec<Box<TypeTree>>,
    pub curried: Box<Type>,
}

#[derive(Debug)]
pub struct ArrayInitialize {
    pub name: String,
    pub vals: Vec<Box<TypeTree>>,
    pub curried: Box<Type>,
}

// todo:: import typetree
// todo:: traitinfo typetree

#[derive(Debug)]
pub struct FunctionInitialize {
    pub name: String,
    pub args: Vec<Box<TypeTree>>,
    pub args_curried: Vec<Box<Type>>,
    pub block: Box<TypeTree>,
}

#[derive(Debug)]
pub struct Block {
    pub exprs: Vec<Box<TypeTree>>,
    pub exprs_curried: Vec<Box<Type>>,
}

#[derive(Debug)]
pub enum TypeTree {
    // info
    StructInfo(StructInfo),
    TagInfo(TagInfo),
    ErrorInfo(ErrorInfo),
    // flow
    For(ForOp),
    Match(MatchOp),
    Return(UnaryOp),
    ReturnVoid(NoOp),
    Never(NoOp),
    Break(UnaryOp),
    // binops
    Plus(BinaryOp),
    Minus(BinaryOp),
    Divide(BinaryOp),
    Multiply(BinaryOp),
    Modulo(BinaryOp),
    Range(BinaryOp),
    CastAs(BinaryOp),
    // unops
    MutRef(UnaryOp),
    ConstRef(UnaryOp),
    Copy(UnaryOp),
    BubbleUndef(UnaryOp),
    BubbleError(UnaryOp),
    SelfRef(NoOp),
    // values
    PropAccess(PropAccess),
    RestAccess(NoOp),
    UndefinedValue(NoOp),
    BoolValue(NoOp),
    // data types
    StructInit(StructInitialize),
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
    I64(i64),
    I32(i32),
    U64(u64),
    U32(u32),
    F64(f64),
}

#[derive(Debug, PartialEq, Eq)]
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
    Frame(Vec<Box<Type>>),
    Error(Box<Type>),
    Struct(Vec<Box<Type>>),
    Tag(Vec<Box<Type>>),
    Function(Vec<Box<Type>>, Box<Type>),
    Custom(String),
    Array(Box<Type>),
    Multi(Vec<Box<Type>>),
}
