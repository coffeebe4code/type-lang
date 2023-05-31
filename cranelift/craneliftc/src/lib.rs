use std::ffi::{c_char, CString};

use cranelift::codegen::ir::{types::*, Function, UserFuncName};
use cranelift::codegen::verifier::verify_function;
use cranelift::prelude::isa::CallConv;
use cranelift::prelude::settings::{self, Builder, Flags};
use cranelift::prelude::{AbiParam, Imm64, Value, Variable};
use cranelift::prelude::{Block, FunctionBuilder, FunctionBuilderContext, InstBuilder, Signature};

#[repr(C)]
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
#[repr(transparent)]
pub struct CImm64(i64);

#[repr(transparent)]
pub struct CVariable(u32);
#[repr(transparent)]
pub struct CBlock(u32);
#[repr(transparent)]
pub struct CValue(u32);
#[repr(transparent)]
pub struct CInst(u32);
#[repr(transparent)]
pub struct CFuncRef(u32);

#[repr(C)]
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
    return easy_type!(
        td, CType, I8, I16, I32, I64, I128, F32, F64, R32, R64, I8X8, I16X4, I32X2, F32X2, I8X16,
        I16X8, I32X4, I64X2, F32X4, F64X2, F32X8, F64X4, F32X16, F64X8,
    );
}

#[allow(non_snake_case)]
fn convert_CCallConv(ccd: CCallConv) -> CallConv {
    return easy_enum!(
        ccd,
        CCallConv,
        CallConv,
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
    );
}

macro_rules! namespace_new {
    ($namespace:ident) => {
        paste::paste! {
            #[no_mangle]
            #[allow(non_snake_case)]
            pub extern "C" fn [< CL_ $namespace _ new >]() -> *mut $namespace {
                return Box::into_raw(Box::new($namespace::new()));
            }
        }
    };
}

//macro_rules! namespace_new_invoke_one {
//    ($namespace:ident, $invoke:ident, $one:ident) => {
//        paste::paste! {
//            #[no_mangle]
//            #[allow(non_snake_case)]
//            pub extern "C" fn [< CL_ $namespace _ $invoke >](one: $one) -> *mut $namespace {
//                let mut val = $namespace::$invoke(one);
//                return &mut val;
//            }
//        }
//    };
//}

macro_rules! namespace_new_one_convert {
    ($namespace:ident, $one:ident) => {
        paste::paste! {
            #[no_mangle]
            #[allow(non_snake_case)]
            pub extern "C" fn [< CL_ $namespace _ new >](one: $one) -> *mut $namespace {
                return Box::into_raw(Box::new($namespace::new([< convert_ $one >](one))));
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
                    let to_drop = unsafe { core::ptr::read(val) };
                    core::mem::drop(to_drop);
                }
            }
        }
    };
}

#[no_mangle]
pub extern "C" fn cstr_free(s: *mut c_char) {
    unsafe {
        if s.is_null() {
            return;
        }
        CString::from_raw(s)
    };
}

// FUNCTIONBUILDERCONTEXT
namespace_new!(FunctionBuilderContext);
empty_dispose!(FunctionBuilderContext);

