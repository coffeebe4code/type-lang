const cl = @cImport({
    @cInclude("craneliftc.h");
    @cInclude("craneliftc_extra.h");
});
const std = @import("std");

test "should create simple function" {
    const context = cl.CL_FunctionBuilderContext_new();
    defer cl.CL_FunctionBuilderContext_dispose(context);

    const sig = cl.CL_Signature_new(cl.SystemV);
    const abi_return = cl.CL_AbiParam_new(cl.I32);
    const abi_param = cl.CL_AbiParam_new(cl.I32);

    cl.CL_Signature_returns_push(sig, abi_return);
    cl.CL_Signature_params_push(sig, abi_param);

    const func = cl.CL_Function_with_name_signature(cl.CL_UserFuncName_user(0, 0), sig);

    const fbuilder = cl.CL_FunctionBuilder_new(func, context);
    const block = cl.CL_FunctionBuilder_create_block(fbuilder);
    const x = cl.CL_Variable_from_u32(0);
    const y = cl.CL_Variable_from_u32(1);
    const z = cl.CL_Variable_from_u32(2);
    cl.CL_FunctionBuilder_declare_var(fbuilder, x, cl.I32);
    cl.CL_FunctionBuilder_declare_var(fbuilder, y, cl.I32);
    cl.CL_FunctionBuilder_declare_var(fbuilder, z, cl.I32);
    cl.CL_FunctionBuilder_append_block_params_for_function_params(fbuilder, block);

    cl.CL_FunctionBuilder_switch_to_block(fbuilder, block);
    cl.CL_FunctionBuilder_seal_block(fbuilder, block);

    // set x to the function input at 0.
    {
        const temp = cl.CL_FunctionBuilder_block_params(fbuilder, block, 0);
        cl.CL_FunctionBuilder_def_var(fbuilder, x, temp);
    }
    // set y to 2
    {
        const temp = cl.CL_FunctionBuilder_iconst(fbuilder, cl.I32, 2);
        cl.CL_FunctionBuilder_def_var(fbuilder, y, temp);
    }

    // add them together and return z.
    {
        const x1 = cl.CL_FunctionBuilder_use_var(fbuilder, x);
        const y1 = cl.CL_FunctionBuilder_use_var(fbuilder, y);

        const temp = cl.CL_FunctionBuilder_iadd(fbuilder, x1, y1);
        cl.CL_FunctionBuilder_def_var(fbuilder, z, temp);
        const result = cl.CL_FunctionBuilder_use_var(fbuilder, z);
        _ = cl.CL_FunctionBuilder_return_(fbuilder, @constCast(&[1]cl.CValue{result}), 1);
    }

    const builder = cl.CL_Builder_builder();
    var flags = cl.CL_Flags_new(builder);

    cl.CL_Function_verify(func, flags);
    const output = cl.CL_Function_display(func);

    const expected: [:0]const u8 = "function u0:0(i32) -> i32 system_v {\n" ++
        "block0(v0: i32):\n" ++
        "    v1 = iconst.i32 2\n" ++
        "    v2 = iadd v0, v1  ; v1 = 2\n" ++
        "    return v2\n" ++
        "}\n";
    try std.testing.expect(std.mem.eql(u8, std.mem.span(output), expected));

    cl.cstr_free(output);
}
