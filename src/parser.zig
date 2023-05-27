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

    pub fn init(lexer: Lexer, allocator: Allocator) anyerror!Parser {
        var asts: ArrayList(Ast) = std.ArrayList(Ast).init(allocator);
        return Parser{
            .lexer = lexer,
            .asts = asts,
        };
    }

    pub fn deinit(self: *Parser) void {
        self.asts.deinit();
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
        });
        if (span) |capture| {
            const local = try ast.make_terminal(capture);
            try self.asts.append(local);
            return self.last_idx();
        }
        return try self.num();
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

fn expect_span(expr: ?Span, message: []const u8) anyerror!Span {
    _ = message;
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
