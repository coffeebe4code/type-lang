const cl = @cImport({
    @cInclude("craneliftc.h");
    @cInclude("craneliftc_extra.h");
});
const std = @import("std");

test "should create module" {
    const builder = cl.CL_Builder_builder();
    var flags = cl.CL_Flags_new(builder);
    _ = flags;
}
