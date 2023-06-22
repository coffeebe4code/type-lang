const cl = @cImport({
    @cInclude("craneliftc.h");
    @cInclude("craneliftc_extra.h");
});
const std = @import("std");

pub const Fir = struct {
    namespace: u32,
    name: u32,
    cvar: u32,
    ctx: ?*cl.FunctionBuilderContext,
    func_builder: ?*cl.FunctionBuilder = undefined,
    func: ?*cl.Function = undefined,
    sig: ?*cl.Signature = undefined,
    flags: ?*cl.Flags = undefined,

    pub fn init(namespace: u32, name: u32) Fir {
        return Fir{
            .namespace = namespace,
            .name = name,
            .cvar = 0,
            .ctx = cl.CL_FunctionBuilderContext_new(),
        };
    }
    pub inline fn add_flags(self: *Fir) void {
        const builder = cl.CL_Builder_builder();
        self.flags = cl.CL_Flags_new(builder);
    }
    pub inline fn create_sig(self: *Fir, call_conv: cl.CCallConv) void {
        self.sig = cl.CL_Signature_new(call_conv);
    }
    pub inline fn create_named_func(self: *Fir) void {
        self.func = cl.CL_Function_with_name_signature(cl.CL_UserFuncName_user(self.namespace, self.name), self.sig);
    }
    pub inline fn create_abi(self: *Fir, abi: cl.CType) ?*cl.AbiParam {
        _ = self;
        return cl.CL_AbiParam_new(abi);
    }

    pub inline fn switch_to_block(self: Fir, block: u32) void {
        return cl.CL_FunctionBuilder_switch_to_block(self.func_builder, block);
    }

    pub inline fn append_params_to_block(self: Fir, block: u32) void {
        return cl.CL_FunctionBuilder_append_block_params_for_function_params(self.func_builder, block);
    }

    pub inline fn get_block_param(self: Fir, block: u32, idx: u32) cl.CValue {
        return cl.CL_FunctionBuilder_block_params(self.func_builder, block, idx);
    }

    pub inline fn seal_block(self: Fir, block: u32) void {
        return cl.CL_FunctionBuilder_seal_block(self.func_builder, block);
    }
    pub inline fn returns_push(self: Fir, abi: ?*cl.AbiParam) void {
        return cl.CL_Signature_returns_push(self.sig, abi);
    }

    pub inline fn params_push(self: Fir, abi: ?*cl.AbiParam) void {
        return cl.CL_Signature_params_push(self.sig, abi);
    }

    pub inline fn create_builder(self: *Fir) void {
        self.func_builder = cl.CL_FunctionBuilder_new(self.func, self.ctx);
    }

    pub inline fn declare_var(self: Fir, variable: cl.CVariable, ctype: cl.CType) void {
        return cl.CL_FunctionBuilder_declare_var(self.func_builder, variable, ctype);
    }

    pub inline fn use_var(self: Fir, variable: cl.CVariable) cl.CValue {
        return cl.CL_FunctionBuilder_use_var(self.func_builder, variable);
    }

    pub inline fn verify(self: Fir) !void {
        return cl.CL_Function_verify(self.func, self.flags);
    }

    pub inline fn display(self: Fir) [*c]u8 {
        return cl.CL_Function_display(self.func);
    }

    pub inline fn def_var(self: Fir, variable: cl.CVariable, val: cl.CValue) void {
        return cl.CL_FunctionBuilder_def_var(self.func_builder, variable, val);
    }

    pub inline fn create_block(self: *Fir) cl.CBlock {
        return cl.CL_FunctionBuilder_create_block(self.func_builder);
    }

    pub inline fn create_var(self: *Fir) cl.CVariable {
        const temp = cl.CL_Variable_from_u32(self.cvar);
        self.cvar += 1;
        return temp;
    }

    pub inline fn finalize(self: *Fir) void {
        return cl.CL_FunctionBuilder_finalize(self.func_builder);
    }

    pub fn deinit(self: Fir) void {
        cl.CL_FunctionBuilderContext_dispose(self.ctx);
    }
};

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

    cl.CL_FunctionBuilder_finalize(fbuilder);

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

test "should create simple function in Fir" {
    var fir = Fir.init(0, 0);
    defer fir.deinit();

    fir.create_sig(cl.SystemV);

    const abi_return = fir.create_abi(cl.I32);
    const abi_param = fir.create_abi(cl.I32);
    fir.returns_push(abi_return);
    fir.params_push(abi_param);

    fir.create_named_func();
    fir.create_builder();
    const block = fir.create_block();
    const x = fir.create_var();
    const y = fir.create_var();
    const z = fir.create_var();
    fir.declare_var(x, cl.I32);
    fir.declare_var(y, cl.I32);
    fir.declare_var(z, cl.I32);
    fir.append_params_to_block(block);

    fir.switch_to_block(block);
    fir.seal_block(block);

    //// set x to the function input at 0.
    {
        const temp = fir.get_block_param(block, 0);
        fir.def_var(x, temp);
    }

    //// set y to 2
    {
        const temp = cl.CL_FunctionBuilder_iconst(fir.func_builder, cl.I32, 2);
        fir.def_var(y, temp);
    }

    // add them together and return z.
    {
        const x1 = fir.use_var(x);
        const y1 = fir.use_var(y);

        const temp = cl.CL_FunctionBuilder_iadd(fir.func_builder, x1, y1);
        fir.def_var(z, temp);
        const result = fir.use_var(z);
        _ = cl.CL_FunctionBuilder_return_(fir.func_builder, @constCast(&[1]cl.CValue{result}), 1);
    }

    fir.finalize();

    fir.add_flags();
    try fir.verify();

    const output = fir.display();
    defer cl.cstr_free(output);

    const expected: [:0]const u8 = "function u0:0(i32) -> i32 system_v {\n" ++
        "block0(v0: i32):\n" ++
        "    v1 = iconst.i32 2\n" ++
        "    v2 = iadd v0, v1  ; v1 = 2\n" ++
        "    return v2\n" ++
        "}\n";
    try std.testing.expect(std.mem.eql(u8, std.mem.span(output), expected));
}
