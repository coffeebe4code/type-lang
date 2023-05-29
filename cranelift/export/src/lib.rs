use cranelift::prelude::*;
use cranelift::codegen::ir::*;
use cranelift::codegen::ir::types::*;
use cranelift::codegen::isa::*;

#[repr(u16)]
pub enum CType {
    I8,
    I16,
    I32,
    I64,
    I128,
    F32,
    F64,
    R32,
    R64,
    I8X8,
    I16X4,
    I32X2,
    F32X2,
    I8X16,
    I16X8,
    I32X4,
    I64X2,
    F32X4,
    F64X2,
    F32X8,
    F64X4,
    F32X16,
    F64X8,
}

#[repr(C)]
pub struct CVariable(u32);
#[repr(C)] 
pub struct CBlock(u32);
#[repr(C)] 
pub struct CValue(u32);


#[repr(u8)]
pub enum CCallConv {
    Fast,
    Cold,
    Tail,
    SystemV,
    WindowsFastcall,
    AppleAarch64,
    Probestack,
    WasmtimeSystemV,
    WasmtimeFastcall,
    WasmtimeAppleAarch64,
}
macro_rules! easy_type {
    ($val:ident, $typ:ident, $($variant:ident,)*) => {
        match $val {
            $($typ::$variant => $variant,)*
        }
    };
}
macro_rules! easy_enum {
    ($val:ident, $nl:ident, $nr:ident, $($variant:ident,)*) => {
        match $val {
            $($nl::$variant => $nr::$variant,)*
        }
    };
}

#[allow(non_snake_case)]
fn convert_CType(td: CType) -> Type {
    return easy_type!(td, CType,
        I8,
        I16,
        I32,
        I64,
        I128,
        F32,
        F64,
        R32,
        R64,
        I8X8,
        I16X4,
        I32X2,
        F32X2,
        I8X16,
        I16X8,
        I32X4,
        I64X2,
        F32X4,
        F64X2,
        F32X8,
        F64X4,
        F32X16,
        F64X8,);
}

#[allow(non_snake_case)]
fn convert_CCallConv(ccd: CCallConv) -> CallConv {
    return easy_enum!(ccd, CCallConv, CallConv,
        Fast,
        Cold,
        Tail,
        SystemV,
        WindowsFastcall,
        AppleAarch64,
        Probestack,
        WasmtimeSystemV,
        WasmtimeFastcall,
        WasmtimeAppleAarch64,);
}

macro_rules! namespace_new {
    ($namespace:ident) => {
        paste::paste! {
        #[no_mangle]
        #[allow(non_snake_case)]
        pub extern "C" fn [< CL_ $namespace _ new >]() -> *mut $namespace {
            let mut val = $namespace::new();
            return &mut val;
        }
    }
    };
}

macro_rules! namespace_new_invoke_one {
    ($namespace:ident, $invoke:ident, $one:ident) => {
        paste::paste! {
        #[no_mangle]
        #[allow(non_snake_case)]
        pub extern "C" fn [< CL_ $namespace _ $invoke >](one: $one) -> *mut $namespace {
            let mut val = $namespace::$invoke(one);
            return &mut val;
        }
    }
    };
}

macro_rules! namespace_new_one_convert {
    ($namespace:ident, $one:ident) => {
        paste::paste! {
        #[no_mangle]
        #[allow(non_snake_case)]
        pub extern "C" fn [< CL_ $namespace _ new >](one: $one) -> *mut $namespace {
            let mut val = $namespace::new([< convert_ $one >](one));
            return &mut val;
        }
    }
    };
}
macro_rules! empty_dispose {
    ($namespace:ident) => {
        paste::paste! {
            #[no_mangle]
            #[allow(non_snake_case)]
            pub extern "C" fn [< CL_ $namespace _dispose >](val: *mut $namespace) -> () {
                if (!val.is_null()) {
                    core::mem::drop(val);
                }
            }
        }
    }
}

// FUNCTIONBUILDERCONTEXT
namespace_new!(FunctionBuilderContext);
empty_dispose!(FunctionBuilderContext);

