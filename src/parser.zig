const Token = @import("./token.zig").Token;
const TokenError = @import("./token.zig").TokenError;
const Lexer = @import("./lexer.zig").Lexer;
const Ast = @import("./ast.zig").Ast;
const ast = @import("./ast.zig");
const Span = @import("./span.zig").Span;
const AstTag = @import("./ast.zig").AstTag;
const Allocator = @import("std").mem.Allocator;
const std = @import("std");
const ArrayList = @import("std").ArrayList;
const testing = @import("std").testing;

const ParserError = error{
    InvalidExpectedToken,
    ExpectedOneOfFound,
    ExpectedTokenFoundNone,
};

const Parser = struct {
    lexer: Lexer,
    asts: ArrayList(Ast),
    extras: ArrayList(ArrayList(usize)),
    allocator: Allocator,

    pub fn init(lexer: Lexer, allocator: Allocator) anyerror!Parser {
        const asts: ArrayList(Ast) = std.ArrayList(Ast).init(allocator);
        const extras = std.ArrayList(ArrayList(usize)).init(allocator);
        return Parser{
            .lexer = lexer,
            .asts = asts,
            .allocator = allocator,
            .extras = extras,
        };
    }

    pub fn deinit(self: *Parser) void {
        self.asts.deinit();
        for (self.extras.items) |e| {
            e.deinit();
        }
        self.extras.deinit();
    }

    pub fn ty(self: *Parser) anyerror!usize {
        var span = try self.lexer.collect_if_of(&[_]Token{ Token.K_Num, Token.K_Any, Token.K_U64 });
        if (span) |cap| {
            const local = try ast.make_type(cap);
            return try self.append_ast(local);
        }
        span = try self.lexer.collect_if_of(&[_]Token{ Token.OBrace, Token.OArray });
        if (span) |cap| {
            const next = try self.lexer.collect_if_of(&[_]Token{ Token.CBrace, Token.CArray });
            if (next == null) {
                if ((cap.token == Token.OBrace and next.?.token == Token.CBrace) or (cap.token == Token.OArray and next.?.token == Token.CArray)) {
                    const local = try ast.make_type(cap);
                    return try self.append_ast(local);
                }
            }
            if (cap.token == Token.OBrace) {
                return ParserError.ExpectedOneOfFound;
            } else {
                return ParserError.ExpectedOneOfFound;
            }
        }
        const m_ident = try self.ident();
        if (m_ident) |cap| {
            const local = ast.make_type_ident(cap);
            return try self.append_ast(local);
        }
        return try self.fn_type();
    }

    pub fn args(self: *Parser) anyerror!usize {
        var exprs: ArrayList(usize) = std.ArrayList(usize).init(self.allocator);
        errdefer {
            exprs.deinit();
        }
        while (true) {
            const expr = try self.arg();
            const comma = try self.lexer.collect_if(Token.Comma);
            try exprs.append(expr);
            if (comma == null) {
                break;
            }
        }
        const local = ast.make_args(&exprs.items);
        try self.extras.append(exprs);
        return try self.append_ast(local);
    }

    pub fn arg(self: *Parser) anyerror!usize {
        const m_self = try self.self_arg();
        if (m_self) |cap| {
            return cap;
        }
        const id = try expect_ast(try self.ident());
        const colon = try self.lexer.collect_if(Token.Colon);
        if (colon != null) {
            const mutability = try self.lexer.collect_if_of(&[_]Token{ Token.Mul, Token.AndLog, Token.K_Let, Token.K_Const });
            const typ = try self.ty();
            const local = ast.make_arg(id, mutability, typ);
            return try self.append_ast(local);
        }
        const local = ast.make_arg(id, null, null);
        return try self.append_ast(local);
    }

    pub fn self_arg(self: *Parser) anyerror!?usize {
        var s = try self.lexer.collect_if(Token.K_Self);
        if (s != null) {
            const semi = try self.lexer.collect_if(Token.SColon);
            if (semi != null) {
                const mutability = try self.lexer.collect_if_of(&[_]Token{ Token.Mul, Token.AndLog, Token.K_Let, Token.K_Const });

                const typ = try self.ty();

                const local = ast.make_selfarg(mutability, typ);
                return try self.append_ast(local);
            }
            const local = ast.make_selfarg(null, null);
            return try self.append_ast(local);
        }
        return null;
    }

    pub fn func(self: *Parser) anyerror!usize {
        const has_pub = try self.lexer.collect_if(Token.K_Pub);
        const mutability = try expect_span(try self.lexer.collect_if_of(&[_]Token{
            Token.K_Const,
            Token.K_Let,
        }));

        const identifier = try expect_ast(try self.ident());
        const eq = try expect_span(try self.lexer.collect_if(Token.As));
        _ = eq;
        const func_span = try expect_span(try self.lexer.collect_if(Token.K_Fn));
        _ = func_span;
        const oparen = try expect_span(try self.lexer.collect_if(Token.OParen));
        var cparen = try self.lexer.collect_if(Token.CParen);
        _ = oparen;
        var get_args: ?usize = null;
        if (cparen == null) {
            get_args = try self.args();
            _ = try expect_span(try self.lexer.collect_if(Token.CParen));
        }

        const ret_t = try self.ret_type();
        const blk = try self.block();
        const expr = ast.make_func(
            identifier,
            has_pub != null,
            mutability.token == Token.K_Let,
            get_args,
            ret_t,
            blk,
        );
        return try self.append_ast(expr);
    }

    pub fn fn_type(self: *Parser) anyerror!usize {
        var types: ArrayList(usize) = std.ArrayList(usize).init(self.allocator);
        errdefer {
            types.deinit();
        }
        const func_span = try expect_span(try self.lexer.collect_if(Token.K_Fn));
        _ = func_span;
        const oparen = try expect_span(try self.lexer.collect_if(Token.OParen));
        _ = oparen;
        const paren = try self.lexer.collect_if(Token.CParen);
        if (paren == null) {
            while (true) {
                const span = try self.ty();
                try types.append(span);
                const cparen = try self.lexer.collect_if(Token.CParen);
                if (cparen != null) {
                    break;
                }
                const comma = try self.lexer.collect_if(Token.Comma);
                if (comma != null) {
                    return ParserError.ExpectedOneOfFound;
                }
            }
        }
        const ret_t = try self.ret_type();
        const expr = ast.make_fn_type(&types.items, ret_t);
        try self.extras.append(types);
        return try self.append_ast(expr);
    }

    pub fn ret_type(self: *Parser) anyerror!usize {
        var m_void = try self.lexer.collect_if(Token.K_Void);
        if (m_void) |v| {
            const expr = try ast.make_type(v);
            return try self.append_ast(expr);
        }
        return try self.ty();
    }

    pub fn ret(self: *Parser) anyerror!usize {
        const span = try self.lexer.collect_if(Token.K_Return);
        if (span == null) {
            const expr = ast.make_retvoid(span);
            return try self.append_ast(expr);
        }
        const has_semi = try self.lexer.collect_if(Token.SColon);
        var get: usize = undefined;
        if (has_semi == null) {
            get = try self.or_cmp();
            const semi = try expect_span(try self.lexer.collect_if(Token.SColon));
            _ = semi;
            const expr = ast.make_ret(span.?, get);
            return try self.append_ast(expr);
        }
        const expr = ast.make_retvoid(span);
        return try self.append_ast(expr);
    }

    pub fn block(self: *Parser) anyerror!usize {
        var exprs: ArrayList(usize) = std.ArrayList(usize).init(self.allocator);
        errdefer {
            exprs.deinit();
        }
        var obrace = try expect_span(try self.lexer.collect_if(Token.OBrace));
        _ = obrace;
        while (true) {
            const span = try self.ret();
            try exprs.append(span);
            const cbrace = try self.lexer.collect_if(Token.CBrace);
            if (cbrace != null) {
                break;
            }
        }
        const expr = ast.make_block(&exprs.items);
        try self.extras.append(exprs);
        return try self.append_ast(expr);
    }

    pub fn or_cmp(self: *Parser) anyerror!usize {
        var left = try self.and_cmp();
        while (try self.lexer.collect_if(Token.Or)) |bin| {
            const right = try self.and_cmp();
            const expr = try ast.make_binop(left, bin, right);
            try self.asts.append(expr);
            left = self.last_idx();
        }
        return self.last_idx();
    }

    pub fn and_cmp(self: *Parser) anyerror!usize {
        var left = try self.equality();
        while (try self.lexer.collect_if(Token.And)) |bin| {
            const right = try self.equality();
            const expr = try ast.make_binop(left, bin, right);
            try self.asts.append(expr);
            left = self.last_idx();
        }
        return self.last_idx();
    }

    pub fn equality(self: *Parser) anyerror!usize {
        var left = try self.cmp();
        while (try self.lexer.collect_if_of(&[_]Token{ Token.Equality, Token.NotEquality })) |bin| {
            const right = try self.cmp();
            const expr = try ast.make_binop(left, bin, right);
            try self.asts.append(expr);
            left = self.last_idx();
        }
        return self.last_idx();
    }

    pub fn cmp(self: *Parser) anyerror!usize {
        var left = try self.low_bin();
        while (try self.lexer.collect_if_of(&[_]Token{ Token.Gt, Token.GtEq, Token.Lt, Token.LtEq })) |bin| {
            const right = try self.low_bin();
            const expr = try ast.make_binop(left, bin, right);
            try self.asts.append(expr);
            left = self.last_idx();
        }
        return self.last_idx();
    }

    pub fn low_bin(self: *Parser) anyerror!usize {
        var left = try self.high_bin();
        while (try self.lexer.collect_if_of(&[_]Token{ Token.Plus, Token.Sub })) |bin| {
            const right = try self.high_bin();
            const expr = try ast.make_binop(left, bin, right);
            try self.asts.append(expr);
            left = self.last_idx();
        }
        return self.last_idx();
    }

    pub fn high_bin(self: *Parser) anyerror!usize {
        var left = try self.unary();
        while (try self.lexer.collect_if_of(&[_]Token{ Token.Mul, Token.Div, Token.Mod })) |bin| {
            const right = try self.unary();
            const expr = try ast.make_binop(left, bin, right);
            try self.asts.append(expr);
            left = self.last_idx();
        }
        return self.last_idx();
    }

    pub fn unary(self: *Parser) anyerror!usize {
        const span = try self.lexer.collect_if_of(&[_]Token{ Token.Not, Token.Sub, Token.Mul, Token.AndLog });
        if (span) |capture| {
            const rhs = try self.unary();
            const local = try ast.make_unop(rhs, capture);
            return try self.append_ast(local);
        }
        return expect_ast(try self.terminal());
    }

    pub fn terminal(self: *Parser) anyerror!?usize {
        const span = try self.lexer.collect_if_of(&[_]Token{
            Token.K_True,
            Token.K_False,
            Token.K_Undef,
            Token.K_Self,
            Token.K_Never,
        });
        if (span) |capture| {
            const local = try ast.make_terminal(capture);
            return try self.append_ast(local);
        }
        const m_num = try self.num();
        if (m_num) |cap| {
            return cap;
        }
        return try self.ident();
    }

    pub fn ident(self: *Parser) anyerror!?usize {
        const span = try self.lexer.collect_if(Token.Symbol);
        if (span) |capture| {
            const local = ast.make_ident(capture);
            return try self.append_ast(local);
        }
        return null;
    }

    //pub fn post_unary(self: *Parser) anyerror!usize {
    //    const span = try self.lexer.collect_if_of(&[_]Token{ Token.Question, Token.Not, Token.NotLog });
    //    if (span) |capture| {
    //        const rhs = try expect_ast(try self.unary(), "unary");
    //        const local = ast.make_unop(rhs, capture);
    //        try self.asts.append(local);
    //        return self.last_idx();
    //    }
    //    return try self.num();
    //}

    pub fn num(self: *Parser) anyerror!?usize {
        const span = try self.lexer.collect_if(Token.Num);
        if (span) |capture| {
            const local = ast.make_num(capture);
            return try self.append_ast(local);
        }
        return null;
    }

    fn last_idx(self: Parser) usize {
        return self.asts.items.len - 1;
    }

    fn append_ast(self: *Parser, new_ast: Ast) anyerror!usize {
        try self.asts.append(new_ast);
        return self.last_idx();
    }
};

