const std = @import("std");
const ascii = @import("std").ascii;
const testing = std.testing;

pub const TokenError = error{
    InvalidToken,
};

pub const Token = enum(u8) {
    K_Import,
    K_Use,
    K_Define,
    K_Macro,
    K_Test,
    K_Bench,
    K_Mut,
    K_Let,
    K_Const,
    K_Once,
    K_Local,
    K_Num,
    K_I32,
    K_U32,
    K_U64,
    K_I64,
    K_I16,
    K_U16,
    K_U8,
    K_I8,
    K_Bit,
    K_F64,
    K_F32,
    K_D32,
    K_D64,
    K_If,
    K_Else,
    K_Type,
    K_This,
    K_Null,
    K_Char,
    K_String,
    K_Inline,
    K_Static,
    K_Switch,
    K_For,
    K_In,
    K_Break,
    K_Enum,
    K_Pub,
    K_Return,
    K_Async,
    K_Await,
    K_Box,
    K_Trait,
    K_Ptr,
    K_Match,
    K_Addr,
    K_Vol,
    K_True,
    K_False,
    K_Void,
    K_Iface,
    K_Gen,
    K_Undef,
    K_Never,
    K_Bool,
    K_Byte,
    K_Fn,
    K_Contract,
    K_Queue,
    K_Thread,
    K_Pool,
    K_Observe,
    K_Message,
    K_Block,
    K_Suspend,
    K_Resume,
    K_Export,
    K_Notify,
    K_Not,
    K_And,
    K_Or,
    K_Error,
    K_AnyError,
    K_Extends,
    K_Impl,
    K_Copy,
    K_TypeOf,
    K_Self,
    K_Frame,
    K_Any,
    // non keyword
    OParen,
    CParen,
    OBrace,
    CBrace,
    OArray,
    CArray,
    Dot,
    Comma,
    Dollar,
    Question,
    Pound,
    Colon,
    SColon,
    Backtick,
    At,
    Lt,
    LtEq,
    Gt,
    GtEq,
    Div,
    BSlash,
    Plus,
    Rest,
    Sub,
    Mul,
    Or,
    And,
    Xor,
    LShift,
    RShift,
    Not,
    As,
    NotAs,
    OrAs,
    AndAs,
    XorAs,
    LShiftAs,
    RShiftAs,
    AndLog,
    OrLog,
    NotEquality,
    Equality,
    NotLog,
    Mod,
    Inc,
    Dec,
    AddAs,
    SubAs,
    DivAs,
    MulAs,
    ModAs,
    DQuote,
    SQuote,
    Symbol,
    Num,
    Hex,
    Bin,
    Decimal,
    NewLine,
    Wsp,
    Range,
};

pub fn get_next(buf: []const u8, len: *usize) TokenError!Token {
    len.* = 0;
    const c = buf[0];
    if (ascii.isAlphabetic(c)) {
        return tokenize_chars(buf, len);
    } else if (ascii.isDigit(c)) {
        return tokenize_num(buf, len);
    } else {
        return switch (c) {
            ' ' => {
                len.* += skip_whitespace(buf);
                return Token.Wsp;
            },
            '"' => {
                len.* += skip_whitespace(buf);
                return Token.Wsp;
            },
            '\'' => {
                len.* += skip_whitespace(buf);
                return Token.Wsp;
            },
            '(' => {
                len.* += 1;
                return Token.OParen;
            },
            ')' => {
                len.* += 1;
                return Token.CParen;
            },
            '{' => {
                len.* += 1;
                return Token.OBrace;
            },
            '}' => {
                len.* += 1;
                return Token.CBrace;
            },
            '[' => {
                len.* += 1;
                return Token.OArray;
            },
            ']' => {
                len.* += 1;
                return Token.CArray;
            },
            '.' => {
                len.* += 1;
                return Token.Dot;
            },
            ',' => {
                len.* += 1;
                return Token.Comma;
            },
            '$' => {
                len.* += 1;
                return Token.Dollar;
            },
            '?' => {
                len.* += 1;
                return Token.Question;
            },
            '#' => {
                len.* += 1;
                return Token.Pound;
            },
            ':' => {
                len.* += 1;
                return Token.Colon;
            },
            ';' => {
                len.* += 1;
                return Token.SColon;
            },
            '\\' => {
                len.* += 1;
                return Token.BSlash;
            },
            '`' => {
                len.* += 1;
                return Token.Backtick;
            },
            '_' => {
                len.* += 1;
                return Token.Rest;
            },
            '@' => {
                len.* += 1;
                return Token.At;
            },
            '>' => {
                len.* += skip_whitespace(buf);
                return Token.Gt;
            },
            '|' => {
                return tokenize_two(buf, len, Token.OrLog, '=', Token.OrAs, '|', Token.Or);
            },
            '&' => {
                return tokenize_two(buf, len, Token.AndLog, '&', Token.And, '=', Token.AndAs);
            },
            '<' => {
                len.* += skip_whitespace(buf);
                return Token.Lt;
            },
            '+' => {
                return tokenize_two(buf, len, Token.Plus, '+', Token.Inc, '=', Token.AddAs);
            },
            '-' => {
                return tokenize_two(buf, len, Token.Sub, '-', Token.Dec, '=', Token.SubAs);
            },
            '/' => {
                return tokenize_one(buf, len, Token.Div, '=', Token.DivAs);
            },
            '*' => {
                return tokenize_one(buf, len, Token.Mul, '=', Token.MulAs);
            },
            '^' => {
                return tokenize_one(buf, len, Token.Xor, '=', Token.XorAs);
            },
            '!' => {
                return tokenize_one(buf, len, Token.Not, '=', Token.NotEquality);
            },
            '%' => {
                return tokenize_one(buf, len, Token.Mod, '=', Token.ModAs);
            },
            '~' => {
                return tokenize_one(buf, len, Token.NotLog, '=', Token.NotAs);
            },
            '=' => {
                return tokenize_one(buf, len, Token.As, '=', Token.Equality);
            },
            '\r' => {
                if (buf.len > 2) {
                    if (buf[1] == '\n') {
                        len.* = 2;
                        return Token.NewLine;
                    }
                }
                len.* = 1;
                return TokenError.InvalidToken;
            },
            '\n' => {
                len.* += 1;
                return Token.NewLine;
            },
            else => {
                return TokenError.InvalidToken;
            },
        };
    }
}

