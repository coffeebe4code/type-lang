const std = @import("std");

pub const String = struct {
    allocator: std.mem.Allocator,
    val: []const u8,
};

pub fn sentinel(slice: []const u8) ?String {
    if (slice.len == 0) return;
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    const allocator = gpa.allocator();

    const buf = try allocator.alloc(u8, slice.len + 1);
    errdefer allocator.free(buf);

    std.mem.copy(u8, buf[0..], slice);
    return String{
        .allocator = allocator,
        .val = buf,
    };
}

pub fn concat(slices: []const []const u8) ?[]const u8 {
    if (slices.len == 0) return;
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    const allocator = gpa.allocator();

    const total_len = blk: {
        var sum: usize = 0;
        for (slices) |slice| {
            sum += slice.len;
        }
        break :blk sum;
    };

    const buf = try allocator.alloc(u8, total_len);
    errdefer allocator.free(buf);

    var buf_index: usize = 0;
    for (slices) |slice| {
        std.mem.copy(u8, buf[buf_index..], slice);
        buf_index += slice.len;
    }

    return buf;
}
