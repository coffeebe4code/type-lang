const std = @import("std");

pub const UserFuncName = opaque {
    pub const user = CL_UserFuncName_user;
    extern fn CL_UserFuncName_user(u32, u32) *UserFuncName;

    pub const dispose = CL_UserFuncName_dispose;
    extern fn CL_UserFuncName_dispose(*UserFuncName) void;
};
pub const FunctionParameters = opaque {};
pub const Block = opaque {};

pub const Variable = opaque {
    pub const new = CL_Variable_from_u32;
    extern fn CL_Variable_from_u32(u32) *Variable;

    pub const dispose = CL_Variable_dispose;
    extern fn CL_Variable_dispose(*Variable) void;
};
pub const AbiParam = opaque {
    pub const new_i32 = CL_AbiParam_i32;
    extern fn CL_AbiParam_i32() *AbiParam;

    pub const dispose = CL_AbiParam_dispose;
    extern fn CL_AbiParam_dispose(*AbiParam) void;
};

pub const Signature = opaque {
    pub const new_systemv = CL_Signature_systemv;
    extern fn CL_Signature_systemv() *Signature;

    pub const dispose = CL_Signature_dispose;
    extern fn CL_Signature_dispose(*Signature) void;
};
pub const Function = opaque {
    pub const named = CL_Function_with_name_signature;
    extern fn CL_Function_with_name_signature(*UserFuncName, *Signature) *Function;

    pub const anonymous = CL_Function_new;
    extern fn CL_Function_new() *Function;

    pub const dispose = CL_Function_dispose;
    extern fn CL_Function_dispose(*Function) void;
};

pub const FunctionBuilder = opaque {
    pub const new = CL_FunctionBuilder_new;
    extern fn CL_FunctionBuilder_new(*Function, *Context) *FunctionBuilder;

    pub const decl_var = CL_FunctionBuilder_declare_var;
    extern fn CL_FunctionBuilder_declare_var(*FunctionBuilder, *Variable) void;

    pub const create_block = CL_FunctionBuilder_create_block;
    extern fn CL_FunctionBuilder_create_block(*FunctionBuilder) *Block;

    pub const append_block_params = CL_FunctionBuilder_append_block_params_for_function_params;
    extern fn CL_FunctionBuilder_append_block_params_for_function_params(*FunctionBuilder, *Block) void;

    pub const dispose = CL_FunctionBuilder_dispose;
    extern fn CL_FunctionBuilder_dispose(*FunctionBuilder) void;
};

pub const Context = opaque {
    pub const new = CL_FunctionBuilderContext_new;
    extern fn CL_FunctionBuilderContext_new() *Context;

    pub const dispose = CL_FunctionBuilderContext_dispose;
    extern fn CL_FunctionBuilderContext_dispose(*Context) void;
};

test "should create and dispose simple block function and context" {
    const context = Context.new();
    defer Context.dispose(context);

    const sig = Signature.new_systemv();
    defer Signature.dispose(sig);

    const user = UserFuncName.user(0, 0);
    defer UserFuncName.dispose(user);

    const abi_param = AbiParam.new_i32();
    defer AbiParam.dispose(abi_param);

    const abi_param2 = AbiParam.new_i32();
    defer AbiParam.dispose(abi_param2);

    const func = Function.named(user, sig);
    defer Function.dispose(func);

    const builder = FunctionBuilder.new(func, context);
    defer FunctionBuilder.dispose(builder);

    const variable1 = Variable.new(0);
    defer Variable.dispose(variable1);

    const variable2 = Variable.new(1);
    defer Variable.dispose(variable2);

    const variable3 = Variable.new(2);
    defer Variable.dispose(variable3);

    const block = FunctionBuilder.create_block(builder);
    FunctionBuilder.append_block_params(builder, block);
}
