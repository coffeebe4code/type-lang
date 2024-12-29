use core::fmt;
use std::rc::Rc;

#[derive(Debug)]
pub struct SigTypes {
    pub left: Ty,
    pub err: Option<Ty>,
    pub undefined: Option<Ty>,
    pub right: Option<Ty>,
}

#[derive(Debug)]
pub struct ArrType {
    pub arr_of: Rc<Box<TypeTree>>,
    pub curried: Ty,
}

#[derive(Debug)]
pub struct SingleType {
    pub right: Ty,
}

#[derive(Debug)]
pub struct ErrorInfo {
    pub message: String,
    pub code: usize,
    pub curried: Ty,
}

#[derive(Debug)]
pub struct TagInfo {
    pub name: String,
    pub props: Vec<Rc<Box<TypeTree>>>,
    pub types: Vec<Ty>,
    pub curried: Ty,
}

#[derive(Debug)]
pub struct EnumInfo {
    pub name: String,
    pub props: Vec<Rc<Box<TypeTree>>>,
    pub curried: Ty,
}

#[derive(Debug)]
pub struct StructInfo {
    pub props: Vec<String>,
    pub types: Vec<Ty>,
    pub curried: Ty,
    pub scope: u32,
}

#[derive(Debug)]
pub struct ArgInfo {
    pub curried: Ty,
}

#[derive(Debug)]
pub struct DeclaratorInfo {
    pub name: String,
    pub curried: Ty,
}

#[derive(Debug)]
pub struct MatchOp {
    pub expr: Rc<Box<TypeTree>>,
    pub curried: Ty,
    pub arms: Vec<Rc<Box<TypeTree>>>,
    pub curried_arms: Ty,
}

#[derive(Debug)]
pub struct ForOp {
    pub in_expr: Rc<Box<TypeTree>>,
    pub in_curried: Ty,
    pub body: Rc<Box<TypeTree>>,
    pub body_curried: Ty,
}

#[derive(Debug)]
pub struct WhileOp {
    pub expr: Rc<Box<TypeTree>>,
    pub expr_curried: Ty,
    pub var_loop: Rc<Box<TypeTree>>,
    pub var_curried: Ty,
}

#[derive(Debug)]
pub struct IfOp {
    pub in_expr: Rc<Box<TypeTree>>,
    pub in_curried: Ty,
    pub body: Rc<Box<TypeTree>>,
    pub body_curried: Ty,
}

#[derive(Debug)]
pub struct BinaryOp {
    pub left: Rc<Box<TypeTree>>,
    pub right: Rc<Box<TypeTree>>,
    pub curried: Ty,
}

#[derive(Debug)]
pub struct UnaryOp {
    pub val: Rc<Box<TypeTree>>,
    pub curried: Ty,
}

#[derive(Debug)]
pub struct NoOp {
    pub curried: Ty,
}

#[derive(Debug)]
pub struct Invoke {
    pub args: Vec<Rc<Box<TypeTree>>>,
    pub args_curried: Vec<Ty>,
    pub ident: Rc<Box<TypeTree>>,
    pub curried: Ty,
}

#[derive(Debug)]
pub struct Initialization {
    pub left: Rc<Box<TypeTree>>,
    pub right: Rc<Box<TypeTree>>,
    pub curried: Ty,
}

#[derive(Debug)]
pub struct TopInitialization {
    pub left: Rc<Box<TypeTree>>,
    pub right: Rc<Box<TypeTree>>,
    pub curried: Ty,
    pub vis: bool,
}

#[derive(Debug)]
pub struct Reassignment {
    pub left: Rc<Box<TypeTree>>,
    pub right: Rc<Box<TypeTree>>,
    pub curried: Ty,
}

#[derive(Debug)]
pub struct PropAccess {
    pub prev: Rc<Box<TypeTree>>,
    pub ident: String,
    pub curried: Ty,
}

#[derive(Debug)]
pub struct SymbolInit {
    pub ident: String,
    pub curried: Ty,
}

