const std = @import("std");

pub const UserFuncName = opaque {};
pub const FunctionParameters = opaque {};
pub const Block = opaque {};

pub const Variable = opaque {
    pub const new = CL_Variable_From_U32;
    extern fn CL_Variable_From_U32() *Variable;

    pub const dispose = CL_Variable_Dispose;
    extern fn CL_Variable_Dispose(*Variable) void;
};
pub const AbiParam = opaque {
    pub const new_i32 = CL_AbiParam_New_I32;
    extern fn CL_AbiParam_New_I32() *AbiParam;

    pub const dispose = CL_AbiParam_Dispose;
    extern fn CL_AbiParam_Dispose(*AbiParam) void;
};

pub const Signature = opaque {
    pub const new = CL_Signature_New_SystemV;
    extern fn CL_Signature_New_SystemV() *Signature;

    pub const dispose = CL_Signature_Dispose;
    extern fn CL_Signature_Dispose(*Signature) void;
};
pub const Function = opaque {
    pub const named = cl_function_with_name_signature;
    extern fn cl_function_with_name_signature(*UserFuncName, *Signature) *Function;

    pub const anonymous = CL_Function_New;
    extern fn CL_Function_New() *Function;

    pub const dispose = CL_Function_Dispose;
    extern fn CL_Function_Dispose(*Function) void;
};

pub const FunctionBuilder = opaque {
    pub const new = CL_FunctionBuilder_New;
    extern fn CL_FunctionBuilder_New(*Function, *Context) *FunctionBuilder;

    pub const decl_var = CL_FunctionBuilder_Declare_Var;
    extern fn CL_FunctionBuilder_Declare_Var(*FunctionBuilder, *Variable) void;

    pub const create_block = CL_FunctionBuilder_create_block;
    extern fn CL_FunctionBuilder_create_block(*FunctionBuilder, *Variable) void;

    pub const dispose = CL_FunctionBuilder_Dispose;
    extern fn CL_FunctionBuilder_Dispose(*FunctionBuilder) void;
};

pub const Context = opaque {
    pub const new = CL_FunctionBuilderContext_New;
    extern fn CL_FunctionBuilderContext_New() *Context;

    pub const dispose = CL_FunctionBuilderContext_Dispose;
    extern fn CL_FunctionBuilderContext_Dispose(*Context) void;
};

test "should create and dispose simple block function and context" {
    const context = Context.new();
    defer Context.dispose(context);

    const func = Function.anonymous();
    defer Function.dispose(func);

    const func_builder = FunctionBuilder.new(func, context);
    defer FunctionBuilder.dispose(func_builder);

    const abi_param = AbiParam.new_i32();
    defer AbiParam.dispose(abi_param);

    const abi_param2 = AbiParam.new_i32();
    defer AbiParam.dispose(abi_param2);

    const block = FunctionBuilder.new_block(func_builder);
    _ = block;
}
