const cl = @cImport({
    @cInclude("craneliftc.h");
    @cInclude("craneliftc_extra.h");
});
const std = @import("std");
const Ast = @import("./ast.zig").Ast;

const Tir = struct {
    namespace: u32,
    func_count: u32,
    cvar: u32,
    ctx: ?*cl.FunctionBuilderContext,
    alloc: std.mem.Allocator,
    func_builder: ?*cl.FunctionBuilder = undefined,
    func: ?*cl.Function = undefined,
    sig: ?*cl.Signature = undefined,

    pub fn init(namespace: u32, allocator: std.mem.Allocator) Tir {
        return Tir{
            .namespace = namespace,
            .func_count = 0,
            .cvar = 0,
            .ctx = cl.CL_FunctionBuilderContext_new(),
            .alloc = allocator,
        };
    }
    pub inline fn make_named_func(self: *Tir) void {
        self.func = cl.CL_Function_with_name_signature(cl.CL_UserFuncName_user(self.namespace, self.func_count), self.sig);
        self.func_count += 1;
    }
    pub inline fn make_abi(abi: cl.CType) ?*cl.AbiParam {
        return cl.CL_AbiParam_new(abi);
    }

    pub inline fn set_block(self: Tir, block: u32) void {
        return cl.CL_FunctionBuilder_switch_to_block(self.func_builder, block);
    }

    pub inline fn append_params_to_block(self: Tir, block: u32) void {
        return cl.CL_FunctionBuilder_append_block_params_for_function_params(self.func_builder, block);
    }

    pub inline fn get_block_param(self: Tir, block: u32, idx: u32) void {
        return cl.CL_FunctionBuilder_block_params(self.func_builder, block, idx);
    }

    pub inline fn seal_block(self: Tir, block: u32) void {
        return cl.CL_FunctionBuilder_seal_block(self.func_builder, block);
    }
    pub inline fn returns_push(self: Tir, abi: ?*cl.AbiParam) void {
        return cl.CL_Signature_returns_push(self.sig, abi);
    }

    pub inline fn params_push(self: Tir, abi: ?*cl.AbiParam) void {
        return cl.CL_Signature_params_push(self.sig, abi);
    }

    pub inline fn func_builder(self: Tir) void {
        return cl.CL_FunctionBuilder_new(self.func, self.ctx);
    }

    pub inline fn declare_var(
        self: Tir,
    ) void {
        cl.CL_FunctionBuilder_new(self.func, self.ctx);
    }

    pub inline fn create_block(self: *Tir) u32 {
        return cl.CL_FunctionBuilder_create_block(self.function_builder);
    }

    pub inline fn create_var(self: *Tir) u32 {
        const temp = cl.CL_Variable_from_u32(self.cvar);
        self.cvar += 1;
        return temp;
    }
    pub fn recurse(self: *Tir, node: *Ast) u32 {
        _ = self;
        switch (node) {
            .Num => std.fmt.parseFloat(node.Num.slice),
        }
        return 0;
    }

    pub fn deinit(self: Tir) void {
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

test "should create simple function in Tir" {
    const tir = Tir.init(0, std.testing.allocator);
    defer tir.deinit();
}