// FUNCTIONBUILDER
empty_dispose!(FunctionBuilder);

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn CL_FunctionBuilder_new<'a>(func: *mut Function, ctx: *mut FunctionBuilderContext) -> *mut FunctionBuilder<'a> {
    let ufunc = unsafe {
        assert!(!func.is_null());
        &mut *func
    };
    let uctx = unsafe {
        assert!(!ctx.is_null());
        &mut *ctx
    };
    return Box::into_raw(Box::new(FunctionBuilder::new(ufunc, uctx)));
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn CL_FunctionBuilder_create_block(builder: *mut FunctionBuilder) -> CBlock {
    let ubuilder = unsafe {
        assert!(!builder.is_null());
        &mut *builder
    };
    return CBlock(ubuilder.create_block().as_u32());
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn CL_FunctionBuilder_declare_var(builder: *mut FunctionBuilder, variable: CVariable, typ: CType) -> () {
    let ubuilder = unsafe {
        assert!(!builder.is_null());
        &mut *builder
    };
    ubuilder.declare_var(Variable::from_u32(variable.0), convert_CType(typ));
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn CL_FunctionBuilder_def_var(builder: *mut FunctionBuilder, variable: CVariable, val: CValue) -> () {
    let ubuilder = unsafe {
        assert!(!builder.is_null());
        &mut *builder
    };
    ubuilder.def_var(Variable::from_u32(variable.0), Value::from_u32(val.0));
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn CL_FunctionBuilder_imul(builder: *mut FunctionBuilder, left: CValue, right: CValue) -> CValue {
    let ubuilder = unsafe {
        assert!(!builder.is_null());
        &mut *builder
    };
    let result = ubuilder.ins().imul(Value::from_u32(left.0), Value::from_u32(right.0));
    CValue(result.as_u32())
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn CL_FunctionBuilder_use_var(builder: *mut FunctionBuilder, variable: CVariable) -> CValue {
    let ubuilder = unsafe {
        assert!(!builder.is_null());
        &mut *builder
    };
    let result = ubuilder.use_var(Variable::from_u32(variable.0));
    CValue(result.as_u32())
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn CL_FunctionBuilder_append_block_params_for_function_params(builder: *mut FunctionBuilder, block: CBlock) -> () {
    let ubuilder = unsafe {
        assert!(!builder.is_null());
        &mut *builder
    };
    ubuilder.append_block_params_for_function_params(Block::from_u32(block.0));
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn CL_FunctionBuilder_finalize(builder: *mut FunctionBuilder) -> () {
    let ubuilder = unsafe {
        assert!(!builder.is_null());
        core::ptr::read(builder)
    };
    ubuilder.finalize();
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn CL_FunctionBuilder_switch_to_block(builder: *mut FunctionBuilder, block: CBlock) -> () {
    let ubuilder = unsafe {
        assert!(!builder.is_null());
        &mut *builder
    };
    ubuilder.switch_to_block(Block::from_u32(block.0));
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn CL_FunctionBuilder_seal_block(builder: *mut FunctionBuilder, block: CBlock) -> () {
    let ubuilder = unsafe {
        assert!(!builder.is_null());
        &mut *builder
    };
    ubuilder.seal_block(Block::from_u32(block.0));
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn CL_FunctionBuilder_block_params(builder: *mut FunctionBuilder, block: CBlock, idx: usize) -> CValue {
    let ubuilder = unsafe {
        assert!(!builder.is_null());
        &mut *builder
    };
    let result = ubuilder.block_params(Block::from_u32(block.0))[idx];
    CValue(result.as_u32())
}
// VARIABLE
empty_dispose!(Variable);
namespace_new_invoke_one!(Variable, from_u32, u32);

// FUNCTION
empty_dispose!(Function);

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn CL_Function_with_name_signature(user: *mut UserFuncName, sig: *mut Signature) -> *mut Function {
    let usig = unsafe {
        assert!(!sig.is_null());
        core::ptr::read(sig)
    };
    let uuser = unsafe {
        assert!(!user.is_null());
        core::ptr::read(user)
    };
    return Box::into_raw(Box::new(Function::with_name_signature(uuser, usig)));
}

// ABIPARAM
namespace_new_one_convert!(AbiParam, CType);
empty_dispose!(AbiParam);

// SIGNATURE
empty_dispose!(Signature);
namespace_new_one_convert!(Signature, CCallConv);

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn CL_Signature_returns_push(sig: *mut Signature, conv: *mut AbiParam) -> () {
    let usig = unsafe {
        assert!(!sig.is_null());
        &mut *sig
    };
    let uconv = unsafe {
        assert!(!conv.is_null());
        *conv
    };
    usig.returns.push(uconv);
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn CL_Signature_params_push(sig: *mut Signature, conv: *mut AbiParam) -> () {
    let usig = unsafe {
        assert!(!sig.is_null());
        &mut *sig
    };
    let uconv = unsafe {
        assert!(!conv.is_null());
        *conv
    };
    usig.params.push(uconv);
    return ;
}