inline fn tokenize_one(
    buf: []const u8,
    len: *usize,
    def_tok: Token,
    comp: u8,
    comp_tok: Token,
) Token {
    if (buf.len > 2) {
        if (buf[1] == comp) {
            len.* = 2;
            return comp_tok;
        }
    }
    len.* = 1;
    return def_tok;
}

inline fn tokenize_two(
    buf: []const u8,
    len: *usize,
    def_tok: Token,
    comp1: u8,
    comp1_tok: Token,
    comp2: u8,
    comp2_tok: Token,
) Token {
    if (buf.len > 2) {
        if (buf[1] == comp1) {
            len.* = 2;
            return comp1_tok;
        } else if (buf[1] == comp2) {
            len.* = 2;
            return comp2_tok;
        }
    }
    len.* = 1;
    return def_tok;
}

inline fn word_len_check(buf: []const u8) usize {
    var len: usize = 1;
    while (buf.len != len) {
        const c = buf[len];
        if (ascii.isAlphanumeric(c)) {
            len += 1;
        } else {
            switch (c) {
                '_', '-' => {
                    len += 1;
                },
                else => {
                    break;
                },
            }
        }
    }
    return len;
}

inline fn collect_digits(buf: []const u8, len: *usize) void {
    var curr: usize = 0;
    while (buf.len > curr) {
        const c = buf[curr];
        if (ascii.isDigit(c)) {
            curr += 1;
        } else {
            break;
        }
    }
    len.* += curr;
}

inline fn tokenize_num(buf: []const u8, len: *usize) Token {
    var token = Token.Num;
    collect_digits(buf, len);
    return token;
}

inline fn tokenize_chars(buf: []const u8, len: *usize) Token {
    var token = Token.Symbol;
    len.* = word_len_check(buf);
    var check = buf[0..len.*];
    for (keywords, 0..) |word, idx| {
        if (word.len == len.*) {
            if (std.mem.eql(u8, word, check)) {
                token = @enumFromInt(Token, idx);
                return token;
            }
        }
    }
    return token;
}

inline fn skip_whitespace(buf: []const u8) usize {
    var len: usize = 1;
    while (buf.len != len) {
        const c = buf[len];
        if (c == ' ') {
            len += 1;
        } else {
            break;
        }
    }
    return len;
}

const keywords = [_][]const u8{
    "import",
    "use",
    "define",
    "macro",
    "test",
    "bench",
    "mut",
    "let",
    "const",
    "once",
    "local",
    "num",
    "i32",
    "u32",
    "u64",
    "i64",
    "i16",
    "u16",
    "u8",
    "i8",
    "bit",
    "f64",
    "f32",
    "d32",
    "d64",
    "if",
    "else",
    "type",
    "this",
    "null",
    "char",
    "string",
    "inline",
    "static",
    "switch",
    "for",
    "in",
    "break",
    "enum",
    "pub",
    "return",
    "async",
    "await",
    "box",
    "trait",
    "ptr",
    "match",
    "addr",
    "vol",
    "true",
    "false",
    "void",
    "iface",
    "generic",
    "undef",
    "never",
    "bool",
    "byte",
    "fn",
    "contract",
    "queue",
    "thread",
    "pool",
    "observe",
    "message",
    "block",
    "suspend",
    "resume",
    "export",
    "notify",
    "not",
    "and",
    "or",
    "error",
    "anyerror",
    "extends",
    "impl",
    "copy",
    "typeof",
    "self",
    "frame",
    "any",
};

test "word len check regular" {
    const buf = "hello";
    const len = word_len_check(buf);

    try testing.expect(len == 5);
}

test "word len check one" {
    const buf = "x ";
    const len = word_len_check(buf);

    try testing.expect(len == 1);
}