#[derive(Debug)]
pub struct SymbolAccess {
    pub ident: String,
    pub curried: Ty,
}

#[derive(Debug)]
pub struct ArrayAccess {
    pub prev: Rc<Box<TypeTree>>,
    pub inner: Rc<Box<TypeTree>>,
    pub curried: Ty,
}

#[derive(Debug)]
pub struct StructInitialize {
    pub idents: Vec<Rc<Box<TypeTree>>>,
    pub vals: Vec<Rc<Box<TypeTree>>>,
    pub vals_curried: Vec<Ty>,
    pub curried: Ty,
}

#[derive(Debug)]
pub struct ArrayInitialize {
    pub vals: Vec<Rc<Box<TypeTree>>>,
    pub vals_curried: Vec<Ty>,
    pub curried: Ty,
}

#[derive(Debug)]
pub struct FunctionInitialize {
    pub name: String,
    pub args: Vec<Rc<Box<TypeTree>>>,
    pub args_curried: Vec<Ty>,
    pub block: Rc<Box<TypeTree>>,
    pub block_curried: Ty,
}

#[derive(Debug)]
pub struct Block {
    pub exprs: Vec<Rc<Box<TypeTree>>>,
    pub curried: Ty,
}

#[derive(Debug)]
pub enum TypeTree {
    // info
    StructInfo(StructInfo),
    DeclaratorInfo(DeclaratorInfo),
    TagInfo(TagInfo),
    EnumInfo(EnumInfo),
    ErrorInfo(ErrorInfo),
    // flow
    For(ForOp),
    If(IfOp),
    Invoke(Invoke),
    Match(MatchOp),
    Arm(BinaryOp),
    While(WhileOp),
    Block(Block),
    Return(UnaryOp),
    ReturnVoid(NoOp),
    Never(NoOp),
    Break(UnaryOp),
    BreakVoid(NoOp),
    // binops
    Plus(BinaryOp),
    NotEq(BinaryOp),
    Eq(BinaryOp),
    OrLog(BinaryOp),
    Minus(BinaryOp),
    Divide(BinaryOp),
    Multiply(BinaryOp),
    Modulo(BinaryOp),
    Range(BinaryOp),
    CastAs(BinaryOp),
    Gt(BinaryOp),
    // unops
    BubbleUndef(UnaryOp),
    BubbleError(UnaryOp),
    ReadBorrow(UnaryOp),
    MutBorrow(UnaryOp),
    Copy(UnaryOp),
    Clone(UnaryOp),
    Negate(UnaryOp),
    Not(UnaryOp),
    // values
    PropAccess(PropAccess),
    ArrayAccess(ArrayAccess),
    SymbolAccess(SymbolAccess),
    RestAccess(NoOp),
    SelfAccess(NoOp),
    // data types
    ArgInit(SymbolInit),
    SelfInit(NoOp),
    SymbolInit(SymbolInit),
    StructInit(StructInitialize),
    PropInit(Initialization),
    ArrayInit(ArrayInitialize),
    FuncInit(FunctionInitialize),
    AnonFuncInit(FunctionInitialize),
    ConstInit(Initialization),
    TopConstInit(TopInitialization),
    MutInit(Initialization),
    TopMutInit(TopInitialization),
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
    // typereferencing
    SigTypes(SigTypes),
    ValueType(Ty),
    ArrayType(ArrType),
    SingleType(Ty),
}

