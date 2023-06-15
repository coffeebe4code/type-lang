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
    blocks: ArrayList(ArrayList(usize)),
    types: ArrayList(ArrayList(usize)),
    allocator: Allocator,

    pub fn init(lexer: Lexer, allocator: Allocator) anyerror!Parser {
        const asts: ArrayList(Ast) = std.ArrayList(Ast).init(allocator);
        const blocks = std.ArrayList(ArrayList(usize)).init(allocator);
        const types = std.ArrayList(ArrayList(usize)).init(allocator);
        return Parser{
            .lexer = lexer,
            .asts = asts,
            .allocator = allocator,
            .blocks = blocks,
            .types = types,
        };
    }

    pub fn deinit(self: *Parser) void {
        self.asts.deinit();
        for (self.blocks.items) |b| {
            b.deinit();
        }
        self.blocks.deinit();
        for (self.types.items) |t| {
            t.deinit();
        }
        self.types.deinit();
    }

    pub fn ty(self: *Parser) anyerror!usize {
        var span = try self.lexer.collect_if_of(&[_]Token{ Token.K_Num, Token.K_Any });
        if (span) |cap| {
            const expr = try ast.make_type(cap);
            try self.asts.append(expr);
            return self.last_idx();
        }
        span = try self.lexer.collect_if_of(&[_]Token{ Token.OBrace, Token.OArray });
        if (span) |cap| {
            const next = try self.lexer.collect_if_of(&[_]Token{ Token.CBrace, Token.CArray });
            if (next == null) {
                if ((cap.token == Token.OBrace and next.?.token == Token.CBrace) or (cap.token == Token.OArray and next.?.token == Token.CArray)) {
                    const expr = try ast.make_type(cap);
                    try self.asts.append(expr);
                    return self.last_idx();
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
            const expr = ast.make_type_ident(cap);
            try self.asts.append(expr);
            return self.last_idx();
        }
        return try self.fn_type();
    }

    pub fn args(self: *Parser) anyerror!usize {
        _ = self;
    }

    pub fn func(self: *Parser) anyerror!usize {
        const has_pub = try self.lexer.collect_if(Token.K_Pub);
        var mutability = try self.lexer.collect_if_of(&[_]Token{ Token.K_Const, Token.K_Let });
        if (mutability == null) {
            return ParserError.ExpectedOneOfFound;
        }
        const identifier = try self.ident();
        if (identifier == null) {
            return ParserError.ExpectedOneOfFound;
        }
        const eq = try self.lexer.collect_if(Token.As);
        if (eq == null) {
            return ParserError.ExpectedOneOfFound;
        }
        const func_span = try self.lexer.collect_if(Token.K_Fn);
        if (func_span == null) {
            return ParserError.ExpectedOneOfFound;
        }
        const oparen = try self.lexer.collect_if(Token.OParen);
        if (oparen == null) {
            return ParserError.ExpectedOneOfFound;
        }

        const cparen = try self.lexer.collect_if(Token.CParen);
        if (cparen == null) {
            return ParserError.ExpectedOneOfFound;
        }

        const ret = try self.ret_type();
        const blk = try self.block();
        const expr = ast.make_func(
            identifier.?,
            has_pub != null,
            mutability.?.token == Token.K_Let,
            null,
            ret,
            blk,
        );
        try self.asts.append(expr);
        return self.last_idx();
    }

    pub fn fn_type(self: *Parser) anyerror!usize {
        var types: ArrayList(usize) = std.ArrayList(usize).init(self.allocator);
        errdefer {
            types.deinit();
        }
        const func_span = try self.lexer.collect_if(Token.K_Fn);
        if (func_span == null) {
            return ParserError.ExpectedOneOfFound;
        }
        const oparen = try self.lexer.collect_if(Token.OParen);
        if (oparen == null) {
            return ParserError.ExpectedOneOfFound;
        }
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
        const ret = try self.ret_type();
        const expr = ast.make_fn_type(&types.items, ret);
        try self.asts.append(expr);
        try self.types.append(types);
        return self.last_idx();
    }

    pub fn ret_type(self: *Parser) anyerror!usize {
        var m_void = try self.lexer.collect_if(Token.K_Void);
        if (m_void) |v| {
            const expr = try ast.make_type(v);
            try self.asts.append(expr);
            return self.last_idx();
        }
        return try self.ty();
    }

    pub fn block(self: *Parser) anyerror!usize {
        var exprs: ArrayList(usize) = std.ArrayList(usize).init(self.allocator);
        errdefer {
            exprs.deinit();
        }
        var obrace = try self.lexer.collect_if(Token.OBrace);
        const brace = try self.lexer.collect_if(Token.CBrace);
        if (obrace != null) {
            if (brace == null) {
                while (true) {
                    const span = try self.or_cmp();
                    try exprs.append(span);
                    const cbrace = try self.lexer.collect_if(Token.CBrace);
                    if (cbrace != null) {
                        break;
                    }
                }
            }
        }
        const expr = ast.make_block(&exprs.items);
        try self.asts.append(expr);
        try self.blocks.append(exprs);
        return self.last_idx();
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
            try self.asts.append(local);
            return self.last_idx();
        }
        return try self.terminal() orelse {
            return ParserError.ExpectedTokenFoundNone;
        };
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
            try self.asts.append(local);
            return self.last_idx();
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
            try self.asts.append(local);
            return self.last_idx();
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
            try self.asts.append(local);
            return self.last_idx();
        }
        return null;
    }

    fn last_idx(self: Parser) usize {
        return self.asts.items.len - 1;
    }
};

fn expect_span(expr: ?Span, expected: []const u8, found: []const u8) anyerror!Span {
    return expr orelse {
        std.debug.print("expected one of {s}, found {}", expected, found);
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
    const buf = "{ 5 || 5 }";
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

test "parse function" {
    const buf = "pub const main = fn () void {}";
    const lex = Lexer.new(buf, .{ .skip = true });
    var parser = try Parser.init(lex, std.testing.allocator);
    defer parser.deinit();
    const result = try parser.func();
    const root = parser.asts.items[result];

    try testing.expect(root.Function.args == null);
    try testing.expect(root.Function.ret == 1);
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