test "word len check _" {
    const buf = "hello_there";
    const len = word_len_check(buf);

    try testing.expect(len == 11);
}

test "single" {
    const buf = "x";
    var len: usize = 0;
    const tok = try get_next(buf, &len);

    try testing.expect(tok == Token.Symbol);
    try testing.expect(len == 1);
}

test "word len check -" {
    const buf = "hello-there ";
    const len = word_len_check(buf);

    try testing.expect(len == 11);
}

test "skip whitespace" {
    const buf = "     hello";
    var len: usize = undefined;
    const tok = try get_next(buf, &len);

    try testing.expect(len == 5);
    try testing.expect(tok == Token.Wsp);
}

test "basic numbers" {
    var buf: []const u8 = "55";
    var len: usize = 0;
    var tok = tokenize_num(buf, &len);

    try testing.expect(len == 2);
    try testing.expect(tok == Token.Num);
}

test "keywords tokens" {
    var buf: []const u8 = "macro";
    var len: usize = 0;
    var tok = tokenize_chars(buf, &len);

    try testing.expect(len == 5);
    try testing.expect(tok == Token.K_Macro);

    buf = "const";
    len = 0;
    tok = tokenize_chars(buf, &len);

    try testing.expect(len == 5);
    try testing.expect(tok == Token.K_Const);

    buf = "local";
    len = 0;
    tok = tokenize_chars(buf, &len);

    try testing.expect(len == 5);
    try testing.expect(tok == Token.K_Local);

    buf = "true";
    len = 0;
    tok = tokenize_chars(buf, &len);

    try testing.expect(len == 4);
    try testing.expect(tok == Token.K_True);

    buf = "string";
    len = 0;
    tok = tokenize_chars(buf, &len);

    try testing.expect(len == 6);
    try testing.expect(tok == Token.K_String);

    buf = "pub";
    len = 0;
    tok = tokenize_chars(buf, &len);

    try testing.expect(len == 3);
    try testing.expect(tok == Token.K_Pub);

    buf = "resume";
    len = 0;
    tok = tokenize_chars(buf, &len);

    try testing.expect(len == 6);
    try testing.expect(tok == Token.K_Resume);

    buf = "export";
    len = 0;
    tok = tokenize_chars(buf, &len);

    try testing.expect(len == 6);
    try testing.expect(tok == Token.K_Export);

    buf = "u64";
    len = 0;
    tok = tokenize_chars(buf, &len);

    try testing.expect(len == 3);
    try testing.expect(tok == Token.K_U64);
}

test "get next singular" {
    var buf: []const u8 = "(){}[].,$?#:;_\\`@";
    var len: usize = 0;
    var tok = try get_next(buf, &len);

    try testing.expect(len == 1);
    try testing.expect(tok == Token.OParen);

    len = 0;
    tok = try get_next(buf[1..], &len);

    try testing.expect(len == 1);
    try testing.expect(tok == Token.CParen);

    len = 0;
    tok = try get_next(buf[2..], &len);

    try testing.expect(len == 1);
    try testing.expect(tok == Token.OBrace);

    len = 0;
    tok = try get_next(buf[3..], &len);

    try testing.expect(len == 1);
    try testing.expect(tok == Token.CBrace);

    len = 0;
    tok = try get_next(buf[4..], &len);

    try testing.expect(len == 1);
    try testing.expect(tok == Token.OArray);

    len = 0;
    tok = try get_next(buf[5..], &len);

    try testing.expect(len == 1);
    try testing.expect(tok == Token.CArray);

    len = 0;
    tok = try get_next(buf[6..], &len);

    try testing.expect(len == 1);
    try testing.expect(tok == Token.Dot);

    len = 0;
    tok = try get_next(buf[7..], &len);

    try testing.expect(len == 1);
    try testing.expect(tok == Token.Comma);

    len = 0;
    tok = try get_next(buf[8..], &len);

    try testing.expect(len == 1);
    try testing.expect(tok == Token.Dollar);

    len = 0;
    tok = try get_next(buf[9..], &len);

    try testing.expect(len == 1);
    try testing.expect(tok == Token.Question);

    len = 0;
    tok = try get_next(buf[10..], &len);

    try testing.expect(len == 1);
    try testing.expect(tok == Token.Pound);

    len = 0;
    tok = try get_next(buf[11..], &len);

    try testing.expect(len == 1);
    try testing.expect(tok == Token.Colon);

    len = 0;
    tok = try get_next(buf[12..], &len);

    try testing.expect(len == 1);
    try testing.expect(tok == Token.SColon);

    len = 0;
    tok = try get_next(buf[13..], &len);

    try testing.expect(len == 1);
    try testing.expect(tok == Token.Rest);

    len = 0;
    tok = try get_next(buf[14..], &len);

    try testing.expect(len == 1);
    try testing.expect(tok == Token.BSlash);

    len = 0;
    tok = try get_next(buf[15..], &len);

    try testing.expect(len == 1);
    try testing.expect(tok == Token.Backtick);

    len = 0;
    tok = try get_next(buf[16..], &len);

    try testing.expect(len == 1);
    try testing.expect(tok == Token.At);
}