// FUNCTIONBUILDER
empty_dispose!(FunctionBuilder);

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn CL_FunctionBuilder_new<'a>(
    func: *mut Function,
    ctx: *mut FunctionBuilderContext,
) -> *mut FunctionBuilder<'a> {
    assert!(!func.is_null());
    assert!(!ctx.is_null());
    let ufunc = unsafe { &mut *func };
    let uctx = unsafe { &mut *ctx };
    return Box::into_raw(Box::new(FunctionBuilder::new(ufunc, uctx)));
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn CL_FunctionBuilder_create_block(builder: *mut FunctionBuilder) -> CBlock {
    assert!(!builder.is_null());
    let ubuilder = unsafe { &mut *builder };
    return CBlock(ubuilder.create_block().as_u32());
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn CL_FunctionBuilder_declare_var(
    builder: *mut FunctionBuilder,
    variable: CVariable,
    typ: CType,
) -> () {
    assert!(!builder.is_null());
    let ubuilder = unsafe { &mut *builder };
    ubuilder.declare_var(Variable::from_u32(variable.0), convert_CType(typ));
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn CL_FunctionBuilder_def_var(
    builder: *mut FunctionBuilder,
    variable: CVariable,
    val: CValue,
) -> () {
    assert!(!builder.is_null());
    let ubuilder = unsafe { &mut *builder };
    ubuilder.def_var(Variable::from_u32(variable.0), Value::from_u32(val.0));
}

//#[no_mangle]
//#[allow(non_snake_case)]
//pub extern "C" fn CL_FunctionBuilder_imul(
//    builder: *mut FunctionBuilder,
//    left: CValue,
//    right: CValue,
//) -> CValue {
//    assert!(!builder.is_null());
//    let ubuilder = unsafe { &mut *builder };
//    let result = ubuilder
//        .ins()
//        .imul(Value::from_u32(left.0), Value::from_u32(right.0));
//    CValue(result.as_u32())
//}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn CL_FunctionBuilder_use_var(
    builder: *mut FunctionBuilder,
    variable: CVariable,
) -> CValue {
    assert!(!builder.is_null());
    let ubuilder = unsafe { &mut *builder };
    let result = ubuilder.use_var(Variable::from_u32(variable.0));
    CValue(result.as_u32())
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn CL_FunctionBuilder_append_block_params_for_function_params(
    builder: *mut FunctionBuilder,
    block: CBlock,
) -> () {
    assert!(!builder.is_null());
    let ubuilder = unsafe { &mut *builder };
    ubuilder.append_block_params_for_function_params(Block::from_u32(block.0));
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn CL_FunctionBuilder_finalize(builder: *mut FunctionBuilder) -> () {
    assert!(!builder.is_null());
    let ubuilder = unsafe { core::ptr::read(builder) };
    ubuilder.finalize();
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn CL_FunctionBuilder_switch_to_block(
    builder: *mut FunctionBuilder,
    block: CBlock,
) -> () {
    assert!(!builder.is_null());
    let ubuilder = unsafe { &mut *builder };
    ubuilder.switch_to_block(Block::from_u32(block.0));
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn CL_FunctionBuilder_seal_block(
    builder: *mut FunctionBuilder,
    block: CBlock,
) -> () {
    assert!(!builder.is_null());
    let ubuilder = unsafe { &mut *builder };
    ubuilder.seal_block(Block::from_u32(block.0));
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn CL_FunctionBuilder_block_params(
    builder: *mut FunctionBuilder,
    block: CBlock,
    idx: usize,
) -> CValue {
    assert!(!builder.is_null());
    let ubuilder = unsafe { &mut *builder };
    let result = ubuilder.block_params(Block::from_u32(block.0))[idx];
    CValue(result.as_u32())
}
// UserFuncName
empty_dispose!(UserFuncName);

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn CL_UserFuncName_user(one: u32, two: u32) -> *mut UserFuncName {
    return Box::into_raw(Box::new(UserFuncName::user(one, two)));
}
// VARIABLE
#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn CL_Variable_from_u32(val: u32) -> CVariable {
    return CVariable(val);
}

// FUNCTION
empty_dispose!(Function);

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn CL_Function_with_name_signature(
    user: *mut UserFuncName,
    sig: *mut Signature,
) -> *mut Function {
    assert!(!sig.is_null());
    assert!(!user.is_null());
    let usig = unsafe { core::ptr::read(sig) };
    let uuser = unsafe { core::ptr::read(user) };
    return Box::into_raw(Box::new(Function::with_name_signature(uuser, usig)));
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn CL_Function_verify(func: *mut Function, flags: *mut Flags) -> () {
    assert!(!func.is_null());
    assert!(!flags.is_null());
    let ufunc = unsafe { &*func };
    let uflags = unsafe { &*flags };
    return verify_function(ufunc, uflags).unwrap();
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn CL_Function_display(func: *mut Function) -> *mut c_char {
    assert!(!func.is_null());
    let ufunc = unsafe { &*func };
    let display = ufunc.display().to_string();
    return CString::new(display).unwrap().into_raw();
}

// ABIPARAM
namespace_new_one_convert!(AbiParam, CType);
//empty_dispose!(AbiParam);

// SIGNATURE
//empty_dispose!(Signature);
namespace_new_one_convert!(Signature, CCallConv);

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn CL_Signature_returns_push(sig: *mut Signature, conv: *mut AbiParam) -> () {
    assert!(!conv.is_null());
    assert!(!sig.is_null());
    let usig = unsafe { &mut *sig };
    let uconv = unsafe { core::ptr::read(conv) };
    usig.returns.push(uconv);
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn CL_Signature_params_push(sig: *mut Signature, conv: *mut AbiParam) -> () {
    assert!(!sig.is_null());
    assert!(!conv.is_null());
    let usig = unsafe { &mut *sig };
    let uconv = unsafe { *conv };
    usig.params.push(uconv);
}

// FLAGS
empty_dispose!(Flags);

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn CL_Flags_new(builder: *mut Builder) -> *mut Flags {
    assert!(!builder.is_null());
    let ubuilder = unsafe { core::ptr::read(builder) };
    return Box::into_raw(Box::new(Flags::new(ubuilder)));
}

// SETTINGS

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn CL_Builder_builder() -> *mut Builder {
    return Box::into_raw(Box::new(settings::builder()));
}

//
// MACROS FOR INSTRUCTIONS
//

//macro_rules! instr_two_block_svalue_inst {
//    ($invoke:ident) => {
//        paste::paste! {
//            #[no_mangle]
//            #[allow(non_snake_case)]
//            pub extern "C" fn [< CL_FunctionBuilder_ $invoke >](builder: *mut FunctionBuilder, one: CBlock, two: &[CValue]) -> CValue {
//                assert!(!builder.is_null());
//                let ubuilder = unsafe { &mut *builder };
//                let result = ubuilder
//                    .ins()
//                    .$invoke(Value::from_u32(one.0), two.0);
//                CValue(result.as_u32())
//            }
//        }
//    };
//}

macro_rules! instr_two_value_value_value {
    ($invoke:ident) => {
        paste::paste! {
            #[no_mangle]
            #[allow(non_snake_case)]
            pub extern "C" fn [< CL_FunctionBuilder_ $invoke >](builder: *mut FunctionBuilder, one: CValue, two: CValue) -> CValue {
                assert!(!builder.is_null());
                let ubuilder = unsafe { &mut *builder };
                let result = ubuilder
                    .ins()
                    .$invoke(Value::from_u32(one.0), Value::from_u32(two.0));
                CValue(result.as_u32())
            }
        }
    };
}

macro_rules! instr_one_value_value {
    ($invoke:ident) => {
        paste::paste! {
            #[no_mangle]
            #[allow(non_snake_case)]
            pub extern "C" fn [< CL_FunctionBuilder_ $invoke >](builder: *mut FunctionBuilder, one: CValue) -> CValue {
                assert!(!builder.is_null());
                let ubuilder = unsafe { &mut *builder };
                let result = ubuilder
                    .ins()
                    .$invoke(Value::from_u32(one.0));
                CValue(result.as_u32())
            }
        }
    };
}

macro_rules! instr_two_type_imm_value {
    ($invoke:ident) => {
        paste::paste! {
            #[no_mangle]
            #[allow(non_snake_case)]
            pub extern "C" fn [< CL_FunctionBuilder_ $invoke >](builder: *mut FunctionBuilder, one: CType, two: CImm64) -> CValue {
                assert!(!builder.is_null());
                let ubuilder = unsafe { &mut *builder };
                let result = ubuilder
                    .ins()
                    .$invoke(convert_CType(one), Imm64::new(two.0));
                CValue(result.as_u32())
            }
        }
    };
}

macro_rules! instr_one_svalue_inst {
    ($invoke:ident) => {
        paste::paste! {
            #[no_mangle]
            #[allow(non_snake_case)]
            pub extern "C" fn [< CL_FunctionBuilder_ $invoke >](builder: *mut FunctionBuilder, rvals_raw: *mut CValue, len: usize) -> CInst {
                assert!(!builder.is_null());
                assert!(!rvals_raw.is_null());
                let rvals = unsafe { core::slice::from_raw_parts(rvals_raw, len)};
                let ubuilder = unsafe { &mut *builder };
                let converts: Vec<Value> = rvals.into_iter().map(|x| {return Value::from_u32(x.0); }).collect();
                let result = ubuilder
                    .ins()
                    .$invoke(converts.as_slice());
                CInst(result.as_u32())
            }
        }
    };
}
// INSTRUCTIONS

// (svalue) -> inst
instr_one_svalue_inst!(return_);

// (type, imm64) -> value
instr_two_type_imm_value!(iconst);

// (block, svalue) -> inst

// (value, value) -> value

instr_two_value_value_value!(iadd);
instr_two_value_value_value!(isub);
instr_two_value_value_value!(imul);
instr_two_value_value_value!(umulhi);

// (value) -> value
instr_one_value_value!(ineg);
instr_one_value_value!(iabs);
