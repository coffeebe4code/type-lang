const Token = @import("./token.zig").Token;
const Span = @import("./span.zig").Span;

pub const DeveloperAstError = error{
    TokenNotInList,
};

pub const AstTag = enum {
    Function,
    TypeVoid,
    TypeAny,
    TypeObj,
    TypeScalar,
    TypeArray,
    TypeIdent,
    TypeFunction,
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
    Never,
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

pub const FunctionStruct = struct {
    name: usize,
    args: ?[]usize,
    ret: ?usize,
    body: usize,
};

pub const TypeStruct = struct {
    val: Span,
};

pub const TypeIdentStruct = struct {
    val: usize,
};

pub const Ast = union(AstTag) {
    Function: FunctionStruct,
    TypeVoid: TypeStruct,
    TypeAny: TypeStruct,
    TypeObj: TypeStruct,
    TypeScalar: TypeStruct,
    TypeArray: TypeStruct,
    TypeIdent: TypeIdentStruct,
    TypeFunction: TypeStruct,
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
    Never: Span,
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

pub fn make_unop(val: usize, op: Span) DeveloperAstError!Ast {
    const local = UnOpStruct{
        .left = val,
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

pub fn make_type(span: *Span) Ast {
    switch (span.token) {
        Token.K_Num => {
            return Ast{ .TypeScalar = span };
        },
        Token.K_Void => {
            return Ast{ .TypeVoid = span };
        },
        Token.K_Any => {
            return Ast{ .TypeAny = span };
        },
        Token.OBrace => {
            return Ast{ .TypeObj = span };
        },
        Token.OArray => {
            return Ast{ .TypeArray = span };
        },
        else => {
            return DeveloperAstError.TokenNotInList;
        },
    }
}

pub fn make_type_ident(val: usize) Ast {
    return Ast{ .TypeIdent = val };
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
        Token.K_Never => {
            return Ast{ .Never = span };
        },
        else => {
            return DeveloperAstError.TokenNotInList;
        },
    }
}
