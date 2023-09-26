use lexer::Lexeme;

#[derive(Debug, Clone, PartialEq)]
pub struct Import {
    pub mutability: Lexeme,
    pub identifier: Box<Expr>,
    pub expr: Box<Expr>,
}

impl Import {
    pub fn new(mutability: Lexeme, identifier: Box<Expr>, expr: Box<Expr>) -> Self {
        Import {
            mutability,
            identifier,
            expr,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct InnerDecl {
    pub mutability: Lexeme,
    pub identifier: Box<Expr>,
    pub expr: Box<Expr>,
}

impl InnerDecl {
    pub fn new(mutability: Lexeme, identifier: Box<Expr>, expr: Box<Expr>) -> Self {
        InnerDecl {
            mutability,
            identifier,
            expr,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct UnionDecl {
    pub visibility: Option<Lexeme>,
    pub mutability: Lexeme,
    pub identifier: Box<Expr>,
    pub declarators: Option<Vec<Box<Expr>>>,
}

impl UnionDecl {
    pub fn new(
        visibility: Option<Lexeme>,
        mutability: Lexeme,
        identifier: Box<Expr>,
        declarators: Option<Vec<Box<Expr>>>,
    ) -> Self {
        UnionDecl {
            visibility,
            mutability,
            identifier,
            declarators,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct EnumDecl {
    pub visibility: Option<Lexeme>,
    pub mutability: Lexeme,
    pub identifier: Box<Expr>,
    pub declarators: Option<Vec<Box<Expr>>>,
}

impl EnumDecl {
    pub fn new(
        visibility: Option<Lexeme>,
        mutability: Lexeme,
        identifier: Box<Expr>,
        declarators: Option<Vec<Box<Expr>>>,
    ) -> Self {
        EnumDecl {
            visibility,
            mutability,
            identifier,
            declarators,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TagDecl {
    pub visibility: Option<Lexeme>,
    pub mutability: Lexeme,
    pub identifier: Box<Expr>,
    pub declarators: Option<Vec<Box<Expr>>>,
}

impl TagDecl {
    pub fn new(
        visibility: Option<Lexeme>,
        mutability: Lexeme,
        identifier: Box<Expr>,
        declarators: Option<Vec<Box<Expr>>>,
    ) -> Self {
        TagDecl {
            visibility,
            mutability,
            identifier,
            declarators,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct StructDecl {
    pub visibility: Option<Lexeme>,
    pub mutability: Lexeme,
    pub identifier: Box<Expr>,
    pub declarators: Option<Vec<Box<Expr>>>,
}

impl StructDecl {
    pub fn new(
        visibility: Option<Lexeme>,
        mutability: Lexeme,
        identifier: Box<Expr>,
        declarators: Option<Vec<Box<Expr>>>,
    ) -> Self {
        StructDecl {
            visibility,
            mutability,
            identifier,
            declarators,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct MacroDecl {
    pub visibility: Option<Lexeme>,
    pub mutability: Lexeme,
    pub identifier: Box<Expr>,
    pub args: Option<Vec<Box<Expr>>>,
    pub block: Box<Expr>,
}

impl MacroDecl {
    pub fn new(
        visibility: Option<Lexeme>,
        mutability: Lexeme,
        identifier: Box<Expr>,
        args: Option<Vec<Box<Expr>>>,
        block: Box<Expr>,
    ) -> Self {
        MacroDecl {
            visibility,
            mutability,
            identifier,
            args,
            block,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct FuncDecl {
    pub visibility: Option<Lexeme>,
    pub mutability: Lexeme,
    pub identifier: Box<Expr>,
    pub args: Option<Vec<Box<Expr>>>,
    pub block: Box<Expr>,
}

impl FuncDecl {
    pub fn new(
        visibility: Option<Lexeme>,
        mutability: Lexeme,
        identifier: Box<Expr>,
        args: Option<Vec<Box<Expr>>>,
        block: Box<Expr>,
    ) -> Self {
        FuncDecl {
            visibility,
            mutability,
            identifier,
            args,
            block,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ArgsDecl {
    pub args: Vec<Box<Expr>>,
}

impl ArgsDecl {
    pub fn new(args: Vec<Box<Expr>>) -> Self {
        ArgsDecl { args }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TypeSimple {
    pub span: Lexeme,
}

impl TypeSimple {
    pub fn new(span: Lexeme) -> Self {
        TypeSimple { span }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Block {
    pub exprs: Vec<Box<Expr>>,
}
impl Block {
    pub fn new(exprs: Vec<Box<Expr>>) -> Self {
        Block { exprs }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct RetOp {
    pub span: Lexeme,
    pub expr: Box<Expr>,
}
impl RetOp {
    pub fn new(span: Lexeme, expr: Box<Expr>) -> Self {
        RetOp { span, expr }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct UnOp {
    pub op: Lexeme,
    pub val: Box<Expr>,
}
impl UnOp {
    pub fn new(op: Lexeme, val: Box<Expr>) -> Self {
        UnOp { op, val }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Symbol {
    pub val: Lexeme,
}

impl Symbol {
    pub fn new(val: Lexeme) -> Self {
        Symbol { val }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Never {
    pub val: Lexeme,
}

impl Never {
    pub fn new(val: Lexeme) -> Self {
        Never { val }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SelfValue {
    pub val: Lexeme,
}

impl SelfValue {
    pub fn new(val: Lexeme) -> Self {
        SelfValue { val }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct UndefinedValue {
    pub val: Lexeme,
}

impl UndefinedValue {
    pub fn new(val: Lexeme) -> Self {
        UndefinedValue { val }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct BoolValue {
    pub val: Lexeme,
}

impl BoolValue {
    pub fn new(val: Lexeme) -> Self {
        BoolValue { val }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct CharsValue {
    pub val: Lexeme,
}

impl CharsValue {
    pub fn new(val: Lexeme) -> Self {
        CharsValue { val }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Number {
    pub val: Lexeme,
}

impl Number {
    pub fn new(val: Lexeme) -> Self {
        Number { val }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct BinOp {
    pub left: Box<Expr>,
    pub op: Lexeme,
    pub right: Box<Expr>,
}

impl BinOp {
    pub fn new(left: Box<Expr>, op: Lexeme, right: Box<Expr>) -> Self {
        BinOp { left, op, right }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    RetOp(RetOp),
    Block(Block),
    BinOp(BinOp),
    UnOp(UnOp),
    Number(Number),
    CharsValue(CharsValue),
    BoolValue(BoolValue),
    Symbol(Symbol),
    TypeSimple(TypeSimple),
    ArgsDecl(ArgsDecl),
    FuncDecl(FuncDecl),
    MacroDecl(MacroDecl),
    UnionDecl(UnionDecl),
    EnumDecl(EnumDecl),
    StructDecl(StructDecl),
    TagDecl(TagDecl),
    InnerDecl(InnerDecl),
    Import(Import),
    UndefinedValue(UndefinedValue),
    SelfValue(SelfValue),
    Never(Never),
}

impl Expr {
    pub fn into_symbol(&self) -> Symbol {
        match self {
            Expr::Symbol(x) => x.to_owned(),
            _ => panic!("issue no symbol found"),
        }
    }
}

#[macro_export]
macro_rules! expr {
    ($val:ident, $($inner:tt)*) => {
        Box::new(Expr::$val($val::new($($inner)*)))
    };
}

#[macro_export]
macro_rules! result_expr {
    ($val:ident, $($inner:tt)*) => {
        Ok(Box::new(Expr::$val($val::new($($inner)*))))
    };
}

#[macro_export]
macro_rules! bubble_expr {
    ($val:ident, $($inner:tt)*) => {
        Ok(Some(Box::new(Expr::$val($val::new($($inner)*)))))
    };
}

#[macro_export]
macro_rules! opt_expr {
    ($val:ident, $($inner:tt)*) => {
        Some(Box::new(Expr::$val($val::new($($inner)*))))
    };
}
