#[derive(Debug)]
pub struct BinaryOp {
    pub left: Box<TypeTree>,
    pub right: Box<TypeTree>,
    pub curried: Box<Type>,
}

#[derive(Debug)]
pub struct UnaryOpT {
    pub val: Box<TypeTree>,
    pub curried: Box<Type>,
}

#[derive(Debug)]
pub struct InvokeT {
    pub args: Vec<Box<TypeTree>>,
    pub ident: String,
    pub curried: Box<Type>,
}

#[derive(Debug)]
pub enum TypeTree {
    // binops
    Plus(BinaryOp),
    Minus(BinaryOp),
    Divide(BinaryOp),
    Multiply(BinaryOp),
    Modulo(BinaryOp),
    Range(BinaryOp),
    CastAs(BinaryOp),
    // unops
    MutRef(UnaryOpT),
    ConstRef(UnaryOpT),
    Copy(UnaryOpT),
    BubbleUndef(UnaryOpT),
    BubbleError(UnaryOpT),
    // access
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
    Frame(Vec<Box<Type>>),
    Error(Box<Type>),
    Struct(Vec<Box<Type>>),
    Tag(Vec<Box<Type>>),
    Function(Vec<Box<Type>>, Box<Type>),
    Custom(String),
}
