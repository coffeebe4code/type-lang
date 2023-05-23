pub const FContext = opaque {
    pub const new = cl_function_builder_context_new;
    extern fn cl_function_builder_context_new() *FContext;

    pub const dispose = cl_function_builder_context_dispose;
    extern fn cl_function_builder_context_dispose(*FContext) void;
};

test "should create and dispose context" {
    const context = FContext.new();
    defer FContext.dispose(context);
}
