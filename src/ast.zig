const Token = @import("./token.zig").Token;
const Span = @import("./span.zig").Span;

pub const DeveloperAstError = error{
    TokenNotInList,
};

pub const AstTag = enum {
    DeclFunction,
    Function,
    Block,
    RetVoid,
    Ret,
    Arg,
    Args,
    SelfArg,
    TypeVoid,
    TypeAny,
    TypeObj,
    TypeScalar,
    TypeArray,
    TypeIdent,
    TypeFunction,
    BinOpAdd,
    BinOpSub,
    BinOpOrCmp,
    BinOpGtCmp,
    BinOpGtEqCmp,
    BinOpLtCmp,
    BinOpLtEqCmp,
    BinOpAndCmp,
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

pub const RetStruct = struct {
    span: Span,
    expr: usize,
};

pub const RetVoidStruct = struct {
    span: ?Span,
};

pub const SelfArgStruct = struct {
    ty: ?usize,
    mutability: ?Span,
};

pub const ArgStruct = struct {
    ident: usize,
    mutability: ?Span,
    ty: ?usize,
};

pub const UnOpStruct = struct {
    left: usize,
    op: Span,
};

pub const BlockStruct = struct {
    exprs: *[]const usize,
};

pub const ArgsStruct = struct {
    exprs: *[]const usize,
};

pub const FunctionStruct = struct {
    public: bool,
    mutable: bool,
    name: usize,
    block: usize,
    args: ?usize,
    ret: usize,
};

pub const DeclStruct = struct {
    val: Span,
};

pub const TypeStruct = struct {
    val: Span,
};

pub const TypeFunctionStruct = struct {
    types: *[]const usize,
    ret_type: usize,
};

pub const TypeIdentStruct = struct {
    val: usize,
};

pub const Ast = union(AstTag) {
    DeclFunction: DeclStruct,
    Function: FunctionStruct,
    Block: BlockStruct,
    SelfArg: SelfArgStruct,
    Arg: ArgStruct,
    Args: ArgsStruct,
    RetVoid: RetVoidStruct,
    Ret: RetStruct,
    TypeVoid: TypeStruct,
    TypeAny: TypeStruct,
    TypeObj: TypeStruct,
    TypeScalar: TypeStruct,
    TypeArray: TypeStruct,
    TypeIdent: TypeIdentStruct,
    TypeFunction: TypeFunctionStruct,
    BinOpAdd: BinOpStruct,
    BinOpOrCmp: BinOpStruct,
    BinOpAndCmp: BinOpStruct,
    BinOpGtCmp: BinOpStruct,
    BinOpGtEqCmp: BinOpStruct,
    BinOpLtCmp: BinOpStruct,
    BinOpLtEqCmp: BinOpStruct,
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
        Token.Or => {
            return Ast{ .BinOpOrCmp = local };
        },
        Token.And => {
            return Ast{ .BinOpAndCmp = local };
        },
        Token.Gt => {
            return Ast{ .BinOpGtCmp = local };
        },
        Token.GtEq => {
            return Ast{ .BinOpGtEqCmp = local };
        },
        Token.Lt => {
            return Ast{ .BinOpLtCmp = local };
        },
        Token.LtEq => {
            return Ast{ .BinOpLtEqCmp = local };
        },
        else => {
            return DeveloperAstError.TokenNotInList;
        },
    }
}

pub fn make_retvoid(span: ?Span) Ast {
    return Ast{
        .RetVoid = RetVoidStruct{
            .span = span,
        },
    };
}

pub fn make_ret(span: Span, expr: usize) Ast {
    return Ast{
        .Ret = RetStruct{
            .span = span,
            .expr = expr,
        },
    };
}

pub fn make_fn_type(types: *[]const usize, ret: usize) Ast {
    return Ast{
        .TypeFunction = TypeFunctionStruct{
            .types = types,
            .ret_type = ret,
        },
    };
}

pub fn make_func(name: usize, public: bool, mutable: bool, args: ?usize, ret: usize, block: usize) Ast {
    return Ast{
        .Function = FunctionStruct{
            .public = public,
            .mutable = mutable,
            .name = name,
            .block = block,
            .args = args,
            .ret = ret,
        },
    };
}

pub fn make_block(exprs: *[]const usize) Ast {
    return Ast{
        .Block = BlockStruct{ .exprs = exprs },
    };
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

pub fn make_type(span: Span) DeveloperAstError!Ast {
    const local = TypeStruct{
        .val = span,
    };
    switch (span.token) {
        Token.K_Num => {
            return Ast{ .TypeScalar = local };
        },
        Token.K_Void => {
            return Ast{ .TypeVoid = local };
        },
        Token.K_Any => {
            return Ast{ .TypeAny = local };
        },
        Token.OBrace => {
            return Ast{ .TypeObj = local };
        },
        Token.K_U64 => {
            return Ast{ .TypeScalar = local };
        },
        Token.OArray => {
            return Ast{ .TypeArray = local };
        },
        else => {
            return DeveloperAstError.TokenNotInList;
        },
    }
}

pub fn make_args(args: *[]const usize) Ast {
    return Ast{ .Args = ArgsStruct{
        .exprs = args,
    } };
}

pub fn make_arg(ident: usize, mutability: ?Span, ty: ?usize) Ast {
    return Ast{ .Arg = ArgStruct{
        .ident = ident,
        .mutability = mutability,
        .ty = ty,
    } };
}

pub fn make_selfarg(mutability: ?Span, ty: ?usize) Ast {
    return Ast{ .SelfArg = SelfArgStruct{
        .mutability = mutability,
        .ty = ty,
    } };
}

pub fn make_type_ident(val: usize) Ast {
    return Ast{ .TypeIdent = TypeIdentStruct{ .val = val } };
}

pub fn make_ident(span: Span) Ast {
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
