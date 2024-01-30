use logos::Logos;

#[derive(Logos, Copy, Clone, Debug, PartialEq)]
pub enum Token {
    #[token("error")]
    Error,
    #[token("defer")]
    Defer,
    #[token("errdefer")]
    ErrDefer,
    #[token("any")]
    Any,
    #[token("macro")]
    Macro,
    #[token("import")]
    Import,
    #[token("test")]
    Test,
    #[token("bench")]
    Bench,
    #[token("let")]
    Let,
    #[token("const")]
    Const,
    #[token("copy")]
    WCopy,
    #[token("clone")]
    WClone,
    #[token("scalar")]
    Scalar,
    #[token("sized")]
    WSized,
    #[token("i32")]
    I32,
    #[token("u32")]
    U32,
    #[token("i64")]
    I64,
    #[token("u64")]
    U64,
    #[token("i16")]
    I16,
    #[token("u16")]
    U16,
    #[token("u8")]
    U8,
    #[token("i8")]
    I8,
    #[token("bit")]
    Bit,
    #[token("f64")]
    F64,
    #[token("d64")]
    D64,
    #[token("f32")]
    F32,
    #[token("d32")]
    D32,
    #[token("d128")]
    D128,
    #[token("f128")]
    F128,
    #[token("isize")]
    ISize,
    #[token("usize")]
    USize,
    #[token("if")]
    If,
    #[token("else")]
    Else,
    #[token("type")]
    Type,
    #[token("self")]
    WSelf,
    #[token("undefined")]
    Undefined,
    #[token("char")]
    Char,
    #[token("utf8")]
    Utf8,
    #[token("utf16")]
    Utf16,
    #[token("utf32")]
    Utf32,
    #[token("utf64")]
    Utf64,
    #[token("match")]
    Match,
    #[token("while")]
    While,
    #[token("for")]
    For,
    #[token("of")]
    Of,
    #[token("in")]
    In,
    #[token("break")]
    Break,
    #[token("tag")]
    Tag,
    #[token("pub")]
    Pub,
    #[token("return")]
    Return,
    #[token("await")]
    Await,
    #[token("async")]
    Async,
    #[token("as")]
    CastAs,
    #[token("nosuspend")]
    NoSuspend,
    #[token("suspend")]
    Suspend,
    #[token("resume")]
    Resume,
    #[token("frame")]
    Frame,
    #[token("trait")]
    Trait,
    #[token("impl")]
    Impl,
    #[token("true")]
    True,
    #[token("false")]
    False,
    #[token("void")]
    Void,
    #[token("never")]
    Never,
    #[token("bool")]
    Bool,
    #[token("contract")]
    Contract,
    #[token("comptime")]
    Comptime,
    #[token("fn")]
    Func,
    #[token("struct")]
    Struct,
    #[token("extern")]
    Extern,
    #[token("export")]
    Export,
    #[token("keyof")]
    KeyOf,
    #[token("typeof")]
    TypeOf,
    #[token("rangeof")]
    RangeOf,

    #[token("|>")]
    Split,
    #[token("->")]
    Yield,
    #[token("=>")]
    Arrow,
    #[token("(")]
    OParen,
    #[token(")")]
    CParen,
    #[token("{")]
    OBrace,
    #[token("}")]
    CBrace,
    #[token("[")]
    OArray,
    #[token("]")]
    CArray,

    #[token("..")]
    Range,
    #[token(".")]
    Dot,
    #[token(",")]
    Comma,
    #[token("$")]
    Dollar,
    #[token("?")]
    Question,
    #[token("#")]
    Pound,
    #[token(":")]
    Colon,
    #[token(";")]
    SColon,
    #[token("@")]
    At,
    #[token("<")]
    Lt,
    #[token("<=")]
    LtEq,
    #[token(">")]
    Gt,
    #[token(">=")]
    GtEq,
    #[token("/")]
    Div,
    #[token("\\")]
    BSlash,
    #[token("++")]
    Concat,
    #[token("+")]
    Plus,
    #[token("_")]
    Rest,
    #[token("-")]
    Sub,
    #[token("*")]
    Mul,
    #[token("|")]
    Or,
    #[token("&")]
    And,
    #[token("^")]
    Xor,
    #[token("<<")]
    LShift,
    #[token(">>")]
    RShift,
    #[token("~")]
    Not,
    #[token("=")]
    As,
    #[token("~=")]
    NotAs,
    #[token("|=")]
    OrAs,
    #[token("^=")]
    XorAs,
    #[token("<<=")]
    LShiftAs,
    #[token(">>=")]
    RShiftAs,
    #[token("&&")]
    AndLog,
    #[token("||")]
    OrLog,
    #[token("!=")]
    NotEquality,
    #[token("==")]
    Equality,
    #[token("!")]
    NotLog,
    #[token("%")]
    Mod,
    #[token("+=")]
    AddAs,
    #[token("-=")]
    SubAs,
    #[token("/=")]
    DivAs,
    #[token("*=")]
    MulAs,
    #[token("%=")]
    ModAs,
    #[token("&=")]
    AndAs,