fn expect_span(expr: ?Span) anyerror!Span {
    return expr orelse {
        return ParserError.ExpectedOneOfFound;
    };
}

fn expect_ast(expr: ?usize) anyerror!usize {
    return expr orelse {
        return ParserError.ExpectedOneOfFound;
    };
}

test "parse low bin" {
    const buf = "2 + 5 * 1";
    const lex = Lexer.new(buf, .{ .skip = true });
    var parser = try Parser.init(lex, std.testing.allocator);
    defer parser.deinit();
    const root_idx = try parser.low_bin();
    const root = parser.asts.items[root_idx];
    const left = parser.asts.items[0];
    const right = parser.asts.items[3];

    //.     +
    //.    / \
    //.   2   *
    //.      / \
    //.     5   1

    try testing.expect(root_idx == 4);
    try testing.expect(std.mem.eql(u8, left.Num.slice, buf[0..1]));
    try testing.expect(root.BinOpAdd.op.token == Token.Plus);
    try testing.expect(right.BinOpMul.op.token == Token.Mul);
}

test "parse single but top down" {
    const buf = "5";
    const lex = Lexer.new(buf, .{ .skip = true });
    var parser = try Parser.init(lex, std.testing.allocator);
    defer parser.deinit();
    const root_idx = try parser.low_bin();
    const root = parser.asts.items[root_idx];

    try testing.expect(root.Num.token == Token.Num);
}

