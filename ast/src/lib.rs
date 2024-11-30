use lexer::Lexeme;

#[derive(Debug, Clone, PartialEq)]
pub struct For {
    pub expr: Box<Expr>,
    pub var_loop: Box<Expr>,
}

impl For {
    pub fn new(expr: Box<Expr>, var_loop: Box<Expr>) -> Self {
        For { expr, var_loop }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Match {
    pub expr: Box<Expr>,
    pub arms: Vec<Box<Expr>>,
}

impl Match {
    pub fn new(expr: Box<Expr>, arms: Vec<Box<Expr>>) -> Self {
        Match { expr, arms }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Declarator {
    pub ident: Box<Expr>,
    pub typ: Box<Expr>,
}

impl Declarator {
    pub fn new(ident: Box<Expr>, typ: Box<Expr>) -> Self {
        Declarator { ident, typ }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Arm {
    pub left: Box<Expr>,
    pub right: Box<Expr>,
}

impl Arm {
    pub fn new(left: Box<Expr>, right: Box<Expr>) -> Self {
        Arm { left, right }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct FileAll {
    pub top_decls: Vec<Box<Expr>>,
}

impl FileAll {
    pub fn new(top_decls: Vec<Box<Expr>>) -> Self {
        FileAll { top_decls }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct UndefBubble {
    pub prev: Box<Expr>,
}

impl UndefBubble {
    pub fn new(prev: Box<Expr>) -> Self {
        UndefBubble { prev }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct PropAssignment {
    pub ident: Box<Expr>,
    pub val: Box<Expr>,
}

impl PropAssignment {
    pub fn new(ident: Box<Expr>, val: Box<Expr>) -> Self {
        PropAssignment { ident, val }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct PropAssignments {
    pub prev: Box<Expr>,
    pub props: Option<Vec<Box<Expr>>>,
}

impl PropAssignments {
    pub fn new(prev: Box<Expr>, props: Option<Vec<Box<Expr>>>) -> Self {
        PropAssignments { prev, props }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ArrayDecl {
    pub args: Option<Vec<Box<Expr>>>,
}

impl ArrayDecl {
    pub fn new(args: Option<Vec<Box<Expr>>>) -> Self {
        ArrayDecl { args }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct PropAccess {
    pub prev: Box<Expr>,
    pub identifier: Box<Expr>,
}

impl PropAccess {
    pub fn new(prev: Box<Expr>, identifier: Box<Expr>) -> Self {
        PropAccess { prev, identifier }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ArgDef {
    pub ident: Box<Expr>,
    pub typ: Box<Expr>,
}
impl ArgDef {
    pub fn new(ident: Box<Expr>, typ: Box<Expr>) -> Self {
        ArgDef { ident, typ }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ErrBubble {
    pub prev: Box<Expr>,
}

impl ErrBubble {
    pub fn new(prev: Box<Expr>) -> Self {
        ErrBubble { prev }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Invoke {
    pub prev: Box<Expr>,
    pub args: Option<Vec<Box<Expr>>>,
}

impl Invoke {
    pub fn new(prev: Box<Expr>, args: Option<Vec<Box<Expr>>>) -> Self {
        Invoke { prev, args }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct StructureAccess {
    pub prev: Box<Expr>,
    pub inner: Option<Vec<Box<Expr>>>,
}

impl StructureAccess {
    pub fn new(prev: Box<Expr>, inner: Option<Vec<Box<Expr>>>) -> Self {
        StructureAccess { prev, inner }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ArrayAccess {
    pub inner: Box<Expr>,
    pub prev: Box<Expr>,
}

impl ArrayAccess {
    pub fn new(inner: Box<Expr>, prev: Box<Expr>) -> Self {
        ArrayAccess { inner, prev }
    }
}

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
pub struct Reassignment {
    pub left: Box<Expr>,
    pub expr: Box<Expr>,
    pub op: Lexeme,
}

impl Reassignment {
    pub fn new(left: Box<Expr>, expr: Box<Expr>, op: Lexeme) -> Self {
        Reassignment { left, expr, op }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TopDecl {
    pub visibility: Option<Lexeme>,
    pub mutability: Lexeme,
    pub identifier: Box<Expr>,
    pub typ: Option<Box<Expr>>,
    pub expr: Box<Expr>,
}

impl TopDecl {
    pub fn new(
        visibility: Option<Lexeme>,
        mutability: Lexeme,
        identifier: Box<Expr>,
        typ: Option<Box<Expr>>,
        expr: Box<Expr>,
    ) -> Self {
        TopDecl {
            visibility,
            mutability,
            identifier,
            typ,
            expr,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct InnerDecl {
    pub mutability: Lexeme,
    pub identifier: Box<Expr>,
    pub typ: Option<Box<Expr>>,
    pub expr: Box<Expr>,
}

impl InnerDecl {
    pub fn new(
        mutability: Lexeme,
        identifier: Box<Expr>,
        typ: Option<Box<Expr>>,
        expr: Box<Expr>,
    ) -> Self {
        InnerDecl {
            mutability,
            identifier,
            typ,
            expr,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TagDecl {
    pub visibility: Option<Lexeme>,
    pub mutability: Lexeme,
    pub identifier: Box<Expr>,
    pub declarators: Vec<Box<Expr>>,
    pub sig: Option<Box<Expr>>,
}

impl TagDecl {
    pub fn new(
        visibility: Option<Lexeme>,
        mutability: Lexeme,
        identifier: Box<Expr>,
        declarators: Vec<Box<Expr>>,
        sig: Option<Box<Expr>>,
    ) -> Self {
        TagDecl {
            visibility,
            mutability,
            identifier,
            declarators,
            sig,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct EnumDecl {
    pub visibility: Option<Lexeme>,
    pub mutability: Lexeme,
    pub identifier: Box<Expr>,
    pub declarators: Vec<Box<Expr>>,
    pub variant: Option<Box<Expr>>,
}

impl EnumDecl {
    pub fn new(
        visibility: Option<Lexeme>,
        mutability: Lexeme,
        identifier: Box<Expr>,
        declarators: Vec<Box<Expr>>,
        variant: Option<Box<Expr>>,
    ) -> Self {
        EnumDecl {
            visibility,
            mutability,
            identifier,
            declarators,
            variant,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ErrorDecl {
    pub visibility: Option<Lexeme>,
    pub mutability: Lexeme,
    pub identifier: Box<Expr>,
    pub sig: Option<Box<Expr>>,
}

impl ErrorDecl {
    pub fn new(
        visibility: Option<Lexeme>,
        mutability: Lexeme,
        identifier: Box<Expr>,
        sig: Option<Box<Expr>>,
    ) -> Self {
        ErrorDecl {
            visibility,
            mutability,
            identifier,
            sig,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct StructDecl {
    pub visibility: Option<Lexeme>,
    pub mutability: Lexeme,
    pub identifier: Box<Expr>,
    pub declarators: Option<Vec<Box<Expr>>>,
    pub sig: Option<Box<Expr>>,
}

impl StructDecl {
    pub fn new(
        visibility: Option<Lexeme>,
        mutability: Lexeme,
        identifier: Box<Expr>,
        declarators: Option<Vec<Box<Expr>>>,
        sig: Option<Box<Expr>>,
    ) -> Self {
        StructDecl {
            visibility,
            mutability,
            identifier,
            declarators,
            sig,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TraitDecl {
    pub visibility: Option<Lexeme>,
    pub mutability: Lexeme,
    pub identifier: Box<Expr>,
    pub args: Option<Vec<Box<Expr>>>,
    pub block: Option<Box<Expr>>,
    pub sig: Option<Box<Expr>>,
}

impl TraitDecl {
    pub fn new(
        visibility: Option<Lexeme>,
        mutability: Lexeme,
        identifier: Box<Expr>,
        args: Option<Vec<Box<Expr>>>,
        block: Option<Box<Expr>>,
        sig: Option<Box<Expr>>,
    ) -> Self {
        TraitDecl {
            visibility,
            mutability,
            identifier,
            args,
            block,
            sig,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct AnonFuncDecl {
    pub args: Option<Vec<Box<Expr>>>,
    pub ret_typ: Box<Expr>,
    pub block: Box<Expr>,
}

impl AnonFuncDecl {
    pub fn new(args: Option<Vec<Box<Expr>>>, ret_typ: Box<Expr>, block: Box<Expr>) -> Self {
        AnonFuncDecl {
            args,
            ret_typ,
            block,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct FuncType {
    // this can be an empty vec right now
    pub args: Option<Vec<Box<Expr>>>,
    pub ret_typ: Box<Expr>,
}

impl FuncType {
    pub fn new(args: Option<Vec<Box<Expr>>>, ret_typ: Box<Expr>) -> Self {
        FuncType { args, ret_typ }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct FuncDecl {
    pub visibility: Option<Lexeme>,
    pub mutability: Lexeme,
    pub identifier: Box<Expr>,
    pub args: Option<Vec<Box<Expr>>>,
    pub ret_typ: Box<Expr>,
    pub block: Box<Expr>,
    pub sig: Option<Box<Expr>>,
}

impl FuncDecl {
    pub fn new(
        visibility: Option<Lexeme>,
        mutability: Lexeme,
        identifier: Box<Expr>,
        args: Option<Vec<Box<Expr>>>,
        ret_typ: Box<Expr>,
        block: Box<Expr>,
        sig: Option<Box<Expr>>,
    ) -> Self {
        FuncDecl {
            visibility,
            mutability,
            identifier,
            args,
            ret_typ,
            block,
            sig,
        }
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
pub struct Rest {
    pub val: Lexeme,
}

impl Rest {
    pub fn new(val: Lexeme) -> Self {
        Rest { val }
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
pub struct NeverKeyword {
    pub val: Lexeme,
}

impl NeverKeyword {
    pub fn new(val: Lexeme) -> Self {
        NeverKeyword { val }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SelfKeyword {
    pub val: Lexeme,
}

impl SelfKeyword {
    pub fn new(val: Lexeme) -> Self {
        SelfKeyword { val }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct UndefinedKeyword {
    pub val: Lexeme,
}

impl UndefinedKeyword {
    pub fn new(val: Lexeme) -> Self {
        UndefinedKeyword { val }
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
pub struct ArrayType {
    pub arr_of: Box<Expr>,
    pub sized: Option<Box<Expr>>,
}

impl ArrayType {
    pub fn new(arr_of: Box<Expr>, sized: Option<Box<Expr>>) -> Self {
        ArrayType { arr_of, sized }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ValueType {
    pub val: Lexeme,
}

impl ValueType {
    pub fn new(val: Lexeme) -> Self {
        ValueType { val }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Sig {
    // look at parser to see how this is implemented
    pub left_most_type: Option<Box<Expr>>,
    pub err: Option<Lexeme>,
    pub undef: Option<Lexeme>,
    pub right_most_type: Option<Box<Expr>>,
}

impl Sig {
    pub fn new(
        left_most_ident: Option<Box<Expr>>,
        err: Option<Lexeme>,
        undef: Option<Lexeme>,
        right_most_ident: Option<Box<Expr>>,
    ) -> Self {
        Sig {
            left_most_type: left_most_ident,
            err,
            undef,
            right_most_type: right_most_ident,
        }
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
    PropAssignments(PropAssignments),
    PropAssignment(PropAssignment),
    Declarator(Declarator),
    Sig(Sig),
    ArrayDecl(ArrayDecl),
    Rest(Rest),
    For(For),
    Match(Match),
    Arm(Arm),
    FileAll(FileAll),
    RetOp(RetOp),
    Block(Block),
    BinOp(BinOp),
    UnOp(UnOp),
    Number(Number),
    CharsValue(CharsValue),
    ArrayType(ArrayType),
    ValueType(ValueType),
    FuncType(FuncType),
    ArgDef(ArgDef),
    BoolValue(BoolValue),
    Symbol(Symbol),
    SymbolDecl(Symbol),
    AnonFuncDecl(AnonFuncDecl),
    FuncDecl(FuncDecl),
    SelfDecl(SelfKeyword),
    TraitDecl(TraitDecl),
    StructDecl(StructDecl),
    ErrorDecl(ErrorDecl),
    TagDecl(TagDecl),
    EnumDecl(EnumDecl),
    InnerDecl(InnerDecl),
    TopDecl(TopDecl),
    Reassignment(Reassignment),
    Import(Import),
    UndefinedValue(UndefinedKeyword),
    SelfValue(SelfKeyword),
    Never(NeverKeyword),
    ArrayAccess(ArrayAccess),
    PropAccess(PropAccess),
    Invoke(Invoke),
    ErrBubble(ErrBubble),
    UndefBubble(UndefBubble),
}

impl Expr {
    pub fn into_file_all(&self) -> FileAll {
        match self {
            Expr::FileAll(x) => x.to_owned(),
            _ => panic!("issue no symbol found"),
        }
    }
    pub fn into_self(&self) -> SelfKeyword {
        match self {
            Expr::SelfValue(x) => x.to_owned(),
            Expr::SelfDecl(x) => x.to_owned(),
            _ => panic!("issue no self keyword found"),
        }
    }
    pub fn is_self_val(&self) -> bool {
        match self {
            Expr::SelfValue(_) => true,
            _ => false,
        }
    }
    pub fn into_symbol(&self) -> Symbol {
        match self {
            Expr::Symbol(x) => x.to_owned(),
            Expr::SymbolDecl(x) => x.to_owned(),
            _ => panic!("issue no symbol found {:?}", self),
        }
    }
    pub fn into_arg_def(&self) -> ArgDef {
        match self {
            Expr::ArgDef(x) => x.to_owned(),
            _ => panic!("issue no argument definition found"),
        }
    }
    pub fn into_chars_value(&self) -> CharsValue {
        match self {
            Expr::CharsValue(x) => x.to_owned(),
            _ => panic!("issue no symbol found"),
        }
    }
    pub fn into_val_type(&self) -> ValueType {
        match self {
            Expr::ValueType(x) => x.to_owned(),
            _ => panic!("issue no symbol found"),
        }
    }
}

#[macro_export]
macro_rules! expr {
    ($val:ident, $($inner:tt)*) => {
        Box::new(Expr::$val(ast::$val::new($($inner)*)))
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
