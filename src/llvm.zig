const llvm = @import("./llvm-bindings.zig");

test "should create context" {
    const context = llvm.Context.create();
    defer {
        context.dispose();
    }
    unreachable;
}
