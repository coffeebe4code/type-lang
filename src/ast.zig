const Token = @import("./token.zig").Token;
const Span = @import("./span.zig").Span;

pub const DeveloperAstError = error{
    TokenNotInList,
};

pub const AstTag = enum {
    BinOpAdd,
    BinOpSub,
    BinOpMul,
    BinOpMod,
    BinOpDiv,
    UnOpNot,
    UnOpNeg,
    UnOpMutRef,
    UnOpConstRef,
    Ident,
    True,
    False,
    Undefined,
    Self,
    Num,
};

pub const BinOpStruct = struct {
    left: usize,
    op: Span,
    right: usize,
};

pub const UnOpStruct = struct {
    left: usize,
    op: Span,
};

pub const Ast = union(AstTag) {
    BinOpAdd: BinOpStruct,
    BinOpSub: BinOpStruct,
    BinOpMul: BinOpStruct,
    BinOpMod: BinOpStruct,
    BinOpDiv: BinOpStruct,
    UnOpNot: UnOpStruct,
    UnOpNeg: UnOpStruct,
    UnOpMutRef: UnOpStruct,
    UnOpConstRef: UnOpStruct,
    Ident: Span,
    True: Span,
    False: Span,
    Undefined: Span,
    Self: Span,
    Num: Span,
};

pub fn make_binop(left: usize, op: Span, right: usize) DeveloperAstError!Ast {
    const local = BinOpStruct{
        .left = left,
        .op = op,
        .right = right,
    };
    switch (op.token) {
        Token.Mul => {
            return Ast{ .BinOpMul = local };
        },
        Token.Div => {
            return Ast{ .BinOpDiv = local };
        },
        Token.Mod => {
            return Ast{ .BinOpMod = local };
        },
        Token.Plus => {
            return Ast{ .BinOpAdd = local };
        },
        Token.Sub => {
            return Ast{ .BinOpSub = local };
        },
        else => {
            return DeveloperAstError.TokenNotInList;
        },
    }
}

pub fn make_unop(left: usize, op: Span) DeveloperAstError!Ast {
    const local = UnOpStruct{
        .left = left,
        .op = op,
    };
    switch (op.token) {
        Token.Not => {
            return Ast{ .UnOpNot = local };
        },
        Token.Sub => {
            return Ast{ .UnOpNeg = local };
        },
        Token.Mul => {
            return Ast{ .UnOpMutRef = local };
        },
        Token.AndLog => {
            return Ast{ .UnOpConstRef = local };
        },
        else => {
            return DeveloperAstError.TokenNotInList;
        },
    }
}

pub fn make_ident(span: *Span) Ast {
    return Ast{
        .Ident = span,
    };
}

pub fn make_num(span: Span) Ast {
    return Ast{
        .Num = span,
    };
}

pub fn make_terminal(span: Span) DeveloperAstError!Ast {
    switch (span.token) {
        Token.K_Undef => {
            return Ast{ .Undefined = span };
        },
        Token.K_Self => {
            return Ast{ .Self = span };
        },
        Token.K_True => {
            return Ast{ .True = span };
        },
        Token.K_False => {
            return Ast{ .False = span };
        },
        else => {
            return DeveloperAstError.TokenNotInList;
        },
    }
}