    #[regex(r#"'([^'\\]|\\t|\\u|\\n|\\')*'"#)]
    #[regex(r#""([^"\\]|\\t|\\u|\\n|\\")*""#)]
    Chars,

    #[regex("[1-9][0-9]*\\.[0-9]+|0\\.[0-9]+|0|[1-9][0-9]*")]
    Num,
    #[regex("[a-zA-Z]+[_0-9a-zA-Z]*")]
    Symbol,
    #[regex("[1-9][0-9]*\\.[0-9]+|0\\.[0-9]+")]
    Decimal,

    #[regex(r"//.*", logos::skip)]
    #[regex(r"[ \t\r\f\n]+", logos::skip)]
    Skip,
}

impl Token {
    pub fn is_kind(self, tok: Token) -> bool {
        return tok == self;
    }
    pub fn is_of_kind(self, tokens: &[Token]) -> bool {
        return tokens.iter().any(|t| *t == self);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_tokenizes() {
        let mut lexer = Token::lexer("let x = 5");
        assert_eq!(lexer.next(), Some(Ok(Token::Let)));
        assert_eq!(lexer.next(), Some(Ok(Token::Symbol)));
        assert_eq!(lexer.next(), Some(Ok(Token::As)));
        assert_eq!(lexer.next(), Some(Ok(Token::Num)));
    }
    #[test]
    fn it_tokenizes_rest() {
        let mut lexer = Token::lexer("let _ = x_b");
        assert_eq!(lexer.next(), Some(Ok(Token::Let)));
        assert_eq!(lexer.next(), Some(Ok(Token::Rest)));
        assert_eq!(lexer.next(), Some(Ok(Token::As)));
        assert_eq!(lexer.next(), Some(Ok(Token::Symbol)));
    }
    #[test]
    fn it_tokenizes_declarators() {
        let mut lexer = Token::lexer("wheels: [const char] ");
        assert_eq!(lexer.next(), Some(Ok(Token::Symbol)));
        assert_eq!(lexer.next(), Some(Ok(Token::Colon)));
        assert_eq!(lexer.next(), Some(Ok(Token::OArray)));
        assert_eq!(lexer.next(), Some(Ok(Token::Const)));
        assert_eq!(lexer.next(), Some(Ok(Token::Char)));
        assert_eq!(lexer.next(), Some(Ok(Token::CArray)));
    }
    #[test]
    fn it_tokenizes_nums() {
        let mut lexer1 = Token::lexer("5");
        let mut lexer2 = Token::lexer("50");
        let mut lexer3 = Token::lexer("0");
        assert_eq!(lexer1.next(), Some(Ok(Token::Num)));
        assert_eq!(lexer2.next(), Some(Ok(Token::Num)));
        assert_eq!(lexer3.next(), Some(Ok(Token::Num)));
    }
    #[test]
    fn it_tokenizes_decimals() {
        let mut lexer1 = Token::lexer("5.0");
        let mut lexer2 = Token::lexer("50.0");
        let mut lexer3 = Token::lexer("0.0");
        let mut lexer4 = Token::lexer("0.1");
        let mut lexer5 = Token::lexer(".1");
        let mut lexer6 = Token::lexer("1.");
        let mut lexer7 = Token::lexer("01.2");
        let mut lexer8 = Token::lexer("1.00");
        assert_eq!(lexer1.next(), Some(Ok(Token::Decimal)));
        assert_eq!(lexer2.next(), Some(Ok(Token::Decimal)));
        assert_eq!(lexer3.next(), Some(Ok(Token::Decimal)));
        assert_eq!(lexer4.next(), Some(Ok(Token::Decimal)));
        assert_eq!(lexer5.next(), Some(Ok(Token::Dot)));
        assert_eq!(lexer5.next(), Some(Ok(Token::Num)));
        assert_eq!(lexer6.next(), Some(Ok(Token::Num)));
        assert_eq!(lexer6.next(), Some(Ok(Token::Dot)));
        assert_eq!(lexer7.next(), Some(Ok(Token::Num)));
        assert_eq!(lexer7.next(), Some(Ok(Token::Decimal)));
        assert_eq!(lexer8.next(), Some(Ok(Token::Decimal)));
    }
    #[test]
    fn it_tokenizes_chars() {
        let mut lexer1 = Token::lexer(r#""hello""#);
        assert_eq!(lexer1.next(), Some(Ok(Token::Chars)));
    }
}
