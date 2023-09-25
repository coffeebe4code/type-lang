use lexer::Lexeme;

#[derive(Debug, Clone, PartialEq)]
pub struct AsDef {
    pub mutability: Lexeme,
    pub identifier: Box<Expr>,
    pub expr: Box<Expr>,
}

impl AsDef {
    pub fn new(mutability: Lexeme, identifier: Box<Expr>, expr: Box<Expr>) -> Self {
        AsDef {
            mutability,
            identifier,
            expr,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct StructDef {
    pub visibility: Option<Lexeme>,
    pub mutability: Lexeme,
    pub identifier: Box<Expr>,
    pub declarators: Option<Vec<Box<Expr>>>,
}

impl StructDef {
    pub fn new(
        visibility: Option<Lexeme>,
        mutability: Lexeme,
        identifier: Box<Expr>,
        declarators: Option<Vec<Box<Expr>>>,
    ) -> Self {
        StructDef {
            visibility,
            mutability,
            identifier,
            declarators,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct FuncDef {
    pub visibility: Option<Lexeme>,
    pub mutability: Lexeme,
    pub identifier: Box<Expr>,
    pub args: Option<Vec<Box<Expr>>>,
    pub block: Box<Expr>,
}

impl FuncDef {
    pub fn new(
        visibility: Option<Lexeme>,
        mutability: Lexeme,
        identifier: Box<Expr>,
        args: Option<Vec<Box<Expr>>>,
        block: Box<Expr>,
    ) -> Self {
        FuncDef {
            visibility,
            mutability,
            identifier,
            args,
            block,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ArgsDef {
    pub args: Vec<Box<Expr>>,
}

impl ArgsDef {
    pub fn new(args: Vec<Box<Expr>>) -> Self {
        ArgsDef { args }
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
    Symbol(Symbol),
    TypeSimple(TypeSimple),
    ArgsDef(ArgsDef),
    FuncDef(FuncDef),
    StructDef(StructDef),
    AsDef(AsDef),
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
