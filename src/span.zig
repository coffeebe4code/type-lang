const Token = @import("./token.zig").Token;

pub const Span = struct {
    slice: []const u8,
    token: Token,
};

pub const FullSpan = struct {
    slice: []const u8,
    token: Token,
    pos: usize,
    col: usize,
    row: usize,
};
