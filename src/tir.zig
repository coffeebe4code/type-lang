const cl = @cImport({
    @cInclude("craneliftc.h");
    @cInclude("craneliftc_extra.h");
});

const std = @import("std");
const Allocator = @import("std").mem.Allocator;
const Ast = @import("ast.zig").Ast;
const AstType = @import("ast.zig").AstType;
const Fir = @import("cranelift.zig").Fir;

pub const Tir = struct {
    fir: Fir = undefined,
    namespace: u32,
    asts: []const Ast,
    result: std.ArrayList(u8),
    allocator: Allocator,

    pub fn init(namespace: u32, asts: []const Ast, allocator: Allocator) Tir {
        return Tir{
            .namespace = namespace,
            .asts = asts,
            .allocator = allocator,
        };
    }

    pub fn recurse(self: *Tir, idx: usize) void {
        _ = idx;
        _ = self;
    }
};
