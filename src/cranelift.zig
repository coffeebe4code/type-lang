const cl = @cImport("craneliftc.h");
const std = @import("std");

test "should create module" {
    const context = cl.
    const test_allocator = std.testing.allocator;
    const context = try GlobalContext.init(test_allocator, global);
    const module = try Module.init(test_allocator, context, .{
        .file_name = "first_lib",
    });
    defer module.deinit();
}