test "parse high bin" {
    const buf = "2 * 5 * 1";
    const lex = Lexer.new(buf, .{ .skip = true });
    var parser = try Parser.init(lex, std.testing.allocator);
    defer parser.deinit();
    const root_idx = try parser.high_bin();
    const root = parser.asts.items[root_idx];
    const left = parser.asts.items[2];
    const right = parser.asts.items[3];

    //.     *
    //.    / \
    //.   *   1
    //.  / \
    //. 2   5

    try testing.expect(root_idx == 4);
    try testing.expect(root.BinOpMul.op.token == Token.Mul);
    try testing.expect(left.BinOpMul.op.token == Token.Mul);
    try testing.expect(std.mem.eql(u8, right.Num.slice, buf[8..9]));
}

test "parse terminal" {
    const buf = "true";
    const lex = Lexer.new(buf, .{ .skip = true });
    var parser = try Parser.init(lex, std.testing.allocator);
    defer parser.deinit();
    const result = try parser.terminal();
    const root = parser.asts.items[result.?];

    try testing.expect(root.True.token == Token.K_True);
}

test "parse ident" {
    const buf = "ident";
    const lex = Lexer.new(buf, .{ .skip = true });
    var parser = try Parser.init(lex, std.testing.allocator);
    defer parser.deinit();
    const result = try parser.terminal();
    const root = parser.asts.items[result.?];

    try testing.expect(std.mem.eql(u8, root.Ident.slice, buf));
}

