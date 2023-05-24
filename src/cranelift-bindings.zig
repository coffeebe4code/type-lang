const std = @import("std");
pub const UserFuncName = opaque {};
pub const FunctionStencil = opaque {};
pub const FunctionParameters = opaque {};

pub const Signature = opaque {};
pub const Function = opaque {
    pub const named = cl_function_with_name_signature;
    extern fn cl_function_with_name_signature(*UserFuncName, *Signature) *Function;

    pub const anonymous = CL_Function_New;
    extern fn CL_Function_New() *Function;

    pub const dispose = CL_Function_Dispose;
    extern fn CL_Function_Dispose(*Function) void;
};

pub const FunctionBuilder = opaque {
    pub const new = cl_function_builder_new;
    extern fn cl_function_builder_new(*Function, *Context) *FunctionBuilder;

    pub const dispose = CL_FunctionBuilder_Dispose;
    extern fn CL_FunctionBuilder_Dispose(*FunctionBuilder) void;
};

pub const Context = opaque {
    pub const new = cl_function_builder_context_new;
    extern fn cl_function_builder_context_new() *Context;

    pub const dispose = CL_FunctionBuilderContext_Dispose;
    extern fn CL_FunctionBuilderContext_Dispose(*Context) void;
};

test "should create and dispose context" {
    const context = Context.new();
    defer Context.dispose(context);

    const func = Function.anonymous();
    defer Function.dispose(func);

    const func_builder = FunctionBuilder.new(func, context);
    defer FunctionBuilder.dispose(func_builder);
}
