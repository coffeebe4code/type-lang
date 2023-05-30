const cl = @cImport({
    @cInclude("craneliftc.h");
    @cInclude("craneliftc_extra.h");
});
const std = @import("std");

test "should create module" {
    const context = cl.CL_FunctionBuilderContext_new();
    defer context.FunctionBuilderContext_dispose();
}