test "parse block" {
    const buf = "{ return 5 || 5; }";
    const lex = Lexer.new(buf, .{ .skip = true });
    var parser = try Parser.init(lex, std.testing.allocator);
    defer parser.deinit();
    const result = try parser.block();
    const root = parser.asts.items[result];
    const compare = parser.asts.items[2];
    const node = parser.asts.items[compare.BinOpOrCmp.left];

    try testing.expect(root.Block.exprs.len == 1);
    try testing.expect(std.mem.eql(u8, node.Num.slice, "5"));
}

test "parse num" {
    const buf = "5";
    const lex = Lexer.new(buf, .{ .skip = true });
    var parser = try Parser.init(lex, std.testing.allocator);
    defer parser.deinit();
    const result = try parser.num();
    const root = parser.asts.items[result.?];

    try testing.expect(root.Num.token == Token.Num);
}

test "parse unary" {
    const buf = "!5";
    const lex = Lexer.new(buf, .{});
    var parser = try Parser.init(lex, std.testing.allocator);
    defer parser.deinit();
    const result = try parser.unary();

    const left = parser.asts.items[result];

    try testing.expect(left.UnOpNot.op.token == Token.Not);
}

test "parse function empty" {
    const buf = "pub const main = fn () void {}";
    const lex = Lexer.new(buf, .{ .skip = true });
    var parser = try Parser.init(lex, std.testing.allocator);
    defer parser.deinit();
    const result = try parser.func();
    const root = parser.asts.items[result];

    try testing.expect(root.Function.args == null);
    try testing.expect(root.Function.ret == 1);
}

test "parse function" {
    const buf = "const main = fn (count: u64) u64 { return 0 + 1; }";
    const lex = Lexer.new(buf, .{ .skip = true });
    var parser = try Parser.init(lex, std.testing.allocator);
    defer parser.deinit();
    const result = try parser.func();
    const root = parser.asts.items[result];

    try testing.expect(root.Function.ret == 5);
    try testing.expect(root.Function.name == 0);
    try testing.expect(root.Function.mutable == false);
    try testing.expect(root.Function.args.? == 4);
    try testing.expect(parser.asts.items[9].Ret.expr == 8);
}

test "parse fn type empty" {
    const buf = "fn () void";
    const lex = Lexer.new(buf, .{ .skip = true });
    var parser = try Parser.init(lex, std.testing.allocator);
    defer parser.deinit();
    const result = try parser.fn_type();
    const root = parser.asts.items[result];

    try testing.expect(root.TypeFunction.types.len == 0);
    try testing.expect(root.TypeFunction.ret_type == 0);
}

test "parse fn type" {
    const buf = "fn (any) void";
    const lex = Lexer.new(buf, .{ .skip = true });
    var parser = try Parser.init(lex, std.testing.allocator);
    defer parser.deinit();
    const result = try parser.fn_type();
    const root = parser.asts.items[result];

    try testing.expect(root.TypeFunction.types.len == 1);
    try testing.expect(root.TypeFunction.ret_type == 1);
}