impl TypeTree {
    pub fn get_curried(&self) -> Ty {
        match self {
            TypeTree::DeclaratorInfo(x) => x.curried.clone(),
            TypeTree::StructInfo(x) => x.curried.clone(),
            TypeTree::TagInfo(x) => x.curried.clone(),
            TypeTree::EnumInfo(x) => x.curried.clone(),
            TypeTree::SigTypes(x) => x.left.clone(),
            TypeTree::ErrorInfo(x) => x.curried.clone(),
            TypeTree::For(x) => x.body_curried.clone(),
            TypeTree::While(x) => x.var_curried.clone(),
            TypeTree::If(x) => x.body_curried.clone(),
            TypeTree::Invoke(x) => x.curried.clone(),
            TypeTree::Match(x) => x.curried_arms.clone(),
            TypeTree::Arm(x) => x.curried.clone(),
            TypeTree::Block(x) => x.curried.clone(),
            TypeTree::Return(x) => x.curried.clone(),
            TypeTree::ReturnVoid(_) => Ty::Void,
            TypeTree::Never(_) => Ty::Never,
            TypeTree::Break(x) => x.curried.clone(),
            TypeTree::BreakVoid(x) => x.curried.clone(),
            TypeTree::Plus(x) => x.curried.clone(),
            TypeTree::Minus(x) => x.curried.clone(),
            TypeTree::Divide(x) => x.curried.clone(),
            TypeTree::Multiply(x) => x.curried.clone(),
            TypeTree::Modulo(x) => x.curried.clone(),
            TypeTree::Range(x) => x.curried.clone(),
            TypeTree::CastAs(x) => x.curried.clone(),
            TypeTree::BubbleUndef(x) => x.curried.clone(),
            TypeTree::BubbleError(x) => x.curried.clone(),
            TypeTree::ReadBorrow(x) => x.curried.clone(),
            TypeTree::MutBorrow(x) => x.curried.clone(),
            TypeTree::Copy(x) => x.curried.clone(),
            TypeTree::Clone(x) => x.curried.clone(),
            TypeTree::Negate(x) => x.curried.clone(),
            TypeTree::Not(x) => x.curried.clone(),
            TypeTree::PropAccess(x) => x.curried.clone(),
            TypeTree::ArrayAccess(x) => x.curried.clone(),
            TypeTree::SymbolAccess(x) => x.curried.clone(),
            TypeTree::RestAccess(x) => x.curried.clone(),
            TypeTree::SelfAccess(x) => x.curried.clone(),
            TypeTree::StructInit(x) => x.curried.clone(),
            TypeTree::PropInit(x) => x.curried.clone(),
            TypeTree::ArrayInit(x) => x.curried.clone(),
            TypeTree::FuncInit(x) => x.block_curried.clone(),
            TypeTree::AnonFuncInit(x) => x.block_curried.clone(),
            TypeTree::ConstInit(x) => x.curried.clone(),
            TypeTree::MutInit(x) => x.curried.clone(),
            TypeTree::TopConstInit(x) => x.curried.clone(),
            TypeTree::TopMutInit(x) => x.curried.clone(),
            TypeTree::StringInit(x) => x.curried.clone(),
            TypeTree::As(x) => x.curried.clone(),
            TypeTree::PlusAs(x) => x.curried.clone(),
            TypeTree::MinusAs(x) => x.curried.clone(),
            TypeTree::MultiplyAs(x) => x.curried.clone(),
            TypeTree::DivideAs(x) => x.curried.clone(),
            TypeTree::ModAs(x) => x.curried.clone(),
            TypeTree::OrAs(x) => x.curried.clone(),
            TypeTree::NotAs(x) => x.curried.clone(),
            TypeTree::XorAs(x) => x.curried.clone(),
            TypeTree::LShiftAs(x) => x.curried.clone(),
            TypeTree::RShiftAs(x) => x.curried.clone(),
            TypeTree::UndefinedValue => Ty::Undefined,
            TypeTree::BoolValue(_) => Ty::Bool,
            TypeTree::I64(_) => Ty::I64,
            TypeTree::I32(_) => Ty::I32,
            TypeTree::U64(_) => Ty::U64,
            TypeTree::U32(_) => Ty::U32,
            TypeTree::F64(_) => Ty::F64,
            TypeTree::Char(_) => Ty::Char,
            TypeTree::UnknownValue => Ty::Unknown,
            TypeTree::ArgInit(x) => x.curried.clone(),
            TypeTree::SelfInit(x) => x.curried.clone(),
            TypeTree::SymbolInit(x) => x.curried.clone(),
            TypeTree::ValueType(x) => x.clone(),
            TypeTree::SingleType(x) => x.clone(),
            TypeTree::ArrayType(x) => x.curried.clone(),
            TypeTree::NotEq(x) => x.curried.clone(),
            TypeTree::Eq(x) => x.curried.clone(),
            TypeTree::OrLog(x) => x.curried.clone(),
            TypeTree::Gt(x) => x.curried.clone(),
        }
    }
    pub fn into_declarator(&self) -> &DeclaratorInfo {
        match self {
            TypeTree::DeclaratorInfo(x) => x,
            _ => panic!("issue declarator not found"),
        }
    }
    pub fn into_func_init(&self) -> &FunctionInitialize {
        match self {
            TypeTree::FuncInit(x) => x,
            TypeTree::AnonFuncInit(x) => x,
            _ => panic!("issue function not found"),
        }
    }
    pub fn into_symbol_init(&self) -> &SymbolInit {
        match self {
            TypeTree::SymbolInit(x) => x,
            _ => panic!("issue symbol not found"),
        }
    }
    pub fn into_symbol_access(&self) -> &SymbolAccess {
        match self {
            TypeTree::SymbolAccess(x) => x,
            _ => panic!("issue symbol not found"),
        }
    }
    pub fn into_child_scope(&self) -> u32 {
        match self {
            TypeTree::StructInfo(x) => x.scope,
            _ => panic!("issue property not found"),
        }
    }
    pub fn into_prop_init(&self) -> &Initialization {
        match self {
            TypeTree::PropInit(x) => x,
            _ => panic!("issue property not found"),
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
            TypeTree::EnumInfo(_) => "enum declaration",
            TypeTree::SigTypes(_) => "type signature",
            TypeTree::ErrorInfo(_) => "error declaration",
            TypeTree::For(_) => "for loop",
            TypeTree::While(_) => "while loop",
            TypeTree::If(_) => "if statement",
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
            TypeTree::BubbleError(_) => "try error bubble",
            TypeTree::ReadBorrow(_) => "read borrow",
            TypeTree::MutBorrow(_) => "mutable borrow",
            TypeTree::Copy(_) => "unsized copy",
            TypeTree::Clone(_) => "sized clone",
            TypeTree::Negate(_) => "negation",
            TypeTree::Not(_) => "boolean negatation",
            TypeTree::PropAccess(_) => "property access",
            TypeTree::ArrayAccess(_) => "array index access",
            TypeTree::SymbolAccess(_) => "symbol reference",
            TypeTree::RestAccess(_) => "rest access",
            TypeTree::SelfAccess(_) => "self reference",
            TypeTree::StructInit(_) => "struct initialization",
            TypeTree::PropInit(_) => "property assignment",
            TypeTree::ArrayInit(_) => "array initialization",
            TypeTree::FuncInit(_) => "function initialization",
            TypeTree::AnonFuncInit(_) => "anonymous function initialization",
            TypeTree::ConstInit(_) => "constant initialization",
            TypeTree::MutInit(_) => "mutable initialization",
            TypeTree::TopConstInit(_) => "constant initialization",
            TypeTree::TopMutInit(_) => "mutable initialization",
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
            TypeTree::ArgInit(_) => "function argument",
            TypeTree::SelfInit(_) => "self as function argument",
            TypeTree::SymbolInit(_) => "symbol definition or initialization",
            TypeTree::ValueType(_) => "a value type",
            TypeTree::SingleType(_) => "a type",
            TypeTree::ArrayType(_) => "an array type",
            TypeTree::NotEq(_) => "not equality check",
            TypeTree::Eq(_) => "equality check",
            TypeTree::OrLog(_) => "or logical check",
            TypeTree::Gt(_) => "greater than logical check",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Ty {
    Any,
    Sized,
    Scalar,
    I64,
    I32,
    ISize,
    U64,
    USize,
    U32,
    U8,
    F64,
    Unknown,
    Rest,
    Undefined,
    Void,
    Never,
    Bool,
    Char,
    String,
    Const(Box<Ty>),
    Mut(Box<Ty>),
    MutBorrow(Box<Ty>),
    ReadBorrow(Box<Ty>),
    Frame(Vec<Ty>),
    Struct(Vec<Ty>),
    Error,
    Tag(Vec<Ty>),
    Enum(Box<Ty>),
    Function(Vec<Ty>, Box<Ty>),
    Custom(String),
    CustomError(String),
    Trait(String),
    TSelf,
    Array(Box<Ty>),
}

impl fmt::Display for Ty {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Ty::Any => write!(f, "any"),
            Ty::Scalar => write!(f, "scalar"),
            Ty::Sized => write!(f, "sized"),
            Ty::I64 => write!(f, "i64"),
            Ty::ISize => write!(f, "isize"),
            Ty::I32 => write!(f, "i32"),
            Ty::U64 => write!(f, "u64"),
            Ty::USize => write!(f, "usize"),
            Ty::U32 => write!(f, "u32"),
            Ty::F64 => write!(f, "f64"),
            Ty::Unknown => write!(f, "unknown"),
            Ty::Rest => write!(f, "_"),
            Ty::Undefined => write!(f, "undefined"),
            Ty::Void => write!(f, "void"),
            Ty::Never => write!(f, "never"),
            Ty::Bool => write!(f, "bool"),
            Ty::Char => write!(f, "char"),
            // might want to just make this an Array
            Ty::String => write!(f, "[char]"),
            Ty::Const(x) => write!(f, "const {}", x),
            Ty::Mut(x) => write!(f, "let {}", x),
            Ty::ReadBorrow(x) => write!(f, "&{}", x),
            Ty::MutBorrow(x) => write!(f, "*{}", x),
            Ty::Frame(x) => {
                write!(f, "frame(").unwrap();
                for a in x {
                    write!(f, "{},", a).unwrap();
                }
                write!(f, ")").unwrap();
                Ok(())
            }
            Ty::Struct(x) => {
                write!(f, "struct {{").unwrap();
                for a in x {
                    write!(f, "{},", a).unwrap();
                }
                write!(f, "}}").unwrap();
                Ok(())
            }
            Ty::Error => write!(f, "error"),
            Ty::CustomError(x) => write!(f, "error {}", x),
            Ty::Tag(x) => {
                write!(f, "tag ").unwrap();
                for a in x {
                    write!(f, "| {}", a).unwrap();
                }
                Ok(())
            }
            Ty::Function(x, y) => {
                write!(f, "function(").unwrap();
                for a in x {
                    write!(f, "{},", a).unwrap();
                }
                write!(f, ") {}", y).unwrap();
                Ok(())
            }
            Ty::Custom(x) => write!(f, "type {}", x),
            Ty::Array(x) => write!(f, "[{}]", x),
            Ty::Trait(x) => write!(f, "trait {}", x),
            Ty::TSelf => write!(f, "self"),
            Ty::U8 => write!(f, "u8"),
            Ty::Enum(x) => write!(f, "enum({})", x),
        }
    }
}

impl Ty {
    pub fn ensure_mut(&self) -> Result<(), Ty> {
        match self {
            Ty::I64 => Err(Ty::I64),
            Ty::I32 => Err(Ty::I32),
            Ty::U64 => Err(Ty::U64),
            Ty::Const(val) => Err(Ty::Const(val.to_owned())),
            Ty::Mut(_) => Ok(()),
            Ty::ReadBorrow(val) => Err(Ty::ReadBorrow(val.to_owned())),
            Ty::MutBorrow(_) => Ok(()),
            Ty::Void => Err(Ty::Void),
            Ty::Error => Ok(()),
            Ty::Unknown => Ok(()),
            Ty::Undefined => Ok(()),
            _ => panic!("type lang issue. type not able to be associated to const"),
        }
    }
    pub fn into_vec(&mut self) -> &mut Vec<Ty> {
        match self {
            Ty::Tag(x) => x,
            _ => panic!("type lang issue. unhandled match arm"),
        }
    }
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